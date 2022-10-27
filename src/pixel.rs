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
}
