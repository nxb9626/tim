use std::{collections::BTreeMap, sync::Arc};

use chrono::Utc;
use components::{Component, crosshair::Crosshair, debug::Debugger};
use gui::{
    Color, H_CENTER, HEIGHT, Pos, PosOrientation, Position, Signal, W_CENTER, WIDTH,
    shape::{Rectangle, Shapes},
    text::{Styling, Text},
};

use tokio::{sync::Mutex, task::yield_now};
use vis::Objects;

// The global god object type

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut objects: Objects = Arc::new(Mutex::new(BTreeMap::new()));

    let sdl_context = gui::init();

    let video_subsystem = sdl_context.video().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    let (quit_sender, quit_receiver) = tokio::sync::mpsc::unbounded_channel::<Signal>();

    // eventually need two way channels for events to go from filesystem to gui
    tokio::select! {
        run_res = vis::vis(video_subsystem, objects.clone(), quit_receiver) => run_res.unwrap(), // just exit for now
        update_res = physics(objects.clone()) => update_res.unwrap(), // just exit for now
        input = vis::input(event_pump, objects.clone(), quit_sender) => input.unwrap(), // just exit for now
    };

    Ok(())
}

struct LayerTracker {
    id: usize,
}

impl LayerTracker {
    fn new_layer(&mut self) -> usize {
        self.id += 1;
        return self.id;
    }
}

pub trait Phys {
    fn update_position(&mut self, pos: Position);
    fn get_position(&mut self) -> Position;
    fn has_gravity(&self) -> bool;
}

pub async fn physics(objects: Objects) -> Result<(), ()> {
    let mut layer_count = LayerTracker { id: 0 };
    {
        let bg = Rectangle {
            color: Color::BLACK,
            width: WIDTH,
            height: HEIGHT,
            hollow: false,
            pos: Position {
                x: 0.0,
                y: 0.0,
                relative: PosOrientation::TopLeft,
            },
        };
        let mut objs = objects.lock().await;

        objs.insert(layer_count.new_layer(), BTreeMap::new());

        objs.insert(layer_count.new_layer(), BTreeMap::new());

        objs.insert(layer_count.new_layer(), BTreeMap::new());

        // background always in the back
        {
            let layer = objs.get_mut(&1).unwrap();
            layer.insert(
                "background".to_string(),
                Component::Shapes(Shapes::Rectangle(bg)),
            );
            layer.insert("crosshair".to_string(), Component::Crosshair(Crosshair {}));
        }

        // debugger always on top
        {
            let last_index = 1;
            let layer_last = objs.get_mut(&last_index).unwrap();
            layer_last.insert(
                "debugger".to_string(),
                Component::DebugMenu(Debugger::default()),
            );
        }
    }

    let start = Utc::now();

    loop {
        let mut objs = objects.lock().await;

        let time_since = Utc::now() - start;
        let layer = objs.get_mut(&2).unwrap();

        layer.clear();

        let val = time::format_timedelta(time_since);
        let tx = Text {
            val: val,
            pos: Pos::at(W_CENTER, H_CENTER),
            size: gui::text::TextSize::Large,
            color: Color::WHITE,
            style: vec![Styling::Background(Color::CYAN)],
        };

        layer.insert("crosshair".to_string(), Component::Shapes(tx.into()));
        yield_now().await;
    }
}
