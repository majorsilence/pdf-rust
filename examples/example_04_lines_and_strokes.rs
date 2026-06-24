use majorsilence_pdf::PdfLibrary;
use std::{env, fs, path::Path};

fn main() {
    let lib_path = env::var("PDFNATIVE_LIB").expect("Set PDFNATIVE_LIB");
    let out_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/output");
    fs::create_dir_all(&out_dir).unwrap();

    let lib    = PdfLibrary::load(&lib_path).expect("load library");
    let (w, h) = (595.28_f32, 841.89_f32);

    let mut doc = lib.document();
    doc.set_title("Lines and Strokes");
    let canvas = doc.add_page(w, h);

    let heading = lib.style();
    heading.set_size(18.0).set_bold();
    canvas.draw_text("Lines and Strokes", 72.0, 40.0, Some(&heading));
    drop(heading);

    let label = lib.style();
    label.set_size(10.0);

    let mut y = 80.0_f32;

    canvas.draw_text("Line widths (0.5 → 6 pt)", 72.0, y, Some(&label));
    y += 16.0;
    for &lw in &[0.5_f32, 1.0, 1.5, 2.0, 3.0, 4.0, 6.0] {
        canvas.draw_line(72.0, y, 420.0, y, 0, 0, 0, lw);
        canvas.draw_text(&format!("{} pt", lw), 430.0, y - 4.0, Some(&label));
        y += 18.0;
    }
    y += 12.0;

    canvas.draw_text("Coloured lines (2 pt)", 72.0, y, Some(&label));
    y += 16.0;
    for &(name, r, g, b) in &[
        ("Black",  0u8,   0u8,   0u8  ),
        ("Red",    220,   0,     0    ),
        ("Blue",   0,     0,     200  ),
        ("Green",  0,     160,   0    ),
        ("Orange", 220,   120,   0    ),
        ("Purple", 130,   0,     180  ),
    ] {
        canvas.draw_line(72.0, y, 300.0, y, r, g, b, 2.0);
        canvas.draw_text(name, 310.0, y - 4.0, Some(&label));
        y += 18.0;
    }
    y += 12.0;

    canvas.draw_text("Diagonal lines", 72.0, y, Some(&label));
    y += 16.0;
    canvas.draw_line(72.0, y,        300.0, y + 80.0, 0,   0,   0,   1.0);
    canvas.draw_line(300.0, y,       72.0,  y + 80.0, 0,   0,   0,   1.0);
    canvas.draw_line(72.0, y + 40.0, 300.0, y + 40.0, 180, 180, 180, 0.5);
    y += 100.0;

    canvas.draw_text("Rectangle drawn from four lines", 72.0, y, Some(&label));
    y += 16.0;
    let (x0, x1, y1) = (72.0_f32, 300.0_f32, y + 60.0);
    for &(ax, ay, bx, by) in &[
        (x0, y, x1, y), (x1, y, x1, y1), (x1, y1, x0, y1), (x0, y1, x0, y),
    ] {
        canvas.draw_line(ax, ay, bx, by, 60, 60, 60, 2.0);
    }
    canvas.draw_text("Border from 4 draw_line calls", x0 + 10.0, y + 26.0, Some(&label));
    y += 80.0;

    canvas.draw_text("Heavy rule separator", 72.0, y, Some(&label));
    y += 12.0;
    canvas.draw_line(72.0, y, w - 72.0, y, 26, 86, 160, 3.0);
    y += 8.0;
    canvas.draw_line(72.0, y, w - 72.0, y, 26, 86, 160, 0.5);

    drop(label);
    drop(canvas);
    let out = out_dir.join("example_04_lines_and_strokes.pdf");
    doc.save(out.to_str().unwrap()).unwrap();
    println!("Written to {}", out.display());
}
