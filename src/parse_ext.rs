use std::path::Path;

use super::Error;

use image::ImageFormat;

pub fn parse_img_ext(path: &Path) -> Result<ImageFormat, Error> {
    match path.extension().and_then(|x| x.to_str()) {
        Some("jpg") | Some("jpeg") => Ok(ImageFormat::Jpeg),
        Some("png") => Ok(ImageFormat::Png),
        Some("gif") => Ok(ImageFormat::Gif),
        _ => Err(Box::from(String::from("Unsupported output format."))),
    }
}
