use crate::{
    camera::{Camera, Glyph, Renderable},
    color::ColorRGBA,
};

#[derive(Clone)]
pub struct Quad {
    pub pos: [i32; 2],
    pub width: u16,
    pub height: u16,
    pub color: ColorRGBA,
    pub ch: char,
}

impl Quad {
    pub fn new(pos: [i32; 2], width: u16, height: u16, color: ColorRGBA) -> Self {
        Self {
            pos,
            width,
            height,
            color,
            ch: ' ',
        }
    }
}

impl Renderable for Quad {
    fn render(&self, camera: &mut Camera) {
        let [[startx, starty], [endx, endy]] =
            camera.project([self.pos[0], self.pos[1]], [self.width, self.height]);
        for x in startx..endx {
            for y in starty..endy {
                camera.buffer[(y * camera.width + x) as usize] = Glyph {
                    fg: ColorRGBA::black().into(),
                    bg: self.color.into(),
                    ch: self.ch,
                }
            }
        }
    }
    fn center(&self, camera: &Camera) -> [i32; 2] {
        [
            (self.pos[0] + self.width as i32 / 2),
            (self.pos[1] + self.height as i32 / 2),
        ]
    }
}
