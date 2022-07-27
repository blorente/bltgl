use std::io::stdout;

use camera::{Camera, Renderable};
use color::ColorRGBA;
use components::{quad::Quad, textbox::TextBox};
use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event, KeyCode},
    execute, terminal,
    terminal::{Clear, ClearType},
    Result,
};

mod camera;
mod color;
mod components;

struct World {
    quads: Vec<Quad>,
    textboxes: Vec<TextBox>,
    index: usize,
}

impl Renderable for World {
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
    }
}
fn test_world() -> World {
    World {
        #[rustfmt::skip]
        quads: vec![
            Quad::new( [1, 1], 5, 5, ColorRGBA::green() ),
            Quad::new( [2, 2], 5, 5, ColorRGBA::red() ),
            Quad::new( [11, 3], 10, 10, ColorRGBA::blue() ),
            Quad::new( [20, 4], 1000, 10, ColorRGBA::white() ),
        ],
        #[rustfmt::skip]
        textboxes: vec![
            TextBox::new([10, 10], 29, "Hello"),
            TextBox::new([16, 19], 29, "Hello"),
        ],
        index: 0,
    }
}

fn run_app(mut world: World) -> Result<()> {
    let mut camera = Camera::new([90, 30]);
    Camera::render(&mut camera, &world)?;
    loop {
        // Blocking read
        let event = read()?;

        // Movement of camera
        if event == Event::Key(KeyCode::Char('h').into()) {
            camera.move_left()
        }
        if event == Event::Key(KeyCode::Char('j').into()) {
            camera.move_down()
        }
        if event == Event::Key(KeyCode::Char('k').into()) {
            camera.move_up()
        }
        if event == Event::Key(KeyCode::Char('l').into()) {
            camera.move_right()
        }

        if event == Event::Key(KeyCode::Char('n').into()) {
            world.index = (world.index + 1) % world.textboxes.len();
            camera.focus_on(world.textboxes[world.index].center(&camera));
        }
        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        }

        if let Event::Resize(_, _) = event {}

        if event == Event::Key(KeyCode::Esc.into()) {
            break;
        }
        Camera::render(&mut camera, &world)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), Hide, Clear(ClearType::All))?;

    let world = test_world();
    run_app(world)?;

    execute!(stdout(), Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
