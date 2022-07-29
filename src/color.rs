use crossterm::style::Color;

#[derive(Clone, Copy, Debug)]
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

impl From<u32> for ColorRGBA {
    fn from(val: u32) -> Self {
        let r: u8 = ((val & 0xFF000000) >> 24)
            .try_into()
            .expect("Failed converting r");
        let g: u8 = ((val & 0x00FF0000) >> 16)
            .try_into()
            .expect("Failed converting g");
        let b: u8 = ((val & 0x0000FF00) >> 8)
            .try_into()
            .expect("Failed converting b");
        let a = (val & 0x000000FF).try_into().expect("Failed converting a");
        Self(r, g, b, a)
    }
}

#[rustfmt::skip]
impl ColorRGBA {
 pub fn red() -> ColorRGBA { ColorRGBA(255, 0, 0, 0) }
 pub fn green() -> ColorRGBA { ColorRGBA(0, 255, 0, 0) }
 pub fn blue() -> ColorRGBA { ColorRGBA(0, 0, 255, 0) }
 pub fn black() -> ColorRGBA { ColorRGBA(0, 0, 0, 0) }
 pub fn darkgrey() -> ColorRGBA { ColorRGBA::from(0x413F4200) }
 pub fn grey() -> ColorRGBA { ColorRGBA::from(0x7F848700) }
 pub fn lightgrey() -> ColorRGBA { ColorRGBA::from(0xEEEEEE00) }
 pub fn white() -> ColorRGBA { ColorRGBA(255, 255, 255, 0) }
}
