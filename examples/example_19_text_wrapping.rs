use majorsilence_pdf::{PdfLibrary, ALIGN_LEFT, ALIGN_CENTER, ALIGN_RIGHT};
use std::{env, fs, path::Path};

const LOREM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
     tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, \
     quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo \
     consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse \
     cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non \
     proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

fn main() {
    let lib_path = env::var("PDFNATIVE_LIB").expect("Set PDFNATIVE_LIB");
    let out_dir  = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/output");
    fs::create_dir_all(&out_dir).unwrap();

    let lib    = PdfLibrary::load(&lib_path).expect("load library");
    let (w, h) = (595.28_f32, 841.89_f32);
    let margin = 60.0_f32;
    let tw     = w - 2.0 * margin;

    let mut doc = lib.document();
    doc.set_title("Text Wrapping");
    let canvas = doc.add_page(w, h);

    let heading = lib.style(); heading.set_size(18.0).set_bold();
    canvas.draw_text("Text Wrapping with draw_textbox", margin, 44.0, Some(&heading)); drop(heading);

    let mut y   = 70.0_f32;
    let lbl_s   = lib.style(); lbl_s.set_size(9.0).set_bold().set_color(26, 86, 160);

    for &(label, align) in &[
        ("Left-aligned (full width)", ALIGN_LEFT),
        ("Centred",                   ALIGN_CENTER),
        ("Right-aligned",             ALIGN_RIGHT),
    ] {
        canvas.draw_text(label, margin, y, Some(&lbl_s)); y += 12.0;
        let s = lib.style(); s.set_size(11.0).set_alignment(align);
        canvas.draw_textbox(LOREM, margin, y, tw, 80.0, Some(&s)); drop(s);
        canvas.draw_rect(margin, y, tw, 80.0, None, Some((200, 200, 200)), 0.3);
        y += 92.0;
    }

    // Narrow two-column layout
    canvas.draw_text("Narrow column (160 pt wide)", margin, y, Some(&lbl_s)); y += 12.0;
    let s = lib.style(); s.set_size(10.0).set_alignment(ALIGN_LEFT);
    canvas.draw_textbox(LOREM, margin, y, 160.0, 200.0, Some(&s)); drop(s);
    canvas.draw_rect(margin, y, 160.0, 200.0, None, Some((200, 200, 200)), 0.3);

    let s = lib.style(); s.set_size(10.0).set_alignment(ALIGN_LEFT);
    canvas.draw_textbox(LOREM, margin + 180.0, y, 160.0, 200.0, Some(&s)); drop(s);
    canvas.draw_rect(margin + 180.0, y, 160.0, 200.0, None, Some((200, 200, 200)), 0.3);

    drop(lbl_s);
    drop(canvas);
    let out = out_dir.join("example_19_text_wrapping.pdf");
    doc.save(out.to_str().unwrap()).unwrap();
    println!("Written to {}", out.display());
}
