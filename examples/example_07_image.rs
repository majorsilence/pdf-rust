use majorsilence_pdf::PdfLibrary;
use std::{env, fs, path::Path};

fn make_gradient(pw: u32, ph: u32) -> Vec<u8> {
    let mut v = Vec::with_capacity((pw * ph * 3) as usize);
    for py in 0..ph {
        for px in 0..pw {
            v.push((255 * px / pw) as u8);
            v.push((255 * py / ph) as u8);
            v.push(180);
        }
    }
    v
}

fn make_checkerboard(pw: u32, ph: u32, cell: u32) -> Vec<u8> {
    let mut v = Vec::with_capacity((pw * ph * 3) as usize);
    for py in 0..ph {
        for px in 0..pw {
            let val: u8 = if ((px / cell) + (py / cell)) % 2 == 0 { 255 } else { 60 };
            v.extend_from_slice(&[val, val, val]);
        }
    }
    v
}

fn main() {
    let lib_path  = env::var("PDFNATIVE_LIB").expect("Set PDFNATIVE_LIB");
    let jpeg_path = env::var("JPEG_PATH").unwrap_or_default();
    let out_dir   = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/output");
    fs::create_dir_all(&out_dir).unwrap();

    let lib    = PdfLibrary::load(&lib_path).expect("load library");
    let (w, h) = (595.28_f32, 841.89_f32);

    let mut doc = lib.document();
    doc.set_title("Image Embedding");
    let canvas = doc.add_page(w, h);

    let heading = lib.style(); heading.set_size(18.0).set_bold();
    canvas.draw_text("Image Embedding", 72.0, 40.0, Some(&heading)); drop(heading);

    let label = lib.style(); label.set_size(10.0);
    let mut y = 70.0_f32;

    // Gradient
    canvas.draw_text("Synthetic gradient (raw RGB24, 300×150 pixels):", 72.0, y, Some(&label)); y += 14.0;
    let grad = make_gradient(300, 150);
    canvas.draw_image(&grad, 300, 150, 72.0, y, 300.0, 150.0, false); y += 165.0;

    // Checkerboard
    canvas.draw_text("Checkerboard pattern (raw RGB24, 200×100):", 72.0, y, Some(&label)); y += 14.0;
    let checker = make_checkerboard(200, 100, 20);
    canvas.draw_image(&checker, 200, 100, 72.0, y, 200.0, 100.0, false); y += 115.0;

    // Scaled
    canvas.draw_text("Same gradient at different scales:", 72.0, y, Some(&label)); y += 14.0;
    let mut x_pos = 72.0_f32;
    for &(dw, dh) in &[(80.0_f32, 40.0_f32), (120.0, 60.0), (160.0, 80.0)] {
        canvas.draw_image(&grad, 300, 150, x_pos, y, dw, dh, false);
        canvas.draw_text(&format!("{}×{} pts", dw as u32, dh as u32), x_pos, y + dh + 2.0, Some(&label));
        x_pos += dw + 10.0;
    }
    y += 100.0;

    // JPEG from disk
    let jpeg_exists = !jpeg_path.is_empty() && Path::new(&jpeg_path).exists();
    if jpeg_exists {
        let name = Path::new(&jpeg_path).file_name().unwrap().to_string_lossy();
        canvas.draw_text(&format!("JPEG from disk: {}", name), 72.0, y, Some(&label)); y += 14.0;
        let jpeg_bytes = fs::read(&jpeg_path).unwrap();
        canvas.draw_image(&jpeg_bytes, 0, 0, 72.0, y, 200.0, 150.0, true);
    } else {
        canvas.draw_text("Set JPEG_PATH=/path/to/photo.jpg to embed a JPEG from disk.", 72.0, y, Some(&label));
    }

    drop(label);
    drop(canvas);
    let out = out_dir.join("example_07_image.pdf");
    doc.save(out.to_str().unwrap()).unwrap();
    println!("Written to {}", out.display());
}
