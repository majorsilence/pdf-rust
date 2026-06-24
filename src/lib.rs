//! Rust wrapper for the [`pdfnative`](https://github.com/majorsilence/Reporting) shared library.
//!
//! The PDF engine runs in-process via FFI — no subprocess is spawned and
//! no .NET runtime needs to be installed on the host.
//!
//! # Quick start
//!
//! ```no_run
//! use majorsilence_pdf::PdfLibrary;
//!
//! let lib = PdfLibrary::load("/path/to/libpdfnative.so").unwrap();
//!
//! let mut doc = lib.document();
//! doc.set_title("My Report").unwrap();
//!
//! {
//!     let mut canvas = doc.add_page(595.28, 841.89).unwrap();
//!     canvas.draw_text("Hello, PDF!", 72.0, 100.0, None).unwrap();
//!     canvas.close().unwrap();
//! }
//!
//! doc.save("/tmp/output.pdf").unwrap();
//! doc.close();
//! ```

use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_float, c_int, c_void},
    path::{Path, PathBuf},
    sync::Arc,
};

// ── Error type ─────────────────────────────────────────────────────────────────

/// Error returned by this library.
#[derive(Debug)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

impl From<libloading::Error> for Error {
    fn from(e: libloading::Error) -> Self {
        Error(e.to_string())
    }
}

// ── Page size constants (points) ───────────────────────────────────────────────

pub const PAGE_A4:      (f32, f32) = (595.28, 841.89);
pub const PAGE_A3:      (f32, f32) = (841.89, 1190.55);
pub const PAGE_A5:      (f32, f32) = (419.53, 595.28);
pub const PAGE_LETTER:  (f32, f32) = (612.0,  792.0);
pub const PAGE_LEGAL:   (f32, f32) = (612.0,  1008.0);
pub const PAGE_TABLOID: (f32, f32) = (792.0,  1224.0);

// ── Style constants ────────────────────────────────────────────────────────────

pub const ALIGN_LEFT:   c_int = 0;
pub const ALIGN_CENTER: c_int = 1;
pub const ALIGN_RIGHT:  c_int = 2;

pub const DECOR_NONE:          c_int = 0;
pub const DECOR_UNDERLINE:     c_int = 1;
pub const DECOR_STRIKETHROUGH: c_int = 2;
pub const DECOR_OVERLINE:      c_int = 3;

pub const PERM_PRINT:              c_int =    4;
pub const PERM_MODIFY_CONTENT:     c_int =    8;
pub const PERM_COPY_TEXT:          c_int =   16;
pub const PERM_ADD_ANNOTATIONS:    c_int =   32;
pub const PERM_FILL_FORMS:         c_int =  256;
pub const PERM_EXTRACT_TEXT:       c_int =  512;
pub const PERM_ASSEMBLE:           c_int = 1024;
pub const PERM_PRINT_HIGH_QUALITY: c_int = 2048;
pub const PERM_ALL:                c_int =   -1;

// ── Raw FFI symbol table ───────────────────────────────────────────────────────

#[allow(non_snake_case)]
struct Symbols {
    _lib: libloading::Library,

    pdf_init:    unsafe extern "C" fn() -> c_int,

    pdf_doc_create:       unsafe extern "C" fn() -> *mut c_void,
    pdf_doc_set_title:    unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int,
    pdf_doc_set_author:   unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int,
    pdf_doc_set_subject:  unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int,
    pdf_doc_set_creator:  unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int,
    pdf_doc_set_security: unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char, c_int, c_int) -> c_int,
    pdf_doc_add_page:     unsafe extern "C" fn(*mut c_void, c_float, c_float) -> *mut c_void,
    pdf_doc_save_file:    unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int,
    pdf_doc_save_buffer:  unsafe extern "C" fn(*mut c_void, *mut *mut u8, *mut c_int) -> c_int,
    pdf_doc_close:        unsafe extern "C" fn(*mut c_void),

    pdf_canvas_draw_text:    unsafe extern "C" fn(*mut c_void, *const c_char, c_float, c_float, *mut c_void) -> c_int,
    pdf_canvas_draw_textbox: unsafe extern "C" fn(*mut c_void, *const c_char, c_float, c_float, c_float, c_float, *mut c_void, c_float) -> c_int,
    pdf_canvas_draw_line:    unsafe extern "C" fn(*mut c_void, c_float, c_float, c_float, c_float, u8, u8, u8, c_float) -> c_int,
    pdf_canvas_draw_rect:    unsafe extern "C" fn(*mut c_void, c_float, c_float, c_float, c_float, u8, u8, u8, c_int, u8, u8, u8, c_float, c_int) -> c_int,
    pdf_canvas_draw_ellipse: unsafe extern "C" fn(*mut c_void, c_float, c_float, c_float, c_float, u8, u8, u8, c_int, u8, u8, u8, c_float, c_int) -> c_int,
    pdf_canvas_draw_image:   unsafe extern "C" fn(*mut c_void, *const u8, c_int, c_int, c_int, c_int, c_float, c_float, c_float, c_float) -> c_int,
    pdf_canvas_draw_table:   unsafe extern "C" fn(*mut c_void, *mut c_void, c_float, c_float) -> c_int,
    pdf_canvas_add_link:     unsafe extern "C" fn(*mut c_void, c_float, c_float, c_float, c_float, *const c_char) -> c_int,
    pdf_canvas_close:        unsafe extern "C" fn(*mut c_void),

    pdf_style_create:          unsafe extern "C" fn() -> *mut c_void,
    pdf_style_set_font_family: unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int,
    pdf_style_set_font_file:   unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int,
    pdf_style_set_size:        unsafe extern "C" fn(*mut c_void, c_float) -> c_int,
    pdf_style_set_color:       unsafe extern "C" fn(*mut c_void, u8, u8, u8) -> c_int,
    pdf_style_set_bold:        unsafe extern "C" fn(*mut c_void, c_int) -> c_int,
    pdf_style_set_italic:      unsafe extern "C" fn(*mut c_void, c_int) -> c_int,
    pdf_style_set_alignment:   unsafe extern "C" fn(*mut c_void, c_int) -> c_int,
    pdf_style_set_decoration:  unsafe extern "C" fn(*mut c_void, c_int) -> c_int,
    pdf_style_close:           unsafe extern "C" fn(*mut c_void),

    pdf_table_create:          unsafe extern "C" fn(*const c_float, c_int) -> *mut c_void,
    pdf_table_set_header_bg:   unsafe extern "C" fn(*mut c_void, u8, u8, u8) -> c_int,
    pdf_table_set_alternate_bg:unsafe extern "C" fn(*mut c_void, u8, u8, u8) -> c_int,
    pdf_table_set_border:      unsafe extern "C" fn(*mut c_void, u8, u8, u8, c_float) -> c_int,
    pdf_table_set_cell_padding:unsafe extern "C" fn(*mut c_void, c_float) -> c_int,
    pdf_table_stage_cell:      unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int,
    pdf_table_commit_row:      unsafe extern "C" fn(*mut c_void) -> c_int,
    pdf_table_close:           unsafe extern "C" fn(*mut c_void),

    pdf_merge_create:    unsafe extern "C" fn() -> *mut c_void,
    pdf_merge_add_bytes: unsafe extern "C" fn(*mut c_void, *const u8, c_int) -> c_int,
    pdf_merge_save_file: unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int,
    pdf_merge_save_buffer: unsafe extern "C" fn(*mut c_void, *mut *mut u8, *mut c_int) -> c_int,
    pdf_merge_close:     unsafe extern "C" fn(*mut c_void),

    pdf_free:       unsafe extern "C" fn(*mut c_void),
    pdf_last_error: unsafe extern "C" fn() -> *const c_char,
}

unsafe impl Send for Symbols {}
unsafe impl Sync for Symbols {}

impl Symbols {
    unsafe fn load(path: &Path) -> Result<Self, Error> {
        let lib = libloading::Library::new(path)?;

        macro_rules! sym {
            ($name:literal, $ty:ty) => {{
                let s: libloading::Symbol<$ty> = lib
                    .get($name)
                    .map_err(|e| Error(format!("missing symbol {}: {e}", stringify!($name))))?;
                *s
            }};
        }

        let syms = Symbols {
            pdf_init:    sym!(b"pdf_init\0",    unsafe extern "C" fn() -> c_int),

            pdf_doc_create:       sym!(b"pdf_doc_create\0",       unsafe extern "C" fn() -> *mut c_void),
            pdf_doc_set_title:    sym!(b"pdf_doc_set_title\0",    unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int),
            pdf_doc_set_author:   sym!(b"pdf_doc_set_author\0",   unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int),
            pdf_doc_set_subject:  sym!(b"pdf_doc_set_subject\0",  unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int),
            pdf_doc_set_creator:  sym!(b"pdf_doc_set_creator\0",  unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int),
            pdf_doc_set_security: sym!(b"pdf_doc_set_security\0", unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char, c_int, c_int) -> c_int),
            pdf_doc_add_page:     sym!(b"pdf_doc_add_page\0",     unsafe extern "C" fn(*mut c_void, c_float, c_float) -> *mut c_void),
            pdf_doc_save_file:    sym!(b"pdf_doc_save_file\0",    unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int),
            pdf_doc_save_buffer:  sym!(b"pdf_doc_save_buffer\0",  unsafe extern "C" fn(*mut c_void, *mut *mut u8, *mut c_int) -> c_int),
            pdf_doc_close:        sym!(b"pdf_doc_close\0",        unsafe extern "C" fn(*mut c_void)),

            pdf_canvas_draw_text:    sym!(b"pdf_canvas_draw_text\0",    unsafe extern "C" fn(*mut c_void, *const c_char, c_float, c_float, *mut c_void) -> c_int),
            pdf_canvas_draw_textbox: sym!(b"pdf_canvas_draw_textbox\0", unsafe extern "C" fn(*mut c_void, *const c_char, c_float, c_float, c_float, c_float, *mut c_void, c_float) -> c_int),
            pdf_canvas_draw_line:    sym!(b"pdf_canvas_draw_line\0",    unsafe extern "C" fn(*mut c_void, c_float, c_float, c_float, c_float, u8, u8, u8, c_float) -> c_int),
            pdf_canvas_draw_rect:    sym!(b"pdf_canvas_draw_rect\0",    unsafe extern "C" fn(*mut c_void, c_float, c_float, c_float, c_float, u8, u8, u8, c_int, u8, u8, u8, c_float, c_int) -> c_int),
            pdf_canvas_draw_ellipse: sym!(b"pdf_canvas_draw_ellipse\0", unsafe extern "C" fn(*mut c_void, c_float, c_float, c_float, c_float, u8, u8, u8, c_int, u8, u8, u8, c_float, c_int) -> c_int),
            pdf_canvas_draw_image:   sym!(b"pdf_canvas_draw_image\0",   unsafe extern "C" fn(*mut c_void, *const u8, c_int, c_int, c_int, c_int, c_float, c_float, c_float, c_float) -> c_int),
            pdf_canvas_draw_table:   sym!(b"pdf_canvas_draw_table\0",   unsafe extern "C" fn(*mut c_void, *mut c_void, c_float, c_float) -> c_int),
            pdf_canvas_add_link:     sym!(b"pdf_canvas_add_link\0",     unsafe extern "C" fn(*mut c_void, c_float, c_float, c_float, c_float, *const c_char) -> c_int),
            pdf_canvas_close:        sym!(b"pdf_canvas_close\0",        unsafe extern "C" fn(*mut c_void)),

            pdf_style_create:          sym!(b"pdf_style_create\0",          unsafe extern "C" fn() -> *mut c_void),
            pdf_style_set_font_family: sym!(b"pdf_style_set_font_family\0", unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int),
            pdf_style_set_font_file:   sym!(b"pdf_style_set_font_file\0",   unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int),
            pdf_style_set_size:        sym!(b"pdf_style_set_size\0",        unsafe extern "C" fn(*mut c_void, c_float) -> c_int),
            pdf_style_set_color:       sym!(b"pdf_style_set_color\0",       unsafe extern "C" fn(*mut c_void, u8, u8, u8) -> c_int),
            pdf_style_set_bold:        sym!(b"pdf_style_set_bold\0",        unsafe extern "C" fn(*mut c_void, c_int) -> c_int),
            pdf_style_set_italic:      sym!(b"pdf_style_set_italic\0",      unsafe extern "C" fn(*mut c_void, c_int) -> c_int),
            pdf_style_set_alignment:   sym!(b"pdf_style_set_alignment\0",   unsafe extern "C" fn(*mut c_void, c_int) -> c_int),
            pdf_style_set_decoration:  sym!(b"pdf_style_set_decoration\0",  unsafe extern "C" fn(*mut c_void, c_int) -> c_int),
            pdf_style_close:           sym!(b"pdf_style_close\0",           unsafe extern "C" fn(*mut c_void)),

            pdf_table_create:          sym!(b"pdf_table_create\0",           unsafe extern "C" fn(*const c_float, c_int) -> *mut c_void),
            pdf_table_set_header_bg:   sym!(b"pdf_table_set_header_bg\0",    unsafe extern "C" fn(*mut c_void, u8, u8, u8) -> c_int),
            pdf_table_set_alternate_bg:sym!(b"pdf_table_set_alternate_bg\0", unsafe extern "C" fn(*mut c_void, u8, u8, u8) -> c_int),
            pdf_table_set_border:      sym!(b"pdf_table_set_border\0",       unsafe extern "C" fn(*mut c_void, u8, u8, u8, c_float) -> c_int),
            pdf_table_set_cell_padding:sym!(b"pdf_table_set_cell_padding\0", unsafe extern "C" fn(*mut c_void, c_float) -> c_int),
            pdf_table_stage_cell:      sym!(b"pdf_table_stage_cell\0",       unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int),
            pdf_table_commit_row:      sym!(b"pdf_table_commit_row\0",       unsafe extern "C" fn(*mut c_void) -> c_int),
            pdf_table_close:           sym!(b"pdf_table_close\0",            unsafe extern "C" fn(*mut c_void)),

            pdf_merge_create:      sym!(b"pdf_merge_create\0",      unsafe extern "C" fn() -> *mut c_void),
            pdf_merge_add_bytes:   sym!(b"pdf_merge_add_bytes\0",   unsafe extern "C" fn(*mut c_void, *const u8, c_int) -> c_int),
            pdf_merge_save_file:   sym!(b"pdf_merge_save_file\0",   unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int),
            pdf_merge_save_buffer: sym!(b"pdf_merge_save_buffer\0", unsafe extern "C" fn(*mut c_void, *mut *mut u8, *mut c_int) -> c_int),
            pdf_merge_close:       sym!(b"pdf_merge_close\0",       unsafe extern "C" fn(*mut c_void)),

            pdf_free:       sym!(b"pdf_free\0",       unsafe extern "C" fn(*mut c_void)),
            pdf_last_error: sym!(b"pdf_last_error\0", unsafe extern "C" fn() -> *const c_char),

            _lib: lib,
        };
        Ok(syms)
    }

    fn last_error(&self) -> String {
        unsafe {
            let ptr = (self.pdf_last_error)();
            if ptr.is_null() {
                return "unknown error".into();
            }
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }
}

// ── Public API ─────────────────────────────────────────────────────────────────

/// A loaded instance of the pdfnative shared library.
///
/// Create one per process with [`PdfLibrary::load`], then use it to create
/// [`PdfDocument`], [`PdfStyle`], [`PdfTable`], and [`PdfMerger`] instances.
/// The underlying library stays loaded as long as any clone of this value
/// or any object derived from it exists (backed by `Arc`).
#[derive(Clone)]
pub struct PdfLibrary(Arc<Symbols>);

impl PdfLibrary {
    /// Load the pdfnative shared library from `path` and initialize the engine.
    ///
    /// On Linux and macOS, sibling shared libraries in the same directory are
    /// pre-loaded with `RTLD_GLOBAL` so that .NET's P/Invoke resolver can locate
    /// them.  Call this once at process startup.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref().canonicalize()
            .map_err(|e| Error(format!("cannot resolve library path: {e}")))?;
        let dir = path.parent().unwrap_or(Path::new("."));

        std::env::set_var("PDFNATIVE_LIB_DIR", dir);

        #[cfg(unix)]
        Self::preload_siblings(dir);

        let syms = unsafe { Symbols::load(&path)? };

        let ret = unsafe { (syms.pdf_init)() };
        if ret != 0 {
            let msg = syms.last_error();
            return Err(Error(format!("pdf_init failed: {msg}")));
        }

        Ok(Self(Arc::new(syms)))
    }

    /// Create a new [`PdfDocument`].
    pub fn document(&self) -> PdfDocument {
        PdfDocument { lib: Arc::clone(&self.0), handle: std::ptr::null_mut() }
    }

    /// Create a new [`PdfStyle`] with default settings (Helvetica 12 pt black).
    pub fn style(&self) -> Result<PdfStyle, Error> {
        let h = unsafe { (self.0.pdf_style_create)() };
        if h.is_null() {
            return Err(Error(format!("pdf_style_create failed: {}", self.0.last_error())));
        }
        Ok(PdfStyle { lib: Arc::clone(&self.0), handle: h })
    }

    /// Create a new [`PdfTable`] with the given column widths (in points).
    pub fn table(&self, col_widths: &[f32]) -> Result<PdfTable, Error> {
        let h = unsafe { (self.0.pdf_table_create)(col_widths.as_ptr(), col_widths.len() as c_int) };
        if h.is_null() {
            return Err(Error(format!("pdf_table_create failed: {}", self.0.last_error())));
        }
        Ok(PdfTable { lib: Arc::clone(&self.0), handle: h })
    }

    /// Create a new [`PdfMerger`].
    pub fn merger(&self) -> Result<PdfMerger, Error> {
        let h = unsafe { (self.0.pdf_merge_create)() };
        if h.is_null() {
            return Err(Error(format!("pdf_merge_create failed: {}", self.0.last_error())));
        }
        Ok(PdfMerger { lib: Arc::clone(&self.0), handle: h })
    }

    #[cfg(unix)]
    fn preload_siblings(dir: &Path) {
        use std::ffi::OsStr;
        #[cfg(target_os = "macos")]
        let ext: &OsStr = OsStr::new("dylib");
        #[cfg(not(target_os = "macos"))]
        let ext: &OsStr = OsStr::new("so");

        let mut paths: Vec<_> = std::fs::read_dir(dir)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension() == Some(ext))
            .collect();
        paths.sort();
        for p in paths {
            unsafe {
                let _ = libloading::os::unix::Library::open(
                    Some(&p),
                    libloading::os::unix::RTLD_LAZY | libloading::os::unix::RTLD_GLOBAL,
                );
            }
        }
    }
}

// ── PdfDocument ────────────────────────────────────────────────────────────────

/// A PDF document being built.  Call [`PdfDocument::close`] when done.
pub struct PdfDocument {
    lib:    Arc<Symbols>,
    handle: *mut c_void,
}

unsafe impl Send for PdfDocument {}

impl PdfDocument {
    fn init(&mut self) -> Result<(), Error> {
        if !self.handle.is_null() { return Ok(()); }
        let h = unsafe { (self.lib.pdf_doc_create)() };
        if h.is_null() {
            return Err(Error(format!("pdf_doc_create failed: {}", self.lib.last_error())));
        }
        self.handle = h;
        Ok(())
    }

    fn check(&self, ret: c_int, fn_name: &str) -> Result<(), Error> {
        if ret != 0 {
            Err(Error(format!("{fn_name} failed: {}", self.lib.last_error())))
        } else {
            Ok(())
        }
    }

    pub fn set_title(&mut self, title: &str) -> Result<&mut Self, Error> {
        self.init()?;
        let s = cstring(title)?;
        self.check(unsafe { (self.lib.pdf_doc_set_title)(self.handle, s.as_ptr()) }, "pdf_doc_set_title")?;
        Ok(self)
    }

    pub fn set_author(&mut self, author: &str) -> Result<&mut Self, Error> {
        self.init()?;
        let s = cstring(author)?;
        self.check(unsafe { (self.lib.pdf_doc_set_author)(self.handle, s.as_ptr()) }, "pdf_doc_set_author")?;
        Ok(self)
    }

    pub fn set_subject(&mut self, subject: &str) -> Result<&mut Self, Error> {
        self.init()?;
        let s = cstring(subject)?;
        self.check(unsafe { (self.lib.pdf_doc_set_subject)(self.handle, s.as_ptr()) }, "pdf_doc_set_subject")?;
        Ok(self)
    }

    pub fn set_creator(&mut self, creator: &str) -> Result<&mut Self, Error> {
        self.init()?;
        let s = cstring(creator)?;
        self.check(unsafe { (self.lib.pdf_doc_set_creator)(self.handle, s.as_ptr()) }, "pdf_doc_set_creator")?;
        Ok(self)
    }

    /// Apply password-based AES encryption.
    ///
    /// `aes256 = true` → AES-256 (PDF 2.0, default);
    /// `aes256 = false` → AES-128 (PDF 1.5+).
    /// `permissions = PERM_ALL` (-1) to allow all operations.
    pub fn set_security(
        &mut self,
        user_password:  &str,
        owner_password: Option<&str>,
        permissions:    c_int,
        aes256:         bool,
    ) -> Result<&mut Self, Error> {
        self.init()?;
        let user  = cstring(user_password)?;
        let owner: Option<CString> = owner_password.map(cstring).transpose()?;
        let owner_ptr = owner.as_ref().map_or(std::ptr::null(), CString::as_ptr);
        let enc_ver = if aes256 { 0 } else { 1 };
        self.check(
            unsafe { (self.lib.pdf_doc_set_security)(self.handle, user.as_ptr(), owner_ptr, permissions, enc_ver) },
            "pdf_doc_set_security",
        )?;
        Ok(self)
    }

    /// Add a page and return a [`PdfCanvas`] for drawing on it.
    ///
    /// Coordinates are in points (1 pt = 1/72 inch); A4 = 595.28 × 841.89.
    pub fn add_page(&mut self, width: f32, height: f32) -> Result<PdfCanvas, Error> {
        self.init()?;
        let h = unsafe { (self.lib.pdf_doc_add_page)(self.handle, width, height) };
        if h.is_null() {
            return Err(Error(format!("pdf_doc_add_page failed: {}", self.lib.last_error())));
        }
        Ok(PdfCanvas { lib: Arc::clone(&self.lib), handle: h })
    }

    /// Write the completed document to `path`.
    pub fn save(&mut self, path: impl AsRef<Path>) -> Result<(), Error> {
        self.init()?;
        let s = cstring(&path.as_ref().to_string_lossy())?;
        self.check(unsafe { (self.lib.pdf_doc_save_file)(self.handle, s.as_ptr()) }, "pdf_doc_save_file")
    }

    /// Write the completed document to memory and return the PDF bytes.
    pub fn save_to_memory(&mut self) -> Result<Vec<u8>, Error> {
        self.init()?;
        let mut buf:  *mut u8 = std::ptr::null_mut();
        let mut size: c_int   = 0;
        self.check(
            unsafe { (self.lib.pdf_doc_save_buffer)(self.handle, &mut buf, &mut size) },
            "pdf_doc_save_buffer",
        )?;
        let bytes = unsafe { std::slice::from_raw_parts(buf, size as usize).to_vec() };
        unsafe { (self.lib.pdf_free)(buf.cast::<c_void>()) };
        Ok(bytes)
    }

    /// Release the document handle.
    pub fn close(&mut self) {
        if !self.handle.is_null() {
            unsafe { (self.lib.pdf_doc_close)(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

impl Drop for PdfDocument {
    fn drop(&mut self) {
        self.close();
    }
}

// ── PdfCanvas ──────────────────────────────────────────────────────────────────

/// A drawing canvas for one page.  Call [`PdfCanvas::close`] when done drawing.
pub struct PdfCanvas {
    lib:    Arc<Symbols>,
    handle: *mut c_void,
}

unsafe impl Send for PdfCanvas {}

impl PdfCanvas {
    fn check(&self, ret: c_int, fn_name: &str) -> Result<(), Error> {
        if ret != 0 {
            Err(Error(format!("{fn_name} failed: {}", self.lib.last_error())))
        } else {
            Ok(())
        }
    }

    /// Draw `text` with its baseline at (`x`, `y`).
    /// `style` is an optional [`PdfStyle`] handle; pass `None` for the default.
    pub fn draw_text(&self, text: &str, x: f32, y: f32, style: Option<&PdfStyle>) -> Result<(), Error> {
        let s    = cstring(text)?;
        let sptr = style.map_or(std::ptr::null_mut(), |s| s.handle);
        self.check(
            unsafe { (self.lib.pdf_canvas_draw_text)(self.handle, s.as_ptr(), x, y, sptr) },
            "pdf_canvas_draw_text",
        )
    }

    /// Draw word-wrapped text in a box.  Returns the char offset of the first
    /// character that did not fit (`== text.len()` if all text rendered).
    pub fn draw_textbox(
        &self,
        text:         &str,
        x:            f32,
        y:            f32,
        width:        f32,
        height:       f32,
        style:        Option<&PdfStyle>,
        line_spacing: f32,
    ) -> Result<usize, Error> {
        let s    = cstring(text)?;
        let sptr = style.map_or(std::ptr::null_mut(), |s| s.handle);
        let ret  = unsafe {
            (self.lib.pdf_canvas_draw_textbox)(self.handle, s.as_ptr(), x, y, width, height, sptr, line_spacing)
        };
        if ret < 0 {
            return Err(Error(format!("pdf_canvas_draw_textbox failed: {}", self.lib.last_error())));
        }
        Ok(ret as usize)
    }

    /// Draw a line from (`x1`, `y1`) to (`x2`, `y2`).
    pub fn draw_line(&self, x1: f32, y1: f32, x2: f32, y2: f32, r: u8, g: u8, b: u8, width: f32) -> Result<(), Error> {
        self.check(
            unsafe { (self.lib.pdf_canvas_draw_line)(self.handle, x1, y1, x2, y2, r, g, b, width) },
            "pdf_canvas_draw_line",
        )
    }

    /// Draw a rectangle.  Pass `fill_rgb` and/or `stroke_rgb` as `(r, g, b)`.
    pub fn draw_rect(
        &self,
        x: f32, y: f32, width: f32, height: f32,
        fill_rgb:     Option<(u8, u8, u8)>,
        stroke_rgb:   Option<(u8, u8, u8)>,
        stroke_width: f32,
    ) -> Result<(), Error> {
        let (fr, fg, fb) = fill_rgb.unwrap_or((0, 0, 0));
        let (sr, sg, sb) = stroke_rgb.unwrap_or((0, 0, 0));
        self.check(
            unsafe {
                (self.lib.pdf_canvas_draw_rect)(
                    self.handle, x, y, width, height,
                    fr, fg, fb, fill_rgb.is_some() as c_int,
                    sr, sg, sb, stroke_width, stroke_rgb.is_some() as c_int,
                )
            },
            "pdf_canvas_draw_rect",
        )
    }

    /// Draw an ellipse bounded by the given rectangle.
    pub fn draw_ellipse(
        &self,
        x: f32, y: f32, width: f32, height: f32,
        fill_rgb:     Option<(u8, u8, u8)>,
        stroke_rgb:   Option<(u8, u8, u8)>,
        stroke_width: f32,
    ) -> Result<(), Error> {
        let (fr, fg, fb) = fill_rgb.unwrap_or((0, 0, 0));
        let (sr, sg, sb) = stroke_rgb.unwrap_or((0, 0, 0));
        self.check(
            unsafe {
                (self.lib.pdf_canvas_draw_ellipse)(
                    self.handle, x, y, width, height,
                    fr, fg, fb, fill_rgb.is_some() as c_int,
                    sr, sg, sb, stroke_width, stroke_rgb.is_some() as c_int,
                )
            },
            "pdf_canvas_draw_ellipse",
        )
    }

    /// Draw an image.  `data` is either JPEG bytes (`is_jpeg = true`) or raw
    /// RGB24 bytes (width × height × 3, `is_jpeg = false`).
    pub fn draw_image(
        &self,
        data:        &[u8],
        pixel_width: u32,
        pixel_height: u32,
        is_jpeg:     bool,
        x: f32, y: f32, w: f32, h: f32,
    ) -> Result<(), Error> {
        self.check(
            unsafe {
                (self.lib.pdf_canvas_draw_image)(
                    self.handle, data.as_ptr(), data.len() as c_int,
                    pixel_width as c_int, pixel_height as c_int, is_jpeg as c_int,
                    x, y, w, h,
                )
            },
            "pdf_canvas_draw_image",
        )
    }

    /// Draw a [`PdfTable`] with its top-left corner at (`x`, `y`).
    pub fn draw_table(&self, table: &PdfTable, x: f32, y: f32) -> Result<(), Error> {
        self.check(
            unsafe { (self.lib.pdf_canvas_draw_table)(self.handle, table.handle, x, y) },
            "pdf_canvas_draw_table",
        )
    }

    /// Add a clickable hyperlink over the given rectangle.
    pub fn add_link(&self, x: f32, y: f32, width: f32, height: f32, uri: &str) -> Result<(), Error> {
        let s = cstring(uri)?;
        self.check(
            unsafe { (self.lib.pdf_canvas_add_link)(self.handle, x, y, width, height, s.as_ptr()) },
            "pdf_canvas_add_link",
        )
    }

    /// Release the canvas handle.  The drawn content is retained in the document.
    pub fn close(&mut self) -> Result<(), Error> {
        if !self.handle.is_null() {
            unsafe { (self.lib.pdf_canvas_close)(self.handle) };
            self.handle = std::ptr::null_mut();
        }
        Ok(())
    }
}

impl Drop for PdfCanvas {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

// ── PdfStyle ───────────────────────────────────────────────────────────────────

/// A text style.  Defaults: Helvetica 12 pt black left-aligned.
pub struct PdfStyle {
    lib:    Arc<Symbols>,
    handle: *mut c_void,
}

unsafe impl Send for PdfStyle {}

impl PdfStyle {
    fn check(&self, ret: c_int, fn_name: &str) -> Result<&Self, Error> {
        if ret != 0 {
            Err(Error(format!("{fn_name} failed: {}", self.lib.last_error())))
        } else {
            Ok(self)
        }
    }

    pub fn set_font_family(&self, family: &str) -> Result<&Self, Error> {
        let s = cstring(family)?;
        self.check(unsafe { (self.lib.pdf_style_set_font_family)(self.handle, s.as_ptr()) }, "pdf_style_set_font_family")
    }

    pub fn set_font_file(&self, path: &str) -> Result<&Self, Error> {
        let s = cstring(path)?;
        self.check(unsafe { (self.lib.pdf_style_set_font_file)(self.handle, s.as_ptr()) }, "pdf_style_set_font_file")
    }

    pub fn set_size(&self, points: f32) -> Result<&Self, Error> {
        self.check(unsafe { (self.lib.pdf_style_set_size)(self.handle, points) }, "pdf_style_set_size")
    }

    pub fn set_color(&self, r: u8, g: u8, b: u8) -> Result<&Self, Error> {
        self.check(unsafe { (self.lib.pdf_style_set_color)(self.handle, r, g, b) }, "pdf_style_set_color")
    }

    pub fn set_bold(&self, bold: bool) -> Result<&Self, Error> {
        self.check(unsafe { (self.lib.pdf_style_set_bold)(self.handle, bold as c_int) }, "pdf_style_set_bold")
    }

    pub fn set_italic(&self, italic: bool) -> Result<&Self, Error> {
        self.check(unsafe { (self.lib.pdf_style_set_italic)(self.handle, italic as c_int) }, "pdf_style_set_italic")
    }

    /// `alignment`: use [`ALIGN_LEFT`], [`ALIGN_CENTER`], or [`ALIGN_RIGHT`].
    pub fn set_alignment(&self, alignment: c_int) -> Result<&Self, Error> {
        self.check(unsafe { (self.lib.pdf_style_set_alignment)(self.handle, alignment) }, "pdf_style_set_alignment")
    }

    /// `decoration`: use [`DECOR_NONE`], [`DECOR_UNDERLINE`], [`DECOR_STRIKETHROUGH`], or [`DECOR_OVERLINE`].
    pub fn set_decoration(&self, decoration: c_int) -> Result<&Self, Error> {
        self.check(unsafe { (self.lib.pdf_style_set_decoration)(self.handle, decoration) }, "pdf_style_set_decoration")
    }

    pub fn close(&mut self) {
        if !self.handle.is_null() {
            unsafe { (self.lib.pdf_style_close)(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

impl Drop for PdfStyle {
    fn drop(&mut self) { self.close(); }
}

// ── PdfTable ───────────────────────────────────────────────────────────────────

/// A table layout.  Call [`PdfTable::close`] when done.
pub struct PdfTable {
    lib:    Arc<Symbols>,
    handle: *mut c_void,
}

unsafe impl Send for PdfTable {}

impl PdfTable {
    fn check(&self, ret: c_int, fn_name: &str) -> Result<&Self, Error> {
        if ret != 0 {
            Err(Error(format!("{fn_name} failed: {}", self.lib.last_error())))
        } else {
            Ok(self)
        }
    }

    pub fn set_header_bg(&self, r: u8, g: u8, b: u8) -> Result<&Self, Error> {
        self.check(unsafe { (self.lib.pdf_table_set_header_bg)(self.handle, r, g, b) }, "pdf_table_set_header_bg")
    }

    pub fn set_alternate_bg(&self, r: u8, g: u8, b: u8) -> Result<&Self, Error> {
        self.check(unsafe { (self.lib.pdf_table_set_alternate_bg)(self.handle, r, g, b) }, "pdf_table_set_alternate_bg")
    }

    pub fn set_border(&self, r: u8, g: u8, b: u8, width: f32) -> Result<&Self, Error> {
        self.check(unsafe { (self.lib.pdf_table_set_border)(self.handle, r, g, b, width) }, "pdf_table_set_border")
    }

    pub fn set_cell_padding(&self, padding: f32) -> Result<&Self, Error> {
        self.check(unsafe { (self.lib.pdf_table_set_cell_padding)(self.handle, padding) }, "pdf_table_set_cell_padding")
    }

    /// Stage all `cells` and commit them as one row.
    pub fn add_row(&self, cells: &[&str]) -> Result<&Self, Error> {
        for cell in cells {
            let s = cstring(cell)?;
            let ret = unsafe { (self.lib.pdf_table_stage_cell)(self.handle, s.as_ptr()) };
            if ret != 0 {
                return Err(Error(format!("pdf_table_stage_cell failed: {}", self.lib.last_error())));
            }
        }
        self.check(unsafe { (self.lib.pdf_table_commit_row)(self.handle) }, "pdf_table_commit_row")
    }

    pub fn close(&mut self) {
        if !self.handle.is_null() {
            unsafe { (self.lib.pdf_table_close)(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

impl Drop for PdfTable {
    fn drop(&mut self) { self.close(); }
}

// ── PdfMerger ─────────────────────────────────────────────────────────────────

/// Merges multiple PDF documents into one.
pub struct PdfMerger {
    lib:    Arc<Symbols>,
    handle: *mut c_void,
}

unsafe impl Send for PdfMerger {}

impl PdfMerger {
    fn check(&self, ret: c_int, fn_name: &str) -> Result<(), Error> {
        if ret != 0 {
            Err(Error(format!("{fn_name} failed: {}", self.lib.last_error())))
        } else {
            Ok(())
        }
    }

    /// Add a PDF supplied as bytes to the merge queue.
    pub fn add_bytes(&self, data: &[u8]) -> Result<(), Error> {
        self.check(
            unsafe { (self.lib.pdf_merge_add_bytes)(self.handle, data.as_ptr(), data.len() as c_int) },
            "pdf_merge_add_bytes",
        )
    }

    /// Merge all added PDFs and write the result to `path`.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let s = cstring(&path.as_ref().to_string_lossy())?;
        self.check(
            unsafe { (self.lib.pdf_merge_save_file)(self.handle, s.as_ptr()) },
            "pdf_merge_save_file",
        )
    }

    /// Merge all added PDFs and return the result as bytes.
    pub fn save_to_memory(&self) -> Result<Vec<u8>, Error> {
        let mut buf:  *mut u8 = std::ptr::null_mut();
        let mut size: c_int   = 0;
        self.check(
            unsafe { (self.lib.pdf_merge_save_buffer)(self.handle, &mut buf, &mut size) },
            "pdf_merge_save_buffer",
        )?;
        let bytes = unsafe { std::slice::from_raw_parts(buf, size as usize).to_vec() };
        unsafe { (self.lib.pdf_free)(buf.cast::<c_void>()) };
        Ok(bytes)
    }

    pub fn close(&mut self) {
        if !self.handle.is_null() {
            unsafe { (self.lib.pdf_merge_close)(self.handle) };
            self.handle = std::ptr::null_mut();
        }
    }
}

impl Drop for PdfMerger {
    fn drop(&mut self) { self.close(); }
}

// ── Utilities ──────────────────────────────────────────────────────────────────

fn cstring(s: &str) -> Result<CString, Error> {
    CString::new(s).map_err(|e| Error(format!("string contains interior NUL byte: {e}")))
}
