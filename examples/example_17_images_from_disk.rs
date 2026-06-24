use majorsilence_pdf::PdfLibrary;
use std::{env, fs, path::Path};

fn synthetic_rgb(pw: u32, ph: u32, r_base: u8, g_base: u8, b_base: u8) -> Vec<u8> {
    let mut v = Vec::with_capacity((pw * ph * 3) as usize);
    for py in 0..ph {
        for px in 0..pw {
            v.push(((r_base as u32 + px * 2).min(255)) as u8);
            v.push(((g_base as u32 + py * 2).min(255)) as u8);
            v.push(b_base);
        }
    }
    v
}

fn main() {
    let lib_path   = env::var("PDFNATIVE_LIB").expect("Set PDFNATIVE_LIB");
    let image_dir  = env::var("IMAGE_DIR").unwrap_or_default();
    let image_path = env::var("IMAGE_PATH").unwrap_or_default();
    let out_dir    = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/output");
    fs::create_dir_all(&out_dir).unwrap();

    let lib    = PdfLibrary::load(&lib_path).expect("load library");
    let (w, h) = (595.28_f32, 841.89_f32);
    let margin = 50.0_f32;

    let jpeg_paths: Vec<String> = if !image_path.is_empty() && Path::new(&image_path).exists() {
        vec![image_path.clone()]
    } else if !image_dir.is_empty() {
        let mut paths = vec![];
        if let Ok(entries) = fs::read_dir(&image_dir) {
            for e in entries.flatten() {
                let p = e.path();
                if let Some(ext) = p.extension().and_then(|x| x.to_str()) {
                    if ext.eq_ignore_ascii_case("jpg") || ext.eq_ignore_ascii_case("jpeg") {
                        paths.push(p.to_string_lossy().into_owned());
                    }
                }
            }
        }
        paths.sort(); paths.truncate(6); paths
    } else {
        vec![]
    };

    let mut doc = lib.document();
    doc.set_title("Images from Disk");
    let canvas = doc.add_page(w, h);

    let heading = lib.style(); heading.set_size(18.0).set_bold();
    canvas.draw_text("Images from Disk", margin, 40.0, Some(&heading)); drop(heading);

    let cap_s = lib.style(); cap_s.set_size(8.0).set_color(80, 80, 80);
    let mut y = 68.0_f32;

    if !jpeg_paths.is_empty() {
        let info = lib.style(); info.set_size(9.0).set_color(80, 80, 80);
        canvas.draw_text(&format!("Embedding {} JPEG(s)", jpeg_paths.len()), margin, y, Some(&info));
        drop(info); y += 14.0;

        let (thumb_w, thumb_h) = (140.0_f32, 105.0_f32);
        let cols = 3_usize;
        for (i, path) in jpeg_paths.iter().enumerate() {
            let col = (i % cols) as f32; let row = (i / cols) as f32;
            let bx  = margin + col * (thumb_w + 8.0);
            let by  = y + row * (thumb_h + 24.0);
            let data = fs::read(path).unwrap();
            canvas.draw_image(&data, 0, 0, bx, by, thumb_w, thumb_h, true);
            let name = Path::new(path).file_name().unwrap().to_string_lossy();
            canvas.draw_text(&name, bx, by + thumb_h + 4.0, Some(&cap_s));
        }
    } else {
        let info = lib.style(); info.set_size(10.0).set_color(160, 80, 0);
        canvas.draw_text("IMAGE_DIR or IMAGE_PATH not set. Using synthetic images.", margin, y, Some(&info));
        drop(info); y += 18.0;

        let (thumb_w, thumb_h) = (100_u32, 60_u32);
        let synthetics: &[(&str, u8, u8, u8)] = &[
            ("Red-gradient",    200, 80,  0),
            ("Blue-gradient",   0,   80,  200),
            ("Green-gradient",  0,   160, 80),
            ("Purple-gradient", 120, 0,   160),
        ];
        for (i, &(label, r, g, b)) in synthetics.iter().enumerate() {
            let data = synthetic_rgb(thumb_w, thumb_h, r, g, b);
            let bx   = margin + i as f32 * (thumb_w as f32 + 10.0);
            canvas.draw_image(&data, thumb_w, thumb_h, bx, y, thumb_w as f32, thumb_h as f32, false);
            canvas.draw_text(label, bx, y + thumb_h as f32 + 4.0, Some(&cap_s));
        }
        y += thumb_h as f32 + 24.0;

        let note = lib.style(); note.set_size(9.0).set_color(130, 130, 130);
        canvas.draw_textbox(
            "Set IMAGE_DIR=/path/to/photos to embed real JPEG images, \
             or IMAGE_PATH=/path/to/photo.jpg for a single image.",
            margin, y, w - 2.0 * margin, 50.0, Some(&note),
        ); drop(note);
    }
    drop(cap_s);

    drop(canvas);
    let out = out_dir.join("example_17_images_from_disk.pdf");
    doc.save(out.to_str().unwrap()).unwrap();
    println!("Written to {}", out.display());
}
