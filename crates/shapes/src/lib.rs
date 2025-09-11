extern crate sdl3;

pub mod shape;
pub mod text;

use sdl3::Sdl;
use sdl3::pixels::Color as SdlColor;
use sdl3::render::Canvas;
use sdl3::render::FRect;
use sdl3::video::Window;

use crate::shape::Shapes;

const SCALE: f32 = 1.0;

pub const WIDTH: f32 = 800.0 * SCALE;
pub const HEIGHT: f32 = 600.0 * SCALE;

pub const W_CENTER: f32 = WIDTH / 2.0;
pub const H_CENTER: f32 = HEIGHT / 2.0;

#[derive(Debug, Copy, Clone)]
pub enum Color {
    WHITE,
    BLACK,
    CYAN,
    PINK,
}

impl Into<SdlColor> for Color {
    fn into(self) -> SdlColor {
        match self {
            Color::WHITE => SdlColor::WHITE,
            Color::BLACK => SdlColor::BLACK,
            Color::CYAN => SdlColor::CYAN,
            Color::PINK => SdlColor::MAGENTA,
        }
    }
}

// Any objects that need to be drawn need to be given this
pub trait Draw {
    fn draw(&self, target: &mut View);
}

pub struct View {
    pub canvas: Canvas<Window>,
}

#[derive(Clone, Copy, Debug)]
pub enum Failed {
    FailedToDrawRect,
    RenderText,
}

impl View {
    pub fn try_draw_rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        hollow: bool,
        color: impl Into<SdlColor>,
    ) -> Result<(), Failed> {
        self.canvas.set_draw_color(color);

        let r = FRect::new(x, y, w, h);

        if hollow {
            if let Err(_) = self.canvas.draw_rect(r.into()) {
                return Err(Failed::FailedToDrawRect);
            };
        } else {
            if let Err(_) = self.canvas.fill_rect(r) {
                return Err(Failed::FailedToDrawRect);
            }
            if let Err(_) = self.canvas.draw_rect(r.into()) {
                return Err(Failed::FailedToDrawRect);
            };
        }

        return Ok(());
    }
    pub fn present(&mut self) -> bool {
        self.canvas.present()
    }
}

#[derive(Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    // what this is relative to
    pub relative: PosOrientation,
}

pub type Pos = Position;

impl Position {
    pub fn at(x: f32, y: f32) -> Self {
        Position {
            x: x,
            y: y,
            relative: PosOrientation::Center,
        }
    }
}

#[derive(Clone, Debug)]
pub enum PosOrientation {
    TopLeft,
    Center,
    Parent(Box<Position>),
}

pub fn init() -> Sdl {
    let sdl_context = sdl3::init().unwrap();
    return sdl_context;
}

pub fn draw_shapes(font: &mut sdl3::ttf::Font, view: &mut View, shapes: Vec<&Shapes>) {
    for shape in shapes {
        match shape {
            Shapes::Square(w) => w.draw(view),
            Shapes::Pixel(p) => p.draw(view),
            Shapes::Rectangle(r) => r.draw(view),
            Shapes::Line(l) => l.draw(view),
            Shapes::Text(text) => {
                if let Err(e) = text.draw(view, font) {
                    dbg!("Failed to write text:{:?}", e);
                }
            }
        }
    }
}
