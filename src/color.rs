#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new() -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    pub fn make(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}
