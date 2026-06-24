use majorsilence_pdf::{PdfLibrary, ALIGN_RIGHT};
use std::{env, fs, path::Path};

fn main() {
    let lib_path = env::var("PDFNATIVE_LIB").expect("Set PDFNATIVE_LIB");
    let rtl_font = env::var("RTL_FONT_PATH").unwrap_or_default();
    let out_dir  = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/output");
    fs::create_dir_all(&out_dir).unwrap();

    let lib    = PdfLibrary::load(&lib_path).expect("load library");
    let (w, h) = (595.28_f32, 841.89_f32);

    let rtl_samples: &[(&str, &str)] = &[
        ("Arabic — مرحبا بالعالم",  "مرحبا بالعالم! هذا مثال على النص العربي في ملف PDF."),
        ("Arabic — رقم",             "١ ٢ ٣ ٤ ٥ ٦ ٧ ٨ ٩ ٠"),
        ("Hebrew — שלום עולם",       "שלום! זהו טקסט עברי בתוך קובץ PDF."),
        ("Bidirectional — EN/AR",    "Price: ٢٥٠ USD — السعر: ٢٥٠ دولار"),
    ];

    let mut doc = lib.document();
    doc.set_title("RTL Text");
    let canvas = doc.add_page(w, h);

    let heading = lib.style(); heading.set_size(18.0).set_bold();
    canvas.draw_text("Right-to-Left Text", 72.0, 50.0, Some(&heading)); drop(heading);

    let use_font = !rtl_font.is_empty() && Path::new(&rtl_font).exists();
    let info = lib.style(); info.set_size(10.0);
    if use_font {
        info.set_color(0, 100, 0);
        let name = Path::new(&rtl_font).file_name().unwrap().to_string_lossy();
        canvas.draw_text(&format!("RTL font: {}", name), 72.0, 72.0, Some(&info));
    } else {
        info.set_color(160, 80, 0);
        canvas.draw_text("RTL_FONT_PATH not set. Glyphs may not render correctly.", 72.0, 72.0, Some(&info));
    }
    drop(info);

    let mut y  = 100.0_f32;
    let lbl_s  = lib.style(); lbl_s.set_size(9.0).set_color(100, 100, 100);
    let rtl_s  = lib.style(); rtl_s.set_size(14.0).set_alignment(ALIGN_RIGHT);
    if use_font { rtl_s.set_font_file(&rtl_font); }

    for &(label, text) in rtl_samples {
        canvas.draw_text(label, 72.0, y, Some(&lbl_s)); y += 14.0;
        canvas.draw_text(text,  72.0, y, Some(&rtl_s)); y += 24.0;
        canvas.draw_line(72.0, y, w - 72.0, y, 220, 220, 220, 0.3); y += 8.0;
    }
    drop(lbl_s); drop(rtl_s);

    let note = lib.style(); note.set_size(9.0).set_color(130, 130, 130);
    canvas.draw_textbox(
        "Note: Full RTL shaping (ligatures, contextual forms) requires an OpenType font \
         with Arabic/Hebrew GSUB/GPOS tables and a shaping engine (e.g. HarfBuzz).",
        72.0, y + 10.0, w - 144.0, 60.0, Some(&note),
    ); drop(note);

    drop(canvas);
    let out = out_dir.join("example_15_rtl_text.pdf");
    doc.save(out.to_str().unwrap()).unwrap();
    println!("Written to {}", out.display());
}
