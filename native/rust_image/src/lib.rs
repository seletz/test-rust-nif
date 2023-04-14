use image::io::Reader as ImageReader;
use image::codecs::jpeg::JpegEncoder;
use std::fs::File;


#[rustler::nif]
fn jpg(input: String, output: String, quality: i64) -> Result<String, String> {
    let Ok(image_file) = ImageReader::open(&input) else {
        panic!("Can't open file: '{input}'");
    };

    let Ok(image) = image_file.decode() else {
        panic!("Can't decode file: '{input}'");
    };

    let Ok(out_file) = File::create(&output) else {
        panic!("Can't create file: '{output}'");
    };

    let mut jpg = JpegEncoder::new_with_quality(&out_file, quality as u8);

    jpg.encode_image(&image).expect("Error encoding image.");

    Ok(output)
}

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b + 1
}

rustler::init!("Elixir.TestRustNif.RustImage", [add, jpg]);
