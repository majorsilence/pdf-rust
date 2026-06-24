use majorsilence_pdf::PdfLibrary;
use std::{env, fs, path::Path};

fn main() {
    let lib_path = env::var("PDFNATIVE_LIB").expect("Set PDFNATIVE_LIB");
    let out_dir  = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/output");
    fs::create_dir_all(&out_dir).unwrap();

    let lib    = PdfLibrary::load(&lib_path).expect("load library");
    let (w, h) = (595.28_f32, 841.89_f32);
    let margin = 40.0_f32;

    let mut doc = lib.document();
    doc.set_title("Q4 2025 Sales Dashboard");
    let canvas = doc.add_page(w, h);

    // Title bar
    canvas.draw_rect(0.0, 0.0, w, 52.0, Some((30, 30, 50)), None, 0.0);
    let s = lib.style(); s.set_size(18.0).set_bold().set_color(255, 255, 255);
    canvas.draw_text("Q4 2025  ·  Sales Dashboard", margin, 16.0, Some(&s)); drop(s);
    let s = lib.style(); s.set_size(9.0).set_color(160, 180, 220);
    canvas.draw_text("Generated 2025-12-31", margin, 38.0, Some(&s)); drop(s);

    // KPI tiles
    let kpis: &[(&str, &str, &str, (u8, u8, u8))] = &[
        ("Total Revenue", "$4.2M",  "+12%", (26, 86, 160)),
        ("New Customers", "1,840",  "+8%",  (0, 140, 80)),
        ("Avg Order",     "$2,283", "+5%",  (180, 80, 0)),
        ("NPS Score",     "72",     "+4pt", (120, 0, 160)),
    ];
    let (tile_w, tile_h) = (110.0_f32, 75.0_f32);
    for (i, &(title, value, delta, (r, g, b))) in kpis.iter().enumerate() {
        let col = (i % 2) as f32; let row = (i / 2) as f32;
        let (bx, by) = (margin + col * (tile_w + 8.0), 62.0 + row * (tile_h + 8.0));
        canvas.draw_rect(bx, by, tile_w, tile_h, Some((r, g, b)), None, 0.0);
        let s = lib.style(); s.set_size(8.0).set_color(200, 220, 255);
        canvas.draw_text(title, bx + 6.0, by + 10.0, Some(&s)); drop(s);
        let s = lib.style(); s.set_size(20.0).set_bold().set_color(255, 255, 255);
        canvas.draw_text(value, bx + 6.0, by + 34.0, Some(&s)); drop(s);
        let s = lib.style(); s.set_size(9.0).set_color(200, 255, 200);
        canvas.draw_text(delta, bx + 6.0, by + 58.0, Some(&s)); drop(s);
    }

    // Regional table
    let table_x = margin + 2.0 * (tile_w + 8.0) + 16.0;
    let s = lib.style(); s.set_size(11.0).set_bold();
    canvas.draw_text("Regional Breakdown", table_x, 64.0, Some(&s)); drop(s);

    let table = lib.table(&[110.0, 60.0, 60.0, 50.0]);
    table.set_header_bg(30, 30, 50);
    table.set_alternate_bg(245, 245, 250);
    table.set_border(210, 210, 210, 0.4);
    table.set_cell_padding(4.0);
    table.add_row(&["Region",        "Revenue", "Units", "Chg"]);
    table.add_row(&["North America", "$1.7M",   "612",   "+14%"]);
    table.add_row(&["Europe",        "$1.2M",   "441",   "+9%"]);
    table.add_row(&["Asia Pacific",  "$0.9M",   "320",   "+18%"]);
    table.add_row(&["LATAM",         "$0.3M",   "110",   "+6%"]);
    table.add_row(&["Other",         "$0.1M",   "40",    "+2%"]);
    canvas.draw_table(&table, table_x, 78.0);
    drop(table);

    // Bar chart
    let mut chart_top = 230.0_f32;
    let s = lib.style(); s.set_size(11.0).set_bold();
    canvas.draw_text("Quarterly Revenue", margin, chart_top, Some(&s)); drop(s);
    chart_top += 16.0;
    let chart_h   = 120.0_f32; let chart_bot = chart_top + chart_h;
    let bar_w     = 40.0_f32;  let gap       = 20.0_f32;
    let revenues  = [2.2_f32, 2.8, 3.5, 4.2];
    let quarters  = ["Q1", "Q2", "Q3", "Q4"];
    let max_rev   = revenues.iter().cloned().fold(0.0_f32, f32::max);
    canvas.draw_line(margin, chart_top, margin, chart_bot, 150, 150, 150, 0.5);
    canvas.draw_line(margin, chart_bot, margin + revenues.len() as f32 * (bar_w + gap) + gap, chart_bot, 150, 150, 150, 0.5);
    let lbl = lib.style(); lbl.set_size(9.0).set_color(80, 80, 80);
    for (i, &rev) in revenues.iter().enumerate() {
        let bx = margin + gap + i as f32 * (bar_w + gap);
        let bh = chart_h * rev / max_rev;
        let by = chart_bot - bh;
        canvas.draw_rect(bx, by, bar_w, bh, Some((26, 86, 160)), None, 0.0);
        canvas.draw_text(&format!("${:.1}M", rev), bx + 2.0, by - 12.0, Some(&lbl));
        canvas.draw_text(quarters[i], bx + 12.0, chart_bot + 6.0, Some(&lbl));
    }
    drop(lbl);

    // Product mix
    let mut mix_y = chart_bot + 40.0;
    let s = lib.style(); s.set_size(11.0).set_bold();
    canvas.draw_text("Product Mix (% of Revenue)", margin, mix_y, Some(&s)); drop(s);
    mix_y += 14.0;
    let products: &[(&str, f32, (u8, u8, u8))] = &[
        ("PDF Library",   42.0, (26, 86, 160)),
        ("Report Engine", 28.0, (0, 140, 80)),
        ("Integration",   18.0, (220, 120, 0)),
        ("Support",       12.0, (160, 0, 80)),
    ];
    let bar_total_w = w - 2.0 * margin;
    let mut x_cur   = margin;
    let ws = lib.style(); ws.set_size(8.0).set_color(255, 255, 255);
    for &(_, pct, color) in products {
        let seg_w = bar_total_w * pct / 100.0;
        canvas.draw_rect(x_cur, mix_y, seg_w, 22.0, Some(color), None, 0.0);
        if seg_w > 30.0 {
            canvas.draw_text(&format!("{}%", pct as u32), x_cur + 4.0, mix_y + 7.0, Some(&ws));
        }
        x_cur += seg_w;
    }
    drop(ws);
    mix_y += 30.0;
    let ls = lib.style(); ls.set_size(9.0);
    for (i, &(name, pct, color)) in products.iter().enumerate() {
        let lx = margin + i as f32 * 115.0;
        canvas.draw_rect(lx, mix_y, 10.0, 10.0, Some(color), None, 0.0);
        canvas.draw_text(&format!("{} ({}%)", name, pct as u32), lx + 14.0, mix_y + 2.0, Some(&ls));
    }
    drop(ls);

    drop(canvas);
    let out = out_dir.join("example_10_dashboard.pdf");
    doc.save(out.to_str().unwrap()).unwrap();
    println!("Written to {}", out.display());
}
