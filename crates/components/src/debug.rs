use std::collections::BTreeMap;

use input::{SC, get_input_receiver};
use shapes::{
    Color, PosOrientation, Position,
    shape::Shapes,
    text::{self, Text},
};
use tokio::task::yield_now;

use crate::{COMPONENT_LAYERS, Component, DEBUGGER};

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

pub async fn spawn_input_loop() {
    // need send/recv
    let mut recv = get_input_receiver().await;

    // need register that send
    tokio::task::spawn(async move {
        while let Some(signal) = recv.recv().await {
            if signal.is_this_key(&SC::F3) {
                let mut cl = COMPONENT_LAYERS.lock().await;
                let debug_layer = match cl.get_mut(&DEBUGGER) {
                    Some(dbg) => dbg,
                    _ => {
                        panic!("debugger not initialized")
                    }
                };
                match debug_layer.get_mut("debugger") {
                    Some(Component::Debugger(debugger)) => {
                        debugger.toggle();
                    }
                    _ => {}
                };
            };
            yield_now().await;
        }
    });
}

impl Debugger {
    // async create a new one
    pub async fn new() -> Self {
        spawn_input_loop().await;

        Self {
            vals: BTreeMap::new(),
            visible: true,
        }
    }
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
                    x,
                    y,
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
