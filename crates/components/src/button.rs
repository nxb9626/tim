use shapes::{
    Color, Position,
    shape::{Rectangle, Shapes},
    text::Text,
};

#[derive(Debug)]
pub enum ButtonState {
    Hoverred,
    Down,
    Up,
}

#[derive(Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug)]
pub struct Button {
    pub display: Vec<Shapes>, // stuff that is being displayed
    pub state: ButtonState,
    pub position: Position,
    pub size: Size,
}

impl Button {
    pub fn get_shapes(&self) -> Vec<Shapes> {
        let mut shapes = vec![];
        let color = Color::WHITE;

        let outline = Shapes::Rectangle(Rectangle {
            color: color,
            width: self.size.width,
            height: self.size.height,
            pos: self.position.clone(),
            hollow: true,
        });

        let text = Shapes::Text(Text {
            val: "button".to_string(),
            pos: self.position.clone(),
            size: shapes::text::TextSize::Small,
            color,
            style: vec![],
        });

        shapes.push(outline);
        shapes.push(text);

        match self.state {
            ButtonState::Hoverred => {}
            ButtonState::Down => {}
            ButtonState::Up => {}
        };

        shapes
    }
}
