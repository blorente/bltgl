use crate::{
    camera::{Glyph, Renderable},
    color::ColorRGBA,
};

pub struct Status {
    content: String,
    color: ColorRGBA,
}

impl Status {
    pub fn new() -> Self {
        Self {
            content: "Hello, I'm a status line!".to_string(),
            color: ColorRGBA::grey(),
        }
    }
}

impl Renderable for Status {
    fn render(&self, camera: &mut crate::camera::Camera) {
        let starty = camera.height - 2;

        let mut lastx = 0;
        for (x, ch) in self.content.chars().enumerate() {
            let x = x as u16;
            if x > camera.width {
                break;
            }
            camera.buffer[((starty * camera.width) + x) as usize] = Glyph {
                fg: ColorRGBA::black(),
                bg: self.color,
                ch,
            };
            lastx = x;
        }
        if lastx < camera.width {
            for x in lastx..camera.width {
                camera.buffer[((starty * camera.width) + x) as usize] = Glyph {
                    fg: ColorRGBA::black(),
                    bg: self.color,
                    ch: ' ',
                };
            }
        }
    }
}
