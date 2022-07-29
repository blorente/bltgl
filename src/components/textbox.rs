use crate::{
    camera::{Camera, Renderable},
    color::ColorRGBA,
};

use super::quad::Quad;

#[derive(Clone)]
pub struct TextBox {
    max_width: u16,
    text: String,
    pos: [i32; 2],
    quad: Quad,
    color: ColorRGBA,
}

impl TextBox {
    pub fn new(pos: [i32; 2], max_width: u16, text: &str) -> Self {
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
    fn calculate_quad(pos: [i32; 2], max_width: u16, text: &String, color: ColorRGBA) -> Quad {
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
        if starty >= camera.height {
            return;
        }
        if startx == endx {
            return;
        }

        // Indices to hold where the string starts and ends
        let (mut string_start, mut string_end) = (0, self.text.len());
        if startx == 0 && endx < self.text.len() as u16 {
            let offset = self.text.len() - (endx - startx) as usize;
            string_start = offset;
        } else if endx >= camera.width && (endx - startx) < self.text.len() as u16 {
            string_end = (endx - startx) as usize;
        }

        let _ = camera.write_text(
            [startx, starty],
            [string_start, string_end],
            &self.text,
            self.color,
            ColorRGBA::black(),
        );
    }
    fn center(&self, camera: &Camera) -> [i32; 2] {
        self.quad.center(camera)
    }
}
