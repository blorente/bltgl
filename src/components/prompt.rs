use std::cmp::min;

use crate::{camera::Renderable, color::ColorRGBA};

pub struct Prompt {
    prompt: String,
    command: String,
}

impl Prompt {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            command: String::new(),
        }
    }
}

impl Renderable for Prompt {
    fn render(&self, camera: &mut crate::camera::Camera) {

        let len = self.prompt.len() + self.command.len();
        camera.write_text(
            [0, camera.height - 1],
            [0, min(len, camera.width as usize - 1)],
            &format!("{}{}", &self.prompt, &self.command),
            ColorRGBA::black(),
            ColorRGBA::lightgrey(),
        );
    }
}
