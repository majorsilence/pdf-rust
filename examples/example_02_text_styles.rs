//! Example 02 — Text Styles
//!
//! Demonstrates font sizes, bold, italic, colours, alignment, and decorations.
//!
//! Usage:
//!   PDFNATIVE_LIB=/path/to/libpdfnative.so cargo run --example example_02_text_styles

use majorsilence_pdf::{
    PdfLibrary, PAGE_A4,
    ALIGN_LEFT, ALIGN_CENTER, ALIGN_RIGHT,
    DECOR_UNDERLINE, DECOR_STRIKETHROUGH, DECOR_OVERLINE,
};
use std::{env, fs, path::PathBuf};

fn main() {
    let lib_path = env::var("PDFNATIVE_LIB")
        .expect("Set PDFNATIVE_LIB to the path of the pdfnative shared library");

    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples").join("output");
    fs::create_dir_all(&output_dir).unwrap();

    let lib = PdfLibrary::load(&lib_path).expect("failed to load pdfnative");

    let mut doc = lib.document();
    doc.set_title("Text Styles").unwrap();

    let (w, h) = PAGE_A4;
    let mut canvas = doc.add_page(w, h).unwrap();

    let mut y = 50.0_f32;

    let big_heading = lib.style().unwrap();
    big_heading.set_size(24.0).unwrap();
    big_heading.set_bold(true).unwrap();
    canvas.draw_text("Text Styles", 72.0, y, Some(&big_heading)).unwrap();
    y += 36.0;

    let section = lib.style().unwrap();
    section.set_size(18.0).unwrap();
    section.set_bold(true).unwrap();
    canvas.draw_text("Font sizes", 72.0, y, Some(&section)).unwrap();
    y += 26.0;

    for &size in &[8.0_f32, 10.0, 12.0, 14.0, 18.0, 24.0] {
        let s = lib.style().unwrap();
        s.set_size(size).unwrap();
        canvas.draw_text(&format!("{size} pt — The quick brown fox"), 72.0, y, Some(&s)).unwrap();
        y += size + 6.0;
    }
    y += 12.0;

    canvas.draw_text("Bold and italic", 72.0, y, Some(&section)).unwrap();
    y += 26.0;

    let bold = lib.style().unwrap();
    bold.set_size(12.0).unwrap();
    bold.set_bold(true).unwrap();
    canvas.draw_text("Bold text", 72.0, y, Some(&bold)).unwrap();
    y += 18.0;

    let italic = lib.style().unwrap();
    italic.set_size(12.0).unwrap();
    italic.set_italic(true).unwrap();
    canvas.draw_text("Italic text", 72.0, y, Some(&italic)).unwrap();
    y += 18.0;

    let bold_italic = lib.style().unwrap();
    bold_italic.set_size(12.0).unwrap();
    bold_italic.set_bold(true).unwrap();
    bold_italic.set_italic(true).unwrap();
    canvas.draw_text("Bold italic text", 72.0, y, Some(&bold_italic)).unwrap();
    y += 28.0;

    canvas.draw_text("Colour", 72.0, y, Some(&section)).unwrap();
    y += 26.0;

    for &(r, g, b, label) in &[
        (220u8, 0u8,   0u8,   "Red"),
        (0u8,   160u8, 0u8,   "Green"),
        (0u8,   0u8,   200u8, "Blue"),
        (128u8, 128u8, 128u8, "Gray"),
    ] {
        let s = lib.style().unwrap();
        s.set_size(12.0).unwrap();
        s.set_color(r, g, b).unwrap();
        canvas.draw_text(&format!("{label} text (r={r}, g={g}, b={b})"), 72.0, y, Some(&s)).unwrap();
        y += 18.0;
    }
    y += 10.0;

    canvas.draw_text("Alignment", 72.0, y, Some(&section)).unwrap();
    y += 26.0;

    let box_width = w - 144.0;

    let left = lib.style().unwrap();
    left.set_size(12.0).unwrap();
    left.set_alignment(ALIGN_LEFT).unwrap();
    canvas.draw_text("Left-aligned text", 72.0, y, Some(&left)).unwrap();
    y += 18.0;

    let center = lib.style().unwrap();
    center.set_size(12.0).unwrap();
    center.set_alignment(ALIGN_CENTER).unwrap();
    canvas.draw_textbox("Centre-aligned text", 72.0, y, box_width, 20.0, Some(&center), 0.0).unwrap();
    y += 22.0;

    let right = lib.style().unwrap();
    right.set_size(12.0).unwrap();
    right.set_alignment(ALIGN_RIGHT).unwrap();
    canvas.draw_textbox("Right-aligned text", 72.0, y, box_width, 20.0, Some(&right), 0.0).unwrap();
    y += 28.0;

    canvas.draw_text("Decoration", 72.0, y, Some(&section)).unwrap();
    y += 26.0;

    let under = lib.style().unwrap();
    under.set_size(12.0).unwrap();
    under.set_decoration(DECOR_UNDERLINE).unwrap();
    canvas.draw_text("Underlined text", 72.0, y, Some(&under)).unwrap();
    y += 18.0;

    let strike = lib.style().unwrap();
    strike.set_size(12.0).unwrap();
    strike.set_decoration(DECOR_STRIKETHROUGH).unwrap();
    canvas.draw_text("Strikethrough text", 72.0, y, Some(&strike)).unwrap();
    y += 18.0;

    let over = lib.style().unwrap();
    over.set_size(12.0).unwrap();
    over.set_decoration(DECOR_OVERLINE).unwrap();
    canvas.draw_text("Overline text", 72.0, y, Some(&over)).unwrap();

    canvas.close().unwrap();

    let out = output_dir.join("example_02_text_styles.pdf");
    doc.save(&out).unwrap();
    doc.close();

    println!("Written to {}", out.display());
}
