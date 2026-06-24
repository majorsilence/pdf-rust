use majorsilence_pdf::PdfLibrary;
use std::{env, fs, path::Path};

fn main() {
    let lib_path = env::var("PDFNATIVE_LIB").expect("Set PDFNATIVE_LIB");
    let out_dir  = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/output");
    fs::create_dir_all(&out_dir).unwrap();

    let lib    = PdfLibrary::load(&lib_path).expect("load library");
    let (w, h) = (595.28_f32, 841.89_f32);

    let mut doc = lib.document();
    doc.set_title("Multi-Page Document");
    doc.set_author("Majorsilence PDF");

    // ── Page 1: Cover ─────────────────────────────────────────────────────────
    {
        let canvas = doc.add_page(w, h);
        canvas.draw_rect(0.0, 0.0, w, 200.0, Some((26, 86, 160)), None, 0.0);
        let s = lib.style(); s.set_size(32.0).set_bold().set_color(255, 255, 255);
        canvas.draw_text("Annual Report 2025", 72.0, 80.0, Some(&s)); drop(s);
        let s = lib.style(); s.set_size(14.0).set_color(200, 220, 255);
        canvas.draw_text("Majorsilence Corporation", 72.0, 130.0, Some(&s)); drop(s);
        let s = lib.style(); s.set_size(12.0);
        canvas.draw_text("This document demonstrates a multi-page PDF.", 72.0, 250.0, Some(&s)); drop(s);
        let s = lib.style(); s.set_size(10.0).set_color(120, 120, 120);
        canvas.draw_text("Page 1 of 3", 72.0, h - 40.0, Some(&s)); drop(s);
    }

    // ── Page 2: Content ────────────────────────────────────────────────────────
    {
        let canvas = doc.add_page(w, h);
        let s = lib.style(); s.set_size(18.0).set_bold();
        canvas.draw_text("Section 1 — Overview", 72.0, 60.0, Some(&s)); drop(s);
        canvas.draw_line(72.0, 80.0, w - 72.0, 80.0, 26, 86, 160, 1.5);
        let s = lib.style(); s.set_size(11.0);
        canvas.draw_textbox(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor \
             incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud \
             exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
            72.0, 96.0, w - 144.0, 80.0, Some(&s),
        ); drop(s);
        let s = lib.style(); s.set_size(14.0).set_bold();
        canvas.draw_text("Key Metrics", 72.0, 200.0, Some(&s)); drop(s);

        let metrics = [
            ("Revenue", "$4.2M"), ("Customers", "1,840"),
            ("New Products", "12"), ("Net Score", "72"),
        ];
        for (i, (name, value)) in metrics.iter().enumerate() {
            let col = (i % 2) as f32; let row = (i / 2) as f32;
            let (bx, by) = (72.0 + col * 230.0, 220.0 + row * 80.0);
            canvas.draw_rect(bx, by, 210.0, 65.0, Some((240, 245, 252)), Some((200, 210, 230)), 0.5);
            let s = lib.style(); s.set_size(10.0).set_color(80, 80, 80);
            canvas.draw_text(name, bx + 10.0, by + 14.0, Some(&s)); drop(s);
            let s = lib.style(); s.set_size(20.0).set_bold().set_color(26, 86, 160);
            canvas.draw_text(value, bx + 10.0, by + 40.0, Some(&s)); drop(s);
        }
        let s = lib.style(); s.set_size(10.0).set_color(120, 120, 120);
        canvas.draw_text("Page 2 of 3", 72.0, h - 40.0, Some(&s)); drop(s);
    }

    // ── Page 3: Summary table ──────────────────────────────────────────────────
    {
        let canvas = doc.add_page(w, h);
        let s = lib.style(); s.set_size(18.0).set_bold();
        canvas.draw_text("Section 2 — Regional Summary", 72.0, 60.0, Some(&s)); drop(s);
        canvas.draw_line(72.0, 80.0, w - 72.0, 80.0, 26, 86, 160, 1.5);

        let table = lib.table(&[160.0, 90.0, 90.0, 90.0, 90.0]);
        table.set_header_bg(26, 86, 160);
        table.set_alternate_bg(240, 245, 252);
        table.set_border(200, 200, 200, 0.5);
        table.set_cell_padding(5.0);
        table.add_row(&["Region", "Q1", "Q2", "Q3", "Q4"]);
        table.add_row(&["North America", "$1.1M", "$1.0M", "$1.2M", "$1.4M"]);
        table.add_row(&["Europe",        "$0.7M", "$0.8M", "$0.9M", "$0.8M"]);
        table.add_row(&["Asia Pacific",  "$0.3M", "$0.4M", "$0.4M", "$0.5M"]);
        table.add_row(&["Other",         "$0.1M", "$0.1M", "$0.1M", "$0.1M"]);
        table.add_row(&["Total",         "$2.2M", "$2.3M", "$2.6M", "$2.8M"]);
        canvas.draw_table(&table, 72.0, 96.0);

        let s = lib.style(); s.set_size(10.0).set_color(120, 120, 120);
        canvas.draw_text("Page 3 of 3", 72.0, h - 40.0, Some(&s)); drop(s);
    }

    let out = out_dir.join("example_05_multipage.pdf");
    doc.save(out.to_str().unwrap()).unwrap();
    println!("Written to {}", out.display());
}
