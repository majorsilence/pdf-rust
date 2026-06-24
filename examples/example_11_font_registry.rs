use majorsilence_pdf::PdfLibrary;
use std::{env, fs, path::Path};

fn main() {
    let lib_path = env::var("PDFNATIVE_LIB").expect("Set PDFNATIVE_LIB");
    let font_dir = env::var("FONT_DIR").unwrap_or_default();
    let out_dir  = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/output");
    fs::create_dir_all(&out_dir).unwrap();

    let lib    = PdfLibrary::load(&lib_path).expect("load library");
    let (w, h) = (595.28_f32, 841.89_f32);
    let sample = "The quick brown fox jumps over the lazy dog  0123456789";

    let mut doc = lib.document();
    doc.set_title("Font Registry");
    let canvas = doc.add_page(w, h);

    let heading = lib.style(); heading.set_size(18.0).set_bold();
    canvas.draw_text("Font Registry", 72.0, 50.0, Some(&heading)); drop(heading);

    let mut y = 80.0_f32;

    let mut font_files: Vec<String> = vec![];
    if !font_dir.is_empty() {
        if let Ok(entries) = fs::read_dir(&font_dir) {
            for e in entries.flatten() {
                let p = e.path();
                if p.extension().and_then(|x| x.to_str()) == Some("ttf") {
                    font_files.push(p.to_string_lossy().into_owned());
                }
            }
            font_files.sort();
            font_files.truncate(12);
        }
    }

    if !font_files.is_empty() {
        let info = lib.style(); info.set_size(9.0).set_color(100, 100, 100);
        canvas.draw_text(&format!("Loaded {} font(s) from {}", font_files.len(), font_dir), 72.0, y, Some(&info));
        drop(info); y += 16.0;

        for font_path in &font_files {
            let name = Path::new(font_path).file_stem().unwrap().to_string_lossy();
            let lbl  = lib.style(); lbl.set_size(8.0).set_color(100, 100, 100);
            canvas.draw_text(&name, 72.0, y, Some(&lbl)); drop(lbl); y += 11.0;
            let s = lib.style(); s.set_font_file(font_path).set_size(12.0);
            canvas.draw_text(sample, 72.0, y, Some(&s)); drop(s); y += 20.0;
            canvas.draw_line(72.0, y, w - 72.0, y, 220, 220, 220, 0.3); y += 6.0;
        }
    } else {
        let info = lib.style(); info.set_size(10.0).set_color(180, 0, 0);
        canvas.draw_text("FONT_DIR not set — falling back to built-in Helvetica.", 72.0, y, Some(&info)); y += 16.0;
        canvas.draw_text("Set FONT_DIR to a directory of .ttf files.", 72.0, y, Some(&info));
        drop(info); y += 30.0;

        for &(variant, bold, italic) in &[
            ("Regular",     false, false),
            ("Bold",        true,  false),
            ("Italic",      false, true),
            ("Bold-Italic", true,  true),
        ] {
            let lbl = lib.style(); lbl.set_size(8.0).set_color(100, 100, 100);
            canvas.draw_text(&format!("Helvetica {}", variant), 72.0, y, Some(&lbl)); drop(lbl); y += 11.0;
            let s = lib.style(); s.set_size(12.0);
            if bold   { s.set_bold(); }
            if italic { s.set_italic(); }
            canvas.draw_text(sample, 72.0, y, Some(&s)); drop(s); y += 20.0;
        }
    }

    drop(canvas);
    let out = out_dir.join("example_11_font_registry.pdf");
    doc.save(out.to_str().unwrap()).unwrap();
    println!("Written to {}", out.display());
}
