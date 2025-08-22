use std::collections::BTreeMap;

use shapes::{
    Color, PosOrientation, Position,
    shape::Shapes,
    text::{self, Text},
};

#[derive(Debug)]
pub enum DbgVal {
    F64(f64),
    U64(u64),
    String(String),
}

#[derive(Debug)]
pub struct Debugger {
    vals: BTreeMap<String, DbgVal>,
    visible: bool,
}

impl Default for Debugger {
    fn default() -> Self {
        Self {
            vals: BTreeMap::new(),
            visible: true,
        }
    }
}

impl Debugger {
    /// build and it all out and give up the shapes needed to draw the debug menu
    pub fn get_shapes(&self) -> Vec<Shapes> {
        if !self.visible {
            return vec![];
        }

        let x = 5.0;
        let mut y: f32 = 5.0;
        let gap_size = 25.0;

        let mut shapes = vec![];
        for (label, value) in self.vals.iter() {
            let value = match value {
                DbgVal::F64(f) => f.to_string(),
                DbgVal::U64(u) => u.to_string(),
                DbgVal::String(s) => s.to_string(),
            };

            let formatted_value = Text {
                val: format!("{label}: {value}"),
                pos: Position {
                    x: x,
                    y: y,
                    relative: PosOrientation::TopLeft,
                },
                size: text::TextSize::Tiny,
                color: Color::WHITE,
                style: vec![],
            };
            y += gap_size;
            shapes.push(Shapes::Text(formatted_value));
        }
        shapes
    }

    /// add a value to be watched in this
    pub fn watch(&mut self, label: &str, val: DbgVal) {
        self.vals.insert(label.to_string(), val);
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
}
