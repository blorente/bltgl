use std::{
    cmp::{max, min},
    io::stdout,
};

use crossterm::{
    cursor::{Hide, MoveLeft, MoveRight, MoveTo, MoveToColumn, MoveToRow, Show},
    event::{read, Event, EventStream, KeyCode},
    execute,
    style::{Color, SetBackgroundColor, SetForegroundColor},
    terminal,
    terminal::{Clear, ClearType},
    Result,
};

#[derive(Clone, Copy)]
struct ColorRGBA(u8, u8, u8, u8);
impl Into<[u8; 4]> for ColorRGBA {
    fn into(self) -> [u8; 4] {
        [self.0, self.1, self.2, self.3]
    }
}
impl Into<crossterm::style::Color> for ColorRGBA {
    fn into(self) -> crossterm::style::Color {
        Color::Rgb {
            r: self.0,
            g: self.1,
            b: self.2,
        }
    }
}

#[rustfmt::skip]
impl ColorRGBA {
    fn red() -> ColorRGBA { ColorRGBA(255, 0, 0, 0) }
    fn green() -> ColorRGBA { ColorRGBA(0, 255, 0, 0) }
    fn blue() -> ColorRGBA { ColorRGBA(0, 0, 255, 0) }
    fn black() -> ColorRGBA { ColorRGBA(0, 0, 0, 0) }
    fn white() -> ColorRGBA { ColorRGBA(255, 255, 255, 0) }
}

struct Quad {
    pos: [u16; 2],
    width: u16,
    height: u16,
    color: ColorRGBA,
}

impl Quad {
    pub fn new(pos: [u16; 2], width: u16, height: u16, color: ColorRGBA) -> Self {
        Self {
            pos,
            width,
            height,
            color,
        }
    }
}

struct World {
    quads: Vec<Quad>,
    index: usize,
}

fn test_world() -> World {
    World {
        #[rustfmt::skip]
        quads: vec![
            Quad::new( [1, 1], 5, 5, ColorRGBA::green() ),
            Quad::new( [2, 2], 5, 5, ColorRGBA::red() ),
            Quad::new( [11, 3], 10, 10, ColorRGBA::blue() ),
            Quad::new( [20, 4], 1000, 10, ColorRGBA::green() ),
        ],
        index: 0,
    }
}

fn render_quad(quad: &Quad) {
    let [xstart, ystart] = quad.pos;
    execute!(
        stdout(),
        SetBackgroundColor(quad.color.into()),
        SetForegroundColor(Color::Black),
    );
    for i in 0..quad.height {
        let x = xstart;
        let y = ystart + i;
        let width = min(quad.width, 100);
        execute!(stdout(), MoveTo(x, y));
        print!("{}", " ".repeat(width as usize));
    }
}
fn mark_quad(quad: &Quad) {
    let [xstart, ystart] = quad.pos;
    execute!(
        stdout(),
        SetBackgroundColor(quad.color.into()),
        SetForegroundColor(Color::Black),
    );

    execute!(stdout(), MoveTo(xstart, ystart));
    print!("X");
}

fn render(world: &World) {
    execute!(
        stdout(),
        SetBackgroundColor(Color::Black),
        Clear(ClearType::All),
    );
    for (i, ele) in world.quads.iter().enumerate() {
        render_quad(&ele);
        if i == world.index {
            mark_quad(&ele);
        }
    }
}

fn run_app(world: &mut World) -> Result<()> {
    render(&world);
    loop {
        // Blocking read
        let event = read()?;

        if event == Event::Key(KeyCode::Char('c').into()) {}
        if event == Event::Key(KeyCode::Char('n').into()) {
            world.index = (world.index + 1) % world.quads.len();
        }
        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        }

        if let Event::Resize(_, _) = event {}

        if event == Event::Key(KeyCode::Esc.into()) {
            break;
        }
        render(&world);
    }

    Ok(())
}

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), Hide, Clear(ClearType::All))?;

    let mut world = test_world();
    run_app(&mut world)?;

    execute!(stdout(), Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
