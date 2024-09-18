use image::DynamicImage;

use dataurl::DataUrl;
use std::io::Cursor;


// TODO: add error handling: pub fn convert_dataurl() -> Result<String, String>;
pub fn convert_dataurl(image: DynamicImage) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::WebP).unwrap();


    let mut dataurl = DataUrl::new();
    dataurl.set_data(&bytes);
    dataurl.set_media_type(Some("image/webp".to_string()));
    dataurl.set_is_base64_encoded(true);

    dataurl.to_string()





}