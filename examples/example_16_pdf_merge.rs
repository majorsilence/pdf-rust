//! Example 05 — PDF Merge
//!
//! Creates two PDF documents in memory and merges them into a single file.
//!
//! Usage:
//!   PDFNATIVE_LIB=/path/to/libpdfnative.so cargo run --example example_05_merge

use majorsilence_pdf::{PdfLibrary, PAGE_A4};
use std::{env, fs, path::PathBuf};

fn make_page(lib: &PdfLibrary, title: &str, body: &str) -> Vec<u8> {
    let mut doc = lib.document();
    doc.set_title(title).unwrap();

    let (w, h) = PAGE_A4;
    let mut canvas = doc.add_page(w, h).unwrap();

    let hstyle = lib.style().unwrap();
    hstyle.set_size(20.0).unwrap();
    hstyle.set_bold(true).unwrap();
    canvas.draw_text(title, 72.0, 80.0, Some(&hstyle)).unwrap();

    let bstyle = lib.style().unwrap();
    bstyle.set_size(12.0).unwrap();
    canvas.draw_text(body, 72.0, 120.0, Some(&bstyle)).unwrap();

    canvas.close().unwrap();
    let bytes = doc.save_to_memory().unwrap();
    doc.close();
    bytes
}

fn main() {
    let lib_path = env::var("PDFNATIVE_LIB")
        .expect("Set PDFNATIVE_LIB to the path of the pdfnative shared library");

    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples").join("output");
    fs::create_dir_all(&output_dir).unwrap();

    let lib = PdfLibrary::load(&lib_path).expect("failed to load pdfnative");

    let pdf1 = make_page(&lib, "Document 1 — Cover",    "This is the first document, rendered into memory.");
    let pdf2 = make_page(&lib, "Document 2 — Appendix", "This is the second document, also rendered into memory.");

    let merger = lib.merger().unwrap();
    merger.add_bytes(&pdf1).unwrap();
    merger.add_bytes(&pdf2).unwrap();

    let out = output_dir.join("example_05_merge.pdf");
    merger.save(&out).unwrap();

    println!("Merged PDF written to {}", out.display());
}
