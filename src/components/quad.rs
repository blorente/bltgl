use std::cmp::{max, min};

use crate::{
    camera::{Camera, Glyph, Renderable},
    color::ColorRGBA,
};

#[derive(Clone)]
pub struct Quad {
    pub pos: [u16; 2],
    pub width: u16,
    pub height: u16,
    pub color: ColorRGBA,
    pub ch: char,
}

impl Quad {
    pub fn new(pos: [u16; 2], width: u16, height: u16, color: ColorRGBA) -> Self {
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
        let [pointx, pointy] = camera.world_to_camera(self.pos[0], self.pos[1]);

        let [endx, endy]: [u16; 2] = [
            min(max(pointx + self.width as i32, 0) as u16, camera.width),
            min(max(pointy + self.height as i32, 0) as u16, camera.height),
        ];
        let [startx, starty] = [max(pointx, 0) as u16, max(pointy, 0) as u16];
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
}
