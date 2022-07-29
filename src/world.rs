use crate::{
    camera::{Camera, Renderable},
    color::ColorRGBA,
    components::{prompt::Prompt, quad::Quad, status::Status, textbox::TextBox},
};

pub struct App {
    // TODO make all of these private when the world actually works
    pub quads: Vec<Quad>,
    pub textboxes: Vec<TextBox>,
    pub index: usize,
    pub status: Status,
    pub prompt: Prompt,
}

impl Renderable for App {
    fn render(&self, camera: &mut Camera) {
        for ele in self.quads.iter() {
            ele.render(camera);
        }
        for (i, ele) in self.textboxes.iter().enumerate() {
            if i == self.index {
                let mut marked: TextBox = ele.clone();
                marked.set_color(ColorRGBA::green());
                marked.render(camera);
                marked.set_color(ColorRGBA::white());
            } else {
                ele.render(camera);
            }
        }
        self.status.render(camera);
        self.prompt.render(camera);
    }
}
