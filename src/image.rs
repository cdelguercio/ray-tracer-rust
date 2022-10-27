use crate::pixel;

pub struct Image {
    m_width: usize,
    m_height: usize,
    m_pixels: Vec<pixel::Pixel>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::new();
        for _ in 0..width * height {
            pixels.push(pixel::Pixel::new(0, 0, 0));
        }

        Image {
            m_width: width,
            m_height: height,
            m_pixels: pixels,
        }
    }

    pub fn get_width(&self) -> usize {
        self.m_width
    }

    pub fn get_height(&self) -> usize {
        self.m_height
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> pixel::Pixel {
        if x >= self.m_width || y >= self.m_height {
            panic!("Cannot get pixel at ({}, {}), image is only {}x{}", x, y, self.m_width, self.m_height);
        }

        self.m_pixels[x + y * self.m_width]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: pixel::Pixel) {
        if x >= self.m_width || y >= self.m_height {
            panic!("Cannot set pixel at ({}, {}), image is only {}x{}", x, y, self.m_width, self.m_height);
        }

        self.m_pixels[x + y * self.m_width] = pixel;
    }

    pub fn unpack(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        for pixel in &self.m_pixels {
            buffer.push(pixel.red as u8);
            buffer.push(pixel.green as u8);
            buffer.push(pixel.blue as u8);
            buffer.push(255);
        }

        buffer
    }

    fn clear(&mut self) {
        for pixel in self.m_pixels.iter_mut() {
            pixel.red = 0;
            pixel.green = 0;
            pixel.blue = 0;
        }
    }
}
