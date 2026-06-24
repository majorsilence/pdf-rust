//! Example 04 — Table
//!
//! Creates a styled table with a header, alternating row colours, and a border.
//!
//! Usage:
//!   PDFNATIVE_LIB=/path/to/libpdfnative.so cargo run --example example_04_table

use majorsilence_pdf::{PdfLibrary, PAGE_A4};
use std::{env, fs, path::PathBuf};

fn main() {
    let lib_path = env::var("PDFNATIVE_LIB")
        .expect("Set PDFNATIVE_LIB to the path of the pdfnative shared library");

    let output_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples").join("output");
    fs::create_dir_all(&output_dir).unwrap();

    let lib = PdfLibrary::load(&lib_path).expect("failed to load pdfnative");

    let mut doc = lib.document();
    doc.set_title("Table Example").unwrap();

    let (w, h) = PAGE_A4;
    let mut canvas = doc.add_page(w, h).unwrap();

    let heading = lib.style().unwrap();
    heading.set_size(18.0).unwrap();
    heading.set_bold(true).unwrap();
    canvas.draw_text("Table Layout", 72.0, 40.0, Some(&heading)).unwrap();

    let label = lib.style().unwrap();
    label.set_size(10.0).unwrap();
    canvas.draw_text(
        "Sales report — styled table with header and alternating rows:",
        72.0, 76.0, Some(&label),
    ).unwrap();

    let table = lib.table(&[180.0, 80.0, 90.0, 90.0]).unwrap();
    table.set_header_bg(26, 86, 160).unwrap();
    table.set_alternate_bg(240, 245, 252).unwrap();
    table.set_border(200, 200, 200, 0.5).unwrap();
    table.set_cell_padding(5.0).unwrap();
    table.add_row(&["Product",          "Qty", "Unit Price", "Total"]).unwrap();
    table.add_row(&["PDF Library Pro",  "3",   "$400.00",    "$1,200.00"]).unwrap();
    table.add_row(&["Report Designer",  "1",   "$250.00",    "$250.00"]).unwrap();
    table.add_row(&["Integration Pack", "2",   "$180.00",    "$360.00"]).unwrap();
    table.add_row(&["Support (12 mo.)", "1",   "$500.00",    "$500.00"]).unwrap();
    table.add_row(&["",                  "",    "Total:",     "$2,310.00"]).unwrap();
    canvas.draw_table(&table, 72.0, 92.0).unwrap();

    canvas.draw_text("Borderless table:", 72.0, 330.0, Some(&label)).unwrap();

    let report = lib.table(&[200.0, 100.0, 100.0]).unwrap();
    report.set_alternate_bg(245, 245, 245).unwrap();
    report.set_border(0, 0, 0, 0.0).unwrap();
    report.set_cell_padding(4.0).unwrap();
    report.add_row(&["Region",         "Revenue", "Growth"]).unwrap();
    report.add_row(&["North America",  "$1.24M",  "+12%"]).unwrap();
    report.add_row(&["Europe",         "$0.89M",  "+8%"]).unwrap();
    report.add_row(&["Asia Pacific",   "$0.45M",  "+18%"]).unwrap();
    report.add_row(&["Other",          "$0.12M",  "+3%"]).unwrap();
    canvas.draw_table(&report, 72.0, 346.0).unwrap();

    canvas.close().unwrap();

    let out = output_dir.join("example_04_table.pdf");
    doc.save(&out).unwrap();
    doc.close();

    println!("Written to {}", out.display());
}
