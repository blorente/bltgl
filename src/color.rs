use crossterm::style::Color;

#[derive(Clone, Copy)]
pub struct ColorRGBA(u8, u8, u8, u8);
impl Into<[u8; 4]> for ColorRGBA {
    fn into(self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }
}
impl Into<crossterm::style::Color> for ColorRGBA {
    fn into(self) -> crossterm::style::Color {
        Color::Rgb {
            r: self.0,
            g: self.1,
            b: self.2,
        }
    }
}

#[rustfmt::skip]
impl ColorRGBA {
 pub fn red() -> ColorRGBA { ColorRGBA(255, 0, 0, 0) }
 pub fn green() -> ColorRGBA { ColorRGBA(0, 255, 0, 0) }
 pub fn blue() -> ColorRGBA { ColorRGBA(0, 0, 255, 0) }
 pub fn black() -> ColorRGBA { ColorRGBA(0, 0, 0, 0) }
 pub fn white() -> ColorRGBA { ColorRGBA(255, 255, 255, 0) }
}
