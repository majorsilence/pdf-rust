use majorsilence_pdf::PdfLibrary;
use std::{env, fs, path::Path};

fn main() {
    let lib_path = env::var("PDFNATIVE_LIB").expect("Set PDFNATIVE_LIB");
    let out_dir  = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/output");
    fs::create_dir_all(&out_dir).unwrap();

    let lib         = PdfLibrary::load(&lib_path).expect("load library");
    let (w, h)      = (595.28_f32, 841.89_f32);
    let margin      = 60.0_f32;
    let (br, bg, bb) = (26_u8, 86_u8, 160_u8);

    let mut doc = lib.document();
    doc.set_title("Invoice #INV-2025-042");
    doc.set_author("Acme Corporation");
    let canvas = doc.add_page(w, h);

    // Header band
    canvas.draw_rect(0.0, 0.0, w, 100.0, Some((br, bg, bb)), None, 0.0);
    let s = lib.style(); s.set_size(28.0).set_bold().set_color(255, 255, 255);
    canvas.draw_text("ACME CORPORATION", margin, 30.0, Some(&s)); drop(s);
    let s = lib.style(); s.set_size(11.0).set_color(180, 210, 255);
    canvas.draw_text("123 Enterprise Drive · Silicon Valley, CA 94025", margin, 62.0, Some(&s));
    canvas.draw_text("billing@acme.example  ·  +1 (800) 555-0100", margin, 78.0, Some(&s)); drop(s);
    let s = lib.style(); s.set_size(22.0).set_bold().set_color(255, 255, 255);
    canvas.draw_text("INVOICE", w - 160.0, 40.0, Some(&s)); drop(s);

    // Metadata
    let mut y_meta   = 118.0_f32;
    let meta_right   = w - margin - 140.0;
    let label_s      = lib.style(); label_s.set_size(9.0).set_color(100, 100, 100);
    let value_s      = lib.style(); value_s.set_size(10.0).set_bold();
    for &(k, v) in &[
        ("Invoice No.", "INV-2025-042"), ("Date", "2025-11-15"),
        ("Due Date",    "2025-12-15"),   ("Currency", "USD"),
    ] {
        canvas.draw_text(k, meta_right, y_meta, Some(&label_s));
        canvas.draw_text(v, meta_right + 70.0, y_meta, Some(&value_s));
        y_meta += 16.0;
    }
    drop(label_s); drop(value_s);

    // Bill To
    let mut y_bill = 118.0_f32;
    let s = lib.style(); s.set_size(9.0).set_bold().set_color(br, bg, bb);
    canvas.draw_text("BILL TO", margin, y_bill, Some(&s)); drop(s); y_bill += 14.0;
    let s = lib.style(); s.set_size(10.0).set_bold();
    canvas.draw_text("Globex Enterprises Ltd.", margin, y_bill, Some(&s)); drop(s); y_bill += 14.0;
    let s = lib.style(); s.set_size(10.0);
    for line in &["Attn: Mr. H. J. Simpson", "742 Evergreen Terrace", "Springfield, IL 62701"] {
        canvas.draw_text(line, margin, y_bill, Some(&s)); y_bill += 14.0;
    }
    drop(s);

    // Divider and table
    let mut y = 220.0_f32;
    canvas.draw_line(margin, y, w - margin, y, 200, 200, 200, 0.5);
    y += 12.0;

    let table = lib.table(&[210.0, 50.0, 80.0, 80.0, 80.0]);
    table.set_header_bg(br, bg, bb);
    table.set_alternate_bg(245, 248, 255);
    table.set_border(210, 210, 210, 0.5);
    table.set_cell_padding(5.0);
    table.add_row(&["Description", "Qty", "Unit Price", "Discount", "Line Total"]);
    table.add_row(&["PDF Library Pro",  "3", "$400.00", "10%", "$1,080.00"]);
    table.add_row(&["Report Designer",  "1", "$250.00", "—",   "$250.00"]);
    table.add_row(&["Integration Pack", "2", "$180.00", "—",   "$360.00"]);
    table.add_row(&["Priority Support", "1", "$500.00", "—",   "$500.00"]);
    canvas.draw_table(&table, margin, y);
    drop(table);
    y += 185.0;

    // Totals
    canvas.draw_line(w - 220.0, y, w - margin, y, br, bg, bb, 0.5);
    y += 6.0;
    for &(lbl, amt, is_total) in &[
        ("Subtotal",  "$2,190.00", false),
        ("Tax (8%)",  "$175.20",   false),
        ("Total Due", "$2,365.20", true),
    ] {
        let s = lib.style();
        s.set_size(if is_total { 11.0 } else { 10.0 });
        if is_total { s.set_bold(); }
        canvas.draw_text(lbl, w - 220.0, y, Some(&s));
        canvas.draw_text(amt, w - margin - 60.0, y, Some(&s));
        drop(s);
        y += 18.0;
    }
    canvas.draw_line(w - 220.0, y, w - margin, y, br, bg, bb, 1.0);

    // Footer
    canvas.draw_line(margin, h - 60.0, w - margin, h - 60.0, 200, 200, 200, 0.5);
    let s = lib.style(); s.set_size(8.0).set_color(130, 130, 130);
    canvas.draw_text("Payment terms: Net 30. Make cheques payable to Acme Corporation.", margin, h - 48.0, Some(&s));
    canvas.draw_text("Bank: First National · Routing 021000021 · Account 123456789", margin, h - 36.0, Some(&s));
    drop(s);

    drop(canvas);
    let out = out_dir.join("example_09_invoice.pdf");
    doc.save(out.to_str().unwrap()).unwrap();
    println!("Written to {}", out.display());
}
