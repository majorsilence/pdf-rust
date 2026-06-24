use majorsilence_pdf::{PdfLibrary, DECOR_UNDERLINE};
use std::{env, fs, path::Path};

fn main() {
    let lib_path = env::var("PDFNATIVE_LIB").expect("Set PDFNATIVE_LIB");
    let out_dir  = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/output");
    fs::create_dir_all(&out_dir).unwrap();

    let lib    = PdfLibrary::load(&lib_path).expect("load library");
    let (w, h) = (595.28_f32, 841.89_f32);

    let mut doc = lib.document();
    doc.set_title("Annotations");
    doc.set_subject("Hyperlink annotation demo");
    let canvas = doc.add_page(w, h);

    let heading = lib.style(); heading.set_size(18.0).set_bold();
    canvas.draw_text("Hyperlink Annotations", 72.0, 50.0, Some(&heading)); drop(heading);

    let body = lib.style(); body.set_size(11.0);
    canvas.draw_text(
        "Click the links below. The blue underlined text is overlaid with a URI annotation.",
        72.0, 80.0, Some(&body),
    ); drop(body);

    let links = [
        ("Majorsilence GitHub",          "https://github.com/majorsilence"),
        ("Majorsilence Reporting",        "https://github.com/majorsilence/Reporting"),
        ("PDF Specification (ISO 32000)", "https://pdfa.org/resource/pdf-specification-archive/"),
        ("Wikipedia — PDF",               "https://en.wikipedia.org/wiki/PDF"),
    ];

    let link_style = lib.style();
    link_style.set_size(13.0).set_color(26, 86, 160).set_decoration(DECOR_UNDERLINE);

    let mut y = 120.0_f32;
    for (text, uri) in &links {
        canvas.draw_text(text, 72.0, y, Some(&link_style));
        let approx_width = text.len() as f32 * 7.5;
        canvas.add_link(72.0, y - 13.0, approx_width, 18.0, uri);
        y += 28.0;
    }
    drop(link_style);

    y += 10.0;
    let note = lib.style(); note.set_size(10.0).set_color(100, 100, 100);
    canvas.draw_text("Links use pdf_canvas_add_link(canvas, x, y, width, height, uri).", 72.0, y, Some(&note));
    y += 16.0;
    canvas.draw_text("The annotation rectangle is placed over the rendered text.", 72.0, y, Some(&note));
    drop(note);

    drop(canvas);
    let out = out_dir.join("example_08_annotations.pdf");
    doc.save(out.to_str().unwrap()).unwrap();
    println!("Written to {}", out.display());
}
