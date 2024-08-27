use base64_url;
use image::{DynamicImage, EncodableLayout};
use webp::Encoder;

// TODO: add error handling: pub fn convert_dataurl() -> Result<String, String>;
pub fn convert_dataurl(image: DynamicImage) -> String {
    // TODO: convert the image to webp using crate "webp":
    let encoder: Encoder = Encoder::from_image(&image).unwrap();

    let webp_image = encoder.encode(100f32);

    // TODO: convert webp to byte array [u8]
    let bytes_image = webp_image.as_bytes();
    println!("{:?}", &bytes_image);

    // TODO: encode array to Data URL
    base64_url::encode(bytes_image)
}
