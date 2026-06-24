use majorsilence_pdf::PdfLibrary;
use std::{env, fs, path::Path};

fn main() {
    let lib_path = env::var("PDFNATIVE_LIB").expect("Set PDFNATIVE_LIB");
    let uni_font = env::var("UNICODE_FONT_PATH").unwrap_or_default();
    let out_dir  = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/output");
    fs::create_dir_all(&out_dir).unwrap();

    let lib    = PdfLibrary::load(&lib_path).expect("load library");
    let (w, h) = (595.28_f32, 841.89_f32);

    let samples: &[(&str, &str)] = &[
        ("Latin (basic)",   "Hello, World! 0 1 2 3 4 5 6 7 8 9"),
        ("Latin extended",  "Héllo Wörld — café, naïve, résumé, façade"),
        ("Greek",           "Ελληνικά — Αλφάβητο Αβγδεζηθ"),
        ("Cyrillic",        "Привет мир — кириллица"),
        ("Symbols",         "© ® ™ € £ ¥ § ¶ † ‡ • … ‰"),
        ("Arrows & math",   "← → ↑ ↓ ↔ ∑ ∏ √ ∞ ≠ ≤ ≥ ∈"),
        ("Box drawing",     "┌─┬─┐  │ │ │  ├─┼─┤  └─┴─┘"),
    ];

    let mut doc = lib.document();
    doc.set_title("Unicode Text");
    let canvas = doc.add_page(w, h);

    let heading = lib.style(); heading.set_size(18.0).set_bold();
    canvas.draw_text("Unicode Text Rendering", 72.0, 50.0, Some(&heading)); drop(heading);

    let use_font = !uni_font.is_empty() && Path::new(&uni_font).exists();
    if use_font {
        let info = lib.style(); info.set_size(9.0).set_color(80, 80, 80);
        let name = Path::new(&uni_font).file_name().unwrap().to_string_lossy();
        canvas.draw_text(&format!("Using font: {}", name), 72.0, 72.0, Some(&info)); drop(info);
    }

    let mut y = 90.0_f32;
    let lbl_s  = lib.style(); lbl_s.set_size(8.0).set_color(100, 100, 100);
    let smp_s  = lib.style(); smp_s.set_size(12.0);
    if use_font { smp_s.set_font_file(&uni_font); }

    for &(script, text) in samples {
        canvas.draw_text(script, 72.0, y, Some(&lbl_s)); y += 12.0;
        canvas.draw_text(text,   72.0, y, Some(&smp_s)); y += 20.0;
        canvas.draw_line(72.0, y, w - 72.0, y, 220, 220, 220, 0.3); y += 6.0;
    }
    drop(lbl_s); drop(smp_s);

    let note = lib.style(); note.set_size(8.0).set_color(130, 130, 130);
    canvas.draw_text(
        "Tip: set UNICODE_FONT_PATH to a wide-coverage font (e.g. Noto Sans) for full glyph rendering.",
        72.0, y + 10.0, Some(&note),
    ); drop(note);

    drop(canvas);
    let out = out_dir.join("example_12_unicode.pdf");
    doc.save(out.to_str().unwrap()).unwrap();
    println!("Written to {}", out.display());
}
