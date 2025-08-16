use gui::{
    Color, HEIGHT, PosOrientation, Position, WIDTH,
    shape::{Line, Shapes},
};

#[derive(Debug)]
pub struct Crosshair {}

impl Crosshair {
    pub fn get_shapes(&self) -> Vec<Shapes> {
        let x = Line {
            color: Color::PINK,
            end: Position {
                x: 0.0,
                y: HEIGHT / 2.0,
                relative: PosOrientation::TopLeft,
            },
            start: Position {
                x: WIDTH,
                y: HEIGHT / 2.0,
                relative: PosOrientation::TopLeft,
            },
        };
        let y = Line {
            color: Color::PINK,
            end: Position {
                x: WIDTH / 2.0,
                y: 0.0,
                relative: PosOrientation::TopLeft,
            },
            start: Position {
                x: WIDTH / 2.0,
                y: HEIGHT,
                relative: PosOrientation::TopLeft,
            },
        };
        vec![Shapes::Line(y), Shapes::Line(x)]
    }
}
