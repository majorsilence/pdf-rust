//! Example 06 — Password Protection (Security)
//!
//! Creates an AES-256 password-protected PDF.
//!
//! Usage:
//!   PDFNATIVE_LIB=/path/to/libpdfnative.so cargo run --example example_06_security

use majorsilence_pdf::{PdfLibrary, PAGE_A4, PERM_PRINT, PERM_COPY_TEXT, PERM_PRINT_HIGH_QUALITY};
use std::{env, fs, path::PathBuf};

fn main() {
    let lib_path = env::var("PDFNATIVE_LIB")
        .expect("Set PDFNATIVE_LIB to the path of the pdfnative shared library");

    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples").join("output");
    fs::create_dir_all(&output_dir).unwrap();

    let lib = PdfLibrary::load(&lib_path).expect("failed to load pdfnative");

    let mut doc = lib.document();
    doc.set_title("Password Protected Document").unwrap();
    doc.set_author("Majorsilence PDF").unwrap();

    // AES-256 encryption (enc_version = 0).
    // User opens with "userpass"; owner controls with "ownerpass".
    doc.set_security(
        "userpass",
        Some("ownerpass"),
        PERM_PRINT | PERM_COPY_TEXT | PERM_PRINT_HIGH_QUALITY,
        true,
    ).unwrap();

    let (w, h) = PAGE_A4;
    let mut canvas = doc.add_page(w, h).unwrap();

    let heading = lib.style().unwrap();
    heading.set_size(20.0).unwrap();
    heading.set_bold(true).unwrap();
    canvas.draw_text("Password Protected PDF", 72.0, 80.0, Some(&heading)).unwrap();

    let body = lib.style().unwrap();
    body.set_size(12.0).unwrap();
    canvas.draw_text("This document is encrypted with AES-256.",            72.0, 120.0, Some(&body)).unwrap();
    canvas.draw_text("Open it with password: userpass",                     72.0, 140.0, Some(&body)).unwrap();
    canvas.draw_text("Full editing requires password: ownerpass",           72.0, 160.0, Some(&body)).unwrap();
    canvas.draw_text("Allowed operations: Print, CopyText, PrintHighQuality", 72.0, 180.0, Some(&body)).unwrap();

    canvas.close().unwrap();

    let out = output_dir.join("example_06_security.pdf");
    doc.save(&out).unwrap();
    doc.close();

    println!("Password-protected PDF written to {}", out.display());
    println!("User password: userpass   |   Owner password: ownerpass");
}
