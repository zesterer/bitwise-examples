use minifb::{Window, WindowOptions};
use std::{
    marker::PhantomData,
    time::Duration,
};

pub use minifb::Key;

pub trait Game: Sized + 'static {
    const NAME: &'static str;
    const WIDTH: usize;
    const HEIGHT: usize;

    fn init() -> u64;

    fn tick(prev: u64, input: &Input<'_, Self>, output: &mut Output<'_, Self>) -> u64;

    fn run() -> ! {
        let mut win = Window::new(
            Self::NAME,
            Self::WIDTH,
            Self::HEIGHT,
            WindowOptions::default(),
        ).unwrap();

        win.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        let mut state = Self::init();

        let mut tick = 0;
        while win.is_open() && !win.is_key_down(Key::Escape) {
            let mut buf = vec![0; Self::WIDTH * Self::HEIGHT];

            let input = Input {
                win: &win,
                tick,
                phantom: PhantomData,
            };
            let mut output = Output::new();

            state = Self::tick(state, &input, &mut output);

            output.write_to(&mut buf);

            win.update_with_buffer(&buf, Self::WIDTH, Self::HEIGHT).unwrap();

            tick += 1;
        }

        std::process::exit(0)
    }
}

pub struct Input<'a, G: Game> {
    win: &'a Window,
    tick: u64,
    phantom: PhantomData<&'static mut G>,
}

impl<'a, G: Game> Input<'a, G> {
    pub fn tick(&self) -> u64 { self.tick }

    pub fn is_key_down(&self, key: Key) -> bool { self.win.is_key_down(key) }
}

pub struct Output<'a, G: Game> {
    shapes: Vec<Shape>,
    phantom: PhantomData<&'a mut G>,
}

enum Shape {
    Rect { x: i32, y: i32, w: u32, h: u32, color: [u8; 3] },
}

impl<'a, G: Game> Output<'a, G> {
    pub fn rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: [u8; 3]) {
        self.shapes.push(Shape::Rect { x, y, w, h, color });
    }

    fn new() -> Self {
        Self {
            shapes: Vec::new(),
            phantom: PhantomData,
        }
    }

    fn write_to(self, buf: &mut Vec<u32>) {
        for shape in self.shapes {
            match shape {
                Shape::Rect { x, y, w, h, color } => {
                    for j in 0..h {
                        for i in 0..w {
                            let pos = [x + i as i32, y + j as i32];
                            if pos[0] > 0 && pos[0] < G::WIDTH as i32 && pos[1] > 0 && pos[1] < G::HEIGHT as i32 {
                                buf[pos[1] as usize * G::WIDTH + pos[0] as usize] = u32::from_le_bytes([color[0], color[1], color[2], 255]);
                            }
                        }
                    }
                },
            }
        }
    }
}
