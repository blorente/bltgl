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

fn render_quad(quad: &Quad, camera: &mut Camera) {
    let [pointx, pointy] = camera.world_to_camera(quad.pos[0], quad.pos[1]);

    let [endx, endy]: [u16; 2] = [
        min(max(pointx + quad.width as i32, 0) as u16, camera.width),
        min(max(pointy + quad.height as i32, 0) as u16, camera.height),
    ];
    let [startx, starty] = [max(pointx, 0) as u16, max(pointy, 0) as u16];
    eprintln!(
        "Rendering Quad: Start=({}, {}) End=({}, {})",
        startx, starty, endx, endy
    );
    for x in startx..endx {
        for y in starty..endy {
            camera.buffer[(y * camera.width + x) as usize] = Glyph {
                fg: ColorRGBA::black().into(),
                bg: quad.color.into(),
                ch: ' ',
            }
        }
    }
}
fn mark_quad(quad: &Quad, camera: &mut Camera) {
    let [pointx, pointy] = camera.world_to_camera(quad.pos[0], quad.pos[1]);

    let [endx, endy]: [u16; 2] = [
        min(max(pointx + quad.width as i32, 0) as u16, camera.width),
        min(max(pointy + quad.height as i32, 0) as u16, camera.height),
    ];
    let [startx, starty] = [max(pointx, 0) as u16, max(pointy, 0) as u16];
    eprintln!(
        "Marking Quad: Start=({}, {}) End=({}, {})",
        startx, starty, endx, endy
    );
    for x in startx..endx {
        for y in starty..endy {
            camera.buffer[(y * camera.width + x) as usize] = Glyph {
                fg: ColorRGBA::black().into(),
                bg: quad.color.into(),
                ch: 'M',
            }
        }
    }
}

fn render_buffer(camera: &Camera, buffer: &Vec<Glyph>) -> String {
    // TODO measure size of a styled glyph.
    let mut output: String = String::with_capacity(camera.buffer.len() * 4);
    for (i, glyph) in camera.buffer.iter().enumerate() {
        if i % camera.width as usize == 0 && i > 0 {
            output += "\r\n";
        }
        output += &format!("{}", glyph.ch.with(glyph.fg.into()).on(glyph.bg.into()));
    }
    output
}
fn render_to_console(camera: &mut Camera) {
    let blackout_str = render_buffer(&camera, &camera.blackout_buffer);
    let content = render_buffer(&camera, &camera.buffer);
    execute!(stdout(), MoveTo(0, 0),);
    print!("{}", blackout_str);
    execute!(stdout(), MoveTo(0, 0),);
    print!("{}", content);
}

fn render(world: &World, camera: &mut Camera) {
    camera.buffer = camera.blackout_buffer.clone();
    for (i, ele) in world.quads.iter().enumerate() {
        render_quad(&ele, camera);
        if i == world.index {
            mark_quad(&ele, camera);
        }
    }
    render_to_console(camera);
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

struct Camera {
    buffer: Vec<Glyph>,
    blackout_buffer: Vec<Glyph>,
    // TODO Move these to camera
    width: u16,
    height: u16,
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

    pub fn world_to_camera(&self, x: u16, y: u16) -> [i32; 2] {
        let upcornerx = self.focus[0] - (self.width / 2) as i32;
        let upcornery = self.focus[1] - (self.height / 2) as i32;
        [x as i32 - upcornerx, y as i32 - upcornery]
    }
}

fn run_app(world: &mut World) -> Result<()> {
    let mut camera = Camera::new([90, 30]);
    render(&world, &mut camera);
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
            world.index = (world.index + 1) % world.quads.len();
        }
        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        }

        if let Event::Resize(_, _) = event {}

        if event == Event::Key(KeyCode::Esc.into()) {
            break;
        }
        render(&world, &mut camera);
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
