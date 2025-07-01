use sdl3::{rect::Point, render::FRect};

use crate::{Color, Draw, Objects, Position, View};

#[derive(Debug)]
pub struct Square {
    pub color: Color,
    pub size: f32,
    pub pos: Position,
    pub hollow: bool,
}

impl Draw for Square {
    fn draw(&self, t: &mut View) {
        let (x, y) = match self.pos.relative {
            crate::PosOrientation::Center => (self.pos.x / 2.0, self.pos.y / 2.0),
            crate::PosOrientation::TopLeft => (self.pos.x, self.pos.y),
        };

        if let Err(e) = t.try_draw_rect(x, y, self.size, self.size, self.hollow, self.color) {
            dbg!("sqr@{:?}, {:?}", self.pos, e);
        }
    }
}

impl From<Square> for Objects {
    fn from(value: Square) -> Self {
        Objects::Square(value)
    }
}

pub struct Rectangle {
    pub color: Color,
    pub width: f32,
    pub height: f32,
    pub pos: Position,
    pub hollow: bool,
}

impl Into<FRect> for &Rectangle {
    fn into(self) -> FRect {
        FRect {
            x: self.pos.x as f32,
            y: self.pos.y as f32,
            w: self.width as f32,
            h: self.height as f32,
        }
    }
}

impl Draw for Rectangle {
    fn draw(&self, t: &mut View) {
        let (x, y) = match self.pos.relative {
            crate::PosOrientation::Center => (
                self.pos.x - self.width / 2.0,
                self.pos.y - self.height / 2.0,
            ),
            crate::PosOrientation::TopLeft => (self.pos.x, self.pos.y),
        };

        if let Err(e) = t.try_draw_rect(x, y, self.width, self.height, self.hollow, self.color) {
            dbg!("rect@{:?}, {:?}", self.pos, e);
        }
    }
}
pub struct Pixel {
    pub color: Color,
    pub position: Position,
}

impl From<Pixel> for Objects {
    fn from(value: Pixel) -> Self {
        Objects::Pixel(value)
    }
}

impl Draw for Pixel {
    fn draw(&self, t: &mut View) {
        t.canvas.set_draw_color(self.color);
        match t
            .canvas
            .draw_point(Point::new(self.position.x as i32, self.position.y as i32))
        {
            Ok(_) => {}
            Err(_) => {
                dbg!("pixel@({:?},{}:?)", self.position.x, self.position.y);
            }
        };
    }
}

impl From<Rectangle> for Objects {
    fn from(value: Rectangle) -> Self {
        Objects::Rectangle(value)
    }
}
