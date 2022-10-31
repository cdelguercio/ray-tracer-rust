use crate::color;

#[derive(Clone, Copy)]
pub struct Pixel {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

impl Pixel {
    pub fn new(red: u16, green: u16, blue: u16) -> Self {
        Pixel {
            red,
            green,
            blue,
        }
    }

    pub fn from_color(color: &color::Color) -> Self {
        Pixel {
            red: (color.red * 255.0) as u16,
            green: (color.green * 255.0) as u16,
            blue: (color.blue * 255.0) as u16,
        }
    }
}
