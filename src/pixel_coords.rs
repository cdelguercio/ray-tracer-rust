pub struct PixelCoords {
    pub x: usize,
    pub y: usize,
}

impl PixelCoords {
    pub fn new(x: usize, y: usize) -> Self {
        PixelCoords { x, y }
    }
}
