use crate::{
    camera::{Camera, Renderable},
    color::ColorRGBA,
    components::{prompt::Prompt, quad::Quad, status::Status, textbox::TextBox},
};
use crossterm::event::KeyCode;
use eyre::Result;

enum AppMode {
    Normal,
    Command,
}

pub struct App {
    // TODO make all of these private when the world actually works
    pub quads: Vec<Quad>,
    pub textboxes: Vec<TextBox>,
    pub index: usize,
    pub status: Status,
    pub prompt: Prompt,

    mode: AppMode,
}

impl App {
    pub fn new(textboxes: Vec<TextBox>, index: usize, status: Status, prompt: Prompt) -> Self {
        Self {
            quads: vec![],
            textboxes,
            index,
            status,
            prompt,
            mode: AppMode::Normal,
        }
    }

    pub fn handle_key(&mut self, key: KeyCode, camera: &mut Camera) -> Result<bool> {
        match self.mode {
            AppMode::Normal => self.handle_key_normal(key, camera),
            AppMode::Command => self.handle_key_command(key, camera),
        }
    }

    fn handle_key_command(&mut self, key: KeyCode, camera: &mut Camera) -> Result<bool> {
        match key {
            KeyCode::Backspace => self.prompt.handle_key(key),
            KeyCode::Enter => {
                let text = self.prompt.flush_command();
                self.textboxes.push(TextBox::new(camera.focus, 0, &text));
                self.mode = AppMode::Normal;
            }
            KeyCode::Left => self.prompt.handle_key(key),
            KeyCode::Right => self.prompt.handle_key(key),
            KeyCode::Up => self.prompt.handle_key(key),
            KeyCode::Down => self.prompt.handle_key(key),
            KeyCode::Home => self.prompt.handle_key(key),
            KeyCode::End => self.prompt.handle_key(key),
            KeyCode::Tab => self.prompt.handle_key(key),
            KeyCode::Char(_) => self.prompt.handle_key(key),
            KeyCode::Esc => self.mode = AppMode::Normal,
            _ => todo!(),
        }
        Ok(false)
    }
    fn handle_key_normal(&mut self, key: KeyCode, camera: &mut Camera) -> Result<bool> {
        match key {
            KeyCode::Backspace => todo!(),
            KeyCode::Enter => todo!(),
            KeyCode::Left => todo!(),
            KeyCode::Right => todo!(),
            KeyCode::Up => todo!(),
            KeyCode::Down => todo!(),
            KeyCode::Home => todo!(),
            KeyCode::End => todo!(),
            KeyCode::PageUp => todo!(),
            KeyCode::PageDown => todo!(),
            KeyCode::Tab => todo!(),
            KeyCode::BackTab => todo!(),
            KeyCode::Delete => todo!(),
            KeyCode::Insert => todo!(),
            KeyCode::F(_) => todo!(),
            KeyCode::Char(input) => {
                if input == 'q' {
                    return Ok(true);
                }
                if input == 'h' {
                    camera.move_left()
                }
                if input == 'j' {
                    camera.move_down()
                }
                if input == 'k' {
                    camera.move_up()
                }
                if input == 'l' {
                    camera.move_right()
                }
                if input == 'n' {
                    self.index = (self.index + 1) % self.textboxes.len();
                    camera.focus_on(self.textboxes[self.index].center(&camera));
                }
                if input == ';' {
                    self.mode = AppMode::Command;
                }
            }
            KeyCode::Null => todo!(),
            KeyCode::Esc => todo!(),
        }
        Ok(false)
    }
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
