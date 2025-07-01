use sdl3::{render::FRect, ttf::Font};

use crate::{Color, Failed, Objects, Position};

#[derive(Debug)]
pub enum TextSize {
    Tiny,
    Small,
    Large,
    TooBig,
}

impl From<&TextSize> for f32 {
    fn from(value: &TextSize) -> Self {
        match value {
            TextSize::Tiny => 20.0,
            TextSize::Small => 40.0,
            TextSize::Large => 80.0,
            TextSize::TooBig => 160.0,
        }
    }
}

#[derive(Debug)]
pub enum Styling {
    Background(Color),
}

#[derive(Debug)]
pub struct Text {
    pub val: String,
    pub pos: Position,
    pub size: TextSize,
    pub color: Color,
    pub style: Vec<Styling>,
}

impl Text {
    pub fn draw(&self, target: &mut crate::View, font: &mut Font) -> Result<(), Failed> {
        let texture_creator = target.canvas.texture_creator();

        let surface = match font.render(&self.val).blended(Color::PINK) {
            Ok(surf) => surf,
            Err(_e) => return Err(Failed::RenderText),
        };

        let texture = match texture_creator.create_texture_from_surface(&surface) {
            Ok(texture) => texture,
            Err(_e) => return Err(Failed::RenderText),
        };

        //  monospace only, equal height and width for now
        let h: f32 = (&self.size).into();
        let w: f32 = (h * (self.val.chars().count()) as f32) * 0.45;

        let (x, y) = match self.pos.relative {
            crate::PosOrientation::Center => (self.pos.x - w / 2.0, self.pos.y - h / 2.0),
            crate::PosOrientation::TopLeft => (self.pos.x, self.pos.y),
        };

        //  Text fills this box
        let frect = FRect { x, y, w, h };

        for s in &self.style {
            match s {
                Styling::Background(c) => {
                    target.canvas.set_draw_color(c.clone());
                    match target.canvas.draw_rect(frect) {
                        Ok(_) => Ok(()),
                        Err(_) => Err(Failed::RenderText),
                    }?;
                }
            };
        }

        target.canvas.set_draw_color(self.color);

        match target.canvas.copy(&texture, None, Some(frect)) {
            Ok(_) => Ok(()),
            Err(_) => Err(Failed::RenderText),
        }
    }
}

impl From<Text> for Objects {
    fn from(value: Text) -> Self {
        Objects::Text(value)
    }
}
