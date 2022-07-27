use crate::color::ColorRGBA;
use crossterm::{cursor::MoveTo, execute, style::Stylize, Result};
use std::{
    cmp::{max, min},
    io::stdout,
};

pub trait Renderable {
    fn render(&self, camera: &mut Camera);
}
#[derive(Clone)]
pub struct Glyph {
    pub fg: ColorRGBA,
    pub bg: ColorRGBA,
    pub ch: char,
}

impl Glyph {
    pub fn empty() -> Self {
        Self {
            fg: ColorRGBA::black(),
            bg: ColorRGBA::black(),
            ch: ' ',
        }
    }
}

pub struct Camera {
    // TODO Make this private, maybe move the renderable impls to this crate.
    pub buffer: Vec<Glyph>,
    blackout_buffer: Vec<Glyph>,
    // TODO Move these to camera
    pub width: u16,
    pub height: u16,
    focus: [i32; 2],
}

impl Camera {
    pub fn new(screen_dimensions: [u16; 2]) -> Self {
        Self {
            buffer: vec![Glyph::empty(); (screen_dimensions[0] * screen_dimensions[1]) as usize],
            blackout_buffer: vec![
                Glyph::empty();
                (screen_dimensions[0] * screen_dimensions[1]) as usize
            ],
            width: screen_dimensions[0],
            height: screen_dimensions[1],
            focus: [
                (screen_dimensions[0] / 2).try_into().expect("TODO"),
                (screen_dimensions[1] / 2).try_into().expect("TODO"),
            ],
        }
    }

    pub fn move_left(&mut self) {
        self.focus[0] -= 1;
    }
    pub fn move_up(&mut self) {
        self.focus[1] -= 1;
    }
    pub fn move_right(&mut self) {
        self.focus[0] += 1;
    }
    pub fn move_down(&mut self) {
        self.focus[1] += 1;
    }

    pub fn project(&self, position: [u16; 2], dimensions: [u16; 2]) -> [[u16; 2]; 2] {
        let [pointx, pointy] = self.world_to_camera(position[0], position[1]);

        let [endx, endy]: [u16; 2] = [
            min(max(pointx + dimensions[0] as i32, 0) as u16, self.width),
            min(max(pointy + dimensions[1] as i32, 0) as u16, self.height),
        ];
        let [startx, starty] = [max(pointx, 0) as u16, max(pointy, 0) as u16];
        [[startx, starty], [endx, endy]]
    }
    pub fn world_to_camera(&self, x: u16, y: u16) -> [i32; 2] {
        let upcornerx = self.focus[0] - (self.width / 2) as i32;
        let upcornery = self.focus[1] - (self.height / 2) as i32;
        [x as i32 - upcornerx, y as i32 - upcornery]
    }

    pub fn render<Rend: Renderable>(camera: &mut Camera, what: &Rend) -> Result<()> {
        camera.buffer = camera.blackout_buffer.clone();
        what.render(camera);
        camera.render_to_console()
    }
    fn render_to_console(&mut self) -> Result<()> {
        let blackout_str = self.render_buffer();
        let content = self.render_buffer();
        execute!(stdout(), MoveTo(0, 0),)?;
        print!("{}", blackout_str);
        execute!(stdout(), MoveTo(0, 0),)?;
        print!("{}", content);
        Ok(())
    }
    fn render_buffer(&self) -> String {
        // TODO measure size of a styled glyph.
        let mut output: String = String::with_capacity(self.buffer.len() * 4);
        for (i, glyph) in self.buffer.iter().enumerate() {
            if i % self.width as usize == 0 && i > 0 {
                output += "\r\n";
            }
            output += &format!("{}", glyph.ch.with(glyph.fg.into()).on(glyph.bg.into()));
        }
        output
    }
}
