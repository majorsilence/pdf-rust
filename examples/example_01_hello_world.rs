//! Example 01 — Hello World
//!
//! Creates a single A4 page PDF with a title and body text.
//!
//! Usage:
//!   PDFNATIVE_LIB=/path/to/libpdfnative.so cargo run --example example_01_hello_world

use majorsilence_pdf::{PdfLibrary, PAGE_A4};
use std::{env, fs, path::PathBuf};

fn main() {
    let lib_path = env::var("PDFNATIVE_LIB")
        .expect("Set PDFNATIVE_LIB to the path of the pdfnative shared library");

    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples").join("output");
    fs::create_dir_all(&output_dir).unwrap();

    let lib = PdfLibrary::load(&lib_path).expect("failed to load pdfnative");

    let mut doc = lib.document();
    doc.set_title("Hello World").unwrap();
    doc.set_author("Majorsilence PDF").unwrap();

    let heading = lib.style().unwrap();
    heading.set_size(24.0).unwrap();
    heading.set_bold(true).unwrap();

    let body = lib.style().unwrap();
    body.set_size(12.0).unwrap();

    let (w, h) = PAGE_A4;
    let mut canvas = doc.add_page(w, h).unwrap();

    canvas.draw_text("Hello, PDF!", 72.0, 80.0, Some(&heading)).unwrap();
    canvas.draw_text("This PDF was created with the Majorsilence pdfnative library.", 72.0, 120.0, Some(&body)).unwrap();
    canvas.draw_text("No .NET runtime is required — the engine runs in-process via FFI.", 72.0, 140.0, Some(&body)).unwrap();
    canvas.close().unwrap();

    let out = output_dir.join("example_01_hello_world.pdf");
    doc.save(&out).unwrap();
    doc.close();

    println!("Written to {}", out.display());
}
