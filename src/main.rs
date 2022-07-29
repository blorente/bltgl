use std::io::stdout;

use app::App;
use camera::Camera;
use components::{prompt::Prompt, status::Status, textbox::TextBox};
use crossterm::{
    cursor::{Hide, Show},
    event::{read, Event, KeyEvent},
    execute, terminal,
    terminal::{Clear, ClearType},
};
use eyre::Result;

mod app;
mod camera;
mod color;
mod components;

fn test_world() -> App {
    App::new(
        vec![
            TextBox::new([10, 10], 29, "Hello"),
            TextBox::new([16, 19], 29, "nayla is very pretty"),
            TextBox::new([106, 19], 29, "Hello asdfasdf"),
            TextBox::new([16, 50], 29, "Hello this is some text"),
        ],
        0,
        Status::new(),
        Prompt::new("> "),
    )
}

fn run_app(mut world: App) -> Result<()> {
    let mut camera = Camera::new([90, 30]);
    Camera::render(&mut camera, &world)?;
    loop {
        // Blocking read
        let event = read()?;

        // Movement of camera
        match event {
            Event::Key(KeyEvent { code, .. }) => {
                let exit = world.handle_key(code, &mut camera)?;
                if exit {
                    break;
                }
            }
            Event::Mouse(_) => todo!(),
            Event::Resize(cols, rows) => camera.resize(cols, rows),
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
