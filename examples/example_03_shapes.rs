//! Example 03 — Shapes
//!
//! Draws lines, rectangles (filled/stroked/both), and ellipses.
//!
//! Usage:
//!   PDFNATIVE_LIB=/path/to/libpdfnative.so cargo run --example example_03_shapes

use majorsilence_pdf::{PdfLibrary, PAGE_A4};
use std::{env, fs, path::PathBuf};

fn main() {
    let lib_path = env::var("PDFNATIVE_LIB")
        .expect("Set PDFNATIVE_LIB to the path of the pdfnative shared library");

    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples").join("output");
    fs::create_dir_all(&output_dir).unwrap();

    let lib = PdfLibrary::load(&lib_path).expect("failed to load pdfnative");

    let mut doc = lib.document();
    doc.set_title("Shapes").unwrap();

    let (w, h) = PAGE_A4;
    let mut canvas = doc.add_page(w, h).unwrap();

    let mut y = 50.0_f32;

    canvas.draw_text("Lines", 72.0, y, None).unwrap();
    y += 20.0;

    let lines: &[(u8, u8, u8, f32)] = &[
        (0,   0,   0,   0.5),
        (200, 0,   0,   1.0),
        (0,   0,   200, 2.0),
        (0,   150, 0,   3.0),
    ];
    for (i, &(r, g, b, lw)) in lines.iter().enumerate() {
        canvas.draw_line(72.0, y + i as f32 * 14.0, 400.0, y + i as f32 * 14.0, r, g, b, lw).unwrap();
    }
    y += 80.0;

    canvas.draw_text("Filled rectangles", 72.0, y, None).unwrap();
    y += 16.0;

    let colors: &[(u8, u8, u8)] = &[(220, 50, 50), (50, 150, 50), (50, 50, 220), (200, 150, 0)];
    for (i, &(r, g, b)) in colors.iter().enumerate() {
        canvas.draw_rect(72.0 + i as f32 * 110.0, y, 100.0, 60.0, Some((r, g, b)), None, 1.0).unwrap();
    }
    y += 80.0;

    canvas.draw_text("Stroked rectangles", 72.0, y, None).unwrap();
    y += 16.0;

    for (i, &(r, g, b)) in colors.iter().enumerate() {
        canvas.draw_rect(72.0 + i as f32 * 110.0, y, 100.0, 60.0, None, Some((r, g, b)), 2.0).unwrap();
    }
    y += 80.0;

    canvas.draw_text("Filled + stroked rectangles", 72.0, y, None).unwrap();
    y += 16.0;

    canvas.draw_rect(72.0,  y, 100.0, 60.0, Some((240, 200, 200)), Some((180, 0, 0)),   2.0).unwrap();
    canvas.draw_rect(182.0, y, 100.0, 60.0, Some((200, 240, 200)), Some((0, 160, 0)),   2.0).unwrap();
    canvas.draw_rect(292.0, y, 100.0, 60.0, Some((200, 220, 255)), Some((0, 0, 180)),   2.0).unwrap();
    y += 90.0;

    canvas.draw_text("Ellipses", 72.0, y, None).unwrap();
    y += 16.0;

    canvas.draw_ellipse(72.0,  y, 120.0, 80.0, Some((220, 80, 80)), None,              1.0).unwrap();
    canvas.draw_ellipse(210.0, y, 120.0, 80.0, Some((80, 200, 80)), None,              1.0).unwrap();
    canvas.draw_ellipse(348.0, y, 100.0, 80.0, None,                Some((0, 0, 200)), 2.0).unwrap();
    y += 100.0;

    canvas.draw_text("Crosshair in circle", 72.0, y, None).unwrap();
    y += 16.0;

    let (cx, cy, r) = (160.0_f32, y + 50.0, 40.0_f32);
    canvas.draw_ellipse(cx - r, cy - r, r * 2.0, r * 2.0, None, Some((0, 0, 0)), 1.0).unwrap();
    canvas.draw_line(cx - r, cy, cx + r, cy, 0, 0, 0, 0.5).unwrap();
    canvas.draw_line(cx, cy - r, cx, cy + r, 0, 0, 0, 0.5).unwrap();

    canvas.close().unwrap();

    let out = output_dir.join("example_03_shapes.pdf");
    doc.save(&out).unwrap();
    doc.close();

    println!("Written to {}", out.display());
}
