use std::path;
use std::fs;
use std::io;

use png;

use crate::image;

pub struct PngWriter<'a> {
    encoder: png::Encoder<'a, io::BufWriter<fs::File>>,
}

impl PngWriter<'static> {
    pub fn new(width: u32, height: u32, file: &path::Path) -> Self {
        let file = fs::File::create(file).unwrap();
        let mut writer = io::BufWriter::new(file);
        let mut encoder = png::Encoder::new(writer, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        PngWriter {
            encoder,
        }
    }

    pub fn write(self, data: &image::Image) {
        let mut writer = self.encoder.write_header().unwrap();
        writer.write_image_data(&data.unpack()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::pixel;

    #[test]
    fn test_write() {
        let mut image = image::Image::new(2, 2);
        image.set_pixel(0, 0, pixel::Pixel::new(255, 0, 0));
        let writer = PngWriter::new(2, 2, &path::Path::new("test.png"));
        writer.write(&image);
    }
}
