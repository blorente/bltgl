use std::{cmp::min, vec};

use crossterm::event::KeyCode;

use crate::{camera::Renderable, color::ColorRGBA};

pub struct Prompt {
    prompt: String,
    command: String,
    command_buffer: Vec<String>,
    command_buffer_idx: usize,
}

impl Prompt {
    pub fn new(prompt: &str) -> Self {
        Self {
            prompt: prompt.to_string(),
            command: String::new(),
            command_buffer: vec![],
            command_buffer_idx: 0,
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Backspace => {
                self.command.pop();
            }
            KeyCode::Enter => todo!(),
            KeyCode::Left => todo!(),
            KeyCode::Right => todo!(),
            KeyCode::Up => todo!(),
            KeyCode::Down => todo!(),
            KeyCode::Home => todo!(),
            KeyCode::End => todo!(),
            KeyCode::Tab => todo!(),
            KeyCode::Delete => todo!(),
            KeyCode::Insert => todo!(),
            KeyCode::Char(ch) => {
                self.command.push(ch);
            }
            KeyCode::Null => todo!(),
            KeyCode::Esc => todo!(),
            _ => todo!(),
        }
    }

    pub fn flush_command(&mut self) -> String {
        let command = self.command.clone();
        self.command_buffer.push(command.clone());
        self.command.clear();
        command
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
