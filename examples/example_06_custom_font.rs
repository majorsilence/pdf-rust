use majorsilence_pdf::PdfLibrary;
use std::{env, fs, path::Path};

fn main() {
    let lib_path  = env::var("PDFNATIVE_LIB").expect("Set PDFNATIVE_LIB");
    let font_path = env::var("CUSTOM_FONT_PATH").unwrap_or_default();
    let out_dir   = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/output");
    fs::create_dir_all(&out_dir).unwrap();

    let lib    = PdfLibrary::load(&lib_path).expect("load library");
    let (w, h) = (595.28_f32, 841.89_f32);

    let mut doc = lib.document();
    doc.set_title("Custom Font");
    let canvas = doc.add_page(w, h);

    let heading = lib.style(); heading.set_size(18.0).set_bold();
    canvas.draw_text("Custom Font Embedding", 72.0, 50.0, Some(&heading)); drop(heading);

    let use_font = !font_path.is_empty() && Path::new(&font_path).exists();

    if use_font {
        let s = lib.style(); s.set_size(10.0).set_color(80, 80, 80);
        let name = Path::new(&font_path).file_name().unwrap().to_string_lossy();
        canvas.draw_text(&format!("Font file: {}", name), 72.0, 78.0, Some(&s)); drop(s);

        let mut y = 100.0_f32;
        for &size in &[10.0_f32, 12.0, 14.0, 18.0, 24.0, 32.0] {
            let s = lib.style(); s.set_font_file(&font_path).set_size(size);
            canvas.draw_text(&format!("{} pt — The quick brown fox jumps over the lazy dog", size as u32),
                             72.0, y, Some(&s)); drop(s);
            y += size + 8.0;
        }
        y += 10.0;
        let s = lib.style(); s.set_font_file(&font_path).set_size(14.0).set_bold();
        canvas.draw_text("Bold variant (if supported by the font file):", 72.0, y, Some(&s)); drop(s);
        y += 22.0;
        let s = lib.style(); s.set_font_file(&font_path).set_size(12.0).set_italic();
        canvas.draw_text("Italic variant (if supported by the font file):", 72.0, y, Some(&s)); drop(s);
    } else {
        let s = lib.style(); s.set_size(11.0).set_color(180, 0, 0);
        canvas.draw_text("CUSTOM_FONT_PATH not set or file not found.", 72.0, 100.0, Some(&s));
        canvas.draw_text("Set CUSTOM_FONT_PATH=/path/to/a/TrueType.ttf and re-run.", 72.0, 118.0, Some(&s));
        drop(s);

        let s = lib.style(); s.set_size(11.0).set_color(80, 80, 80);
        canvas.draw_text("Falling back to built-in Helvetica:", 72.0, 152.0, Some(&s)); drop(s);

        let mut y = 172.0_f32;
        for &size in &[10.0_f32, 12.0, 14.0, 18.0, 24.0] {
            let s = lib.style(); s.set_size(size);
            canvas.draw_text(&format!("{} pt — The quick brown fox (Helvetica)", size as u32),
                             72.0, y, Some(&s)); drop(s);
            y += size + 8.0;
        }
    }

    drop(canvas);
    let out = out_dir.join("example_06_custom_font.pdf");
    doc.save(out.to_str().unwrap()).unwrap();
    println!("Written to {}", out.display());
}
