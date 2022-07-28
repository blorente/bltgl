use crate::{
    camera::{self, Camera, Renderable},
    color::ColorRGBA,
};

use super::quad::Quad;

#[derive(Clone)]
pub struct TextBox {
    max_width: u16,
    text: String,
    pos: [u16; 2],
    quad: Quad,
    color: ColorRGBA,
}

impl TextBox {
    pub fn new(pos: [u16; 2], max_width: u16, text: &str) -> Self {
        let text = text.to_string();
        let quad = Self::calculate_quad(pos, max_width, &text, ColorRGBA::white());
        Self {
            max_width,
            text,
            quad,
            pos,
            color: ColorRGBA::white(),
        }
    }
    fn calculate_quad(pos: [u16; 2], max_width: u16, text: &String, color: ColorRGBA) -> Quad {
        // TODO We're ignoring max_width for now.
        let width = text.len() as u16 + 2;
        Quad::new(pos, width, 3, color)
    }

    pub fn set_color(&mut self, new: ColorRGBA) {
        self.color = new;
        self.quad.color = new;
    }
}

impl Renderable for TextBox {
    fn render(&self, camera: &mut Camera) {
        self.quad.render(camera);
        let [[startx, starty], [endx, _]] = camera.project(
            [self.pos[0] + 1, self.pos[1] + 1],
            [self.text.len() as u16, 1],
        );

        for (idx, ch) in self.text.chars().enumerate() {
            let truex = startx + idx as u16;
            if truex > endx {
                break;
            }
            camera.buffer[((starty * camera.width) + truex) as usize].ch = ch;
        }
    }
    fn center(&self, camera: &Camera) -> [i32; 2] {
        self.quad.center(camera)
    }
}
