use sdl3::rect::Point;

use crate::{Color, Draw, Position, View};

#[derive(Debug)]
pub struct Square {
    pub color: Color,
    pub size: u32,
    pub pos: Position,
    pub hollow: bool,
}

impl Draw for Square {
    fn draw(&self, t: &mut View) {
        let (x, y) = match self.pos.relative {
            crate::PosOrientation::Center => (
                self.pos.x - self.size.div_ceil(2) as i32,
                self.pos.y - self.size.div_ceil(2) as i32,
            ),
            crate::PosOrientation::TopLeft => (self.pos.x, self.pos.y),
        };

        if let Err(e) = t.try_draw_rect(
            x,
            y,
            self.size as u32,
            self.size as u32,
            self.hollow,
            self.color,
        ) {
            dbg!("sqr@{:?}, {:?}", self.pos, e);
        }
    }
}

pub struct Rectangle {
    pub color: Color,
    pub width: u32,
    pub height: u32,
    pub pos: Position,
    pub hollow: bool,
}

impl Draw for Rectangle {
    fn draw(&self, t: &mut View) {
        let (x, y) = match self.pos.relative {
            crate::PosOrientation::Center => (
                self.pos.x - self.width.div_ceil(2) as i32,
                self.pos.y - self.height.div_ceil(2) as i32,
            ),
            crate::PosOrientation::TopLeft => (self.pos.x, self.pos.y),
        };

        if let Err(e) = t.try_draw_rect(
            x,
            y,
            self.width as u32,
            self.height as u32,
            self.hollow,
            self.color,
        ) {
            dbg!("rect@{:?}, {:?}", self.pos, e);
        }
    }
}
pub struct Pixel {
    pub color: Color,
    pub position: Position,
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
