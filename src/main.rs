use std::{
    cmp::{max, min},
    io::stdout,
};

use crossterm::{
    cursor::{Hide, MoveLeft, MoveRight, MoveTo, MoveToColumn, MoveToRow, Show},
    event::{read, Event, EventStream, KeyCode},
    execute, queue,
    style::{Color, PrintStyledContent, SetBackgroundColor, SetForegroundColor, Stylize},
    terminal,
    terminal::{Clear, ClearType},
    QueueableCommand, Result,
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

fn render_quad(quad: &Quad, buffer: &mut GraphicsBuffer) {
    let [xstart, ystart] = quad.pos;
    for y in ystart..min(quad.height + ystart, buffer.height) {
        for x in xstart..min(quad.width + xstart, buffer.width) {
            eprintln!("Render quad, x={}, y={}", x, y);
            buffer.buffer[(y * buffer.width + x) as usize] = Glyph {
                fg: ColorRGBA::black().into(),
                bg: quad.color.into(),
                ch: ' ',
            }
        }
    }
}
fn mark_quad(quad: &Quad, buffer: &mut GraphicsBuffer) {
    let [xstart, ystart] = quad.pos;
    for y in ystart..min(quad.height + ystart, buffer.height) {
        for x in xstart..min(quad.width + xstart, buffer.width) {
            eprintln!("Render quad, x={}, y={}", x, y);
            buffer.buffer[(y * buffer.width + x) as usize] = Glyph {
                fg: ColorRGBA::black().into(),
                bg: quad.color.into(),
                ch: 'X',
            }
        }
    }
}

fn render_buffer(buffer: &mut GraphicsBuffer) {
    // TODO measure size of a styled glyph.
    let mut output: String = String::with_capacity(buffer.buffer.len() * 4);
    for (i, glyph) in buffer.buffer.iter().enumerate() {
        if i % buffer.width as usize == 0 && i > 0 {
            output += "\r\n";
        }
        output += &format!("{}", glyph.ch.with(glyph.fg.into()).on(glyph.bg.into()));
    }
    execute!(
        stdout(),
        SetBackgroundColor(Color::Black),
        SetForegroundColor(Color::Black),
        MoveTo(0, 0),
    );
    print!("{}", output);
}

fn render(world: &World, buffer: &mut GraphicsBuffer) {
    for (i, ele) in world.quads.iter().enumerate() {
        render_quad(&ele, buffer);
        if i == world.index {
            mark_quad(&ele, buffer);
        }
    }
    render_buffer(buffer);
}

#[derive(Clone)]
struct Glyph {
    fg: ColorRGBA,
    bg: ColorRGBA,
    ch: char,
}

impl Glyph {
    fn empty() -> Self {
        Self {
            fg: ColorRGBA::black(),
            bg: ColorRGBA::black(),
            ch: ' ',
        }
    }
}

struct GraphicsBuffer {
    buffer: Vec<Glyph>,
    // TODO Move these to camera
    width: u16,
    height: u16,
}

impl GraphicsBuffer {
    pub fn new(screen_dimensions: [u16; 2]) -> Self {
        Self {
            buffer: vec![Glyph::empty(); (screen_dimensions[0] * screen_dimensions[1]) as usize],
            width: screen_dimensions[0],
            height: screen_dimensions[1],
        }
    }
}

fn run_app(world: &mut World) -> Result<()> {
    let mut buffer = GraphicsBuffer::new([90, 30]);
    render(&world, &mut buffer);
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
        render(&world, &mut buffer);
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
