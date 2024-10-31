use image::ImageFormat;

use super::Error;

pub fn guess_image_fmt(buf: &[u8]) -> Result<ImageFormat, Error> {
    if buf.starts_with(b"\x52\x49\x46\x46") {
        // RIFF for WebP
        return Ok(ImageFormat::WebP);
    }

    if buf.starts_with(b"\x89PNG\r\n\x1a\n") {
        // PNG
        return Ok(ImageFormat::Png);
    }

    if buf.starts_with(b"\xff\xd8") {
        // JPEG
        return Ok(ImageFormat::Jpeg);
    }

    Err(Box::from(String::from(
        "Unsupported or unknown input format.",
    )))
}
