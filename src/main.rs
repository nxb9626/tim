use std::{collections::HashMap, sync::Arc};

use chrono::Utc;
use gui::{
    Color, HEIGHT, Objects, Pos, PosOrientation, Position, Signal, WIDTH, input,
    shape::Rectangle,
    text::{Styling, Text},
};

use tokio::{sync::Mutex, task::yield_now};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let objects: Arc<Mutex<HashMap<usize, Vec<Objects>>>> = Arc::new(Mutex::new(HashMap::new()));

    let sdl_context = gui::init();

    let video_subsystem = sdl_context.video().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    let (quit_sender, quit_receiver) = tokio::sync::mpsc::unbounded_channel::<Signal>();

    // eventually need two way channels for events to go from filesystem to gui
    tokio::select! {
        run_res = gui::vis(video_subsystem, objects.clone(), quit_receiver) => run_res.unwrap(), // just exit for now
        update_res = physics(objects.clone()) => update_res.unwrap(), // just exit for now
        input = input(event_pump, objects.clone(), quit_sender) => input.unwrap(), // just exit for now
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

pub async fn physics(objects: Arc<Mutex<HashMap<usize, Vec<Objects>>>>) -> Result<(), ()> {
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

        objs.insert(layer_count.new_layer(), Vec::with_capacity(100_000_000));
        objs.insert(layer_count.new_layer(), Vec::with_capacity(100_000_000));
        let layer = objs.get_mut(&1).unwrap();

        layer.push(bg.into());
    }

    // let range_x: Vec<i32> = (0..(WIDTH / 5)).map(|x| x * 2).collect();
    // let range_y: Vec<i32> = (0..(HEIGHT / 5)).map(|x| x * 2).collect();

    let start = Utc::now();

    // for x in range_x.into_iter() {
    //     for y in range_y.clone().into_iter() {
    //         let z = x * y;
    //         let fizz = z % 3 == 0;
    //         let buzz = z % 5 == 0;
    //         let color = match (fizz, buzz) {
    //             (true, true) => gui::Color::WHITE,
    //             (true, false) => gui::Color::PINK,
    //             (false, true) => gui::Color::CYAN,
    //             (false, false) => gui::Color::BLACK,
    //         };
    //         let mut objs = objects.lock().await;

    //         let sq = Pixel {
    //             color,
    //             position: Pos::at(x, y),
    //         };

    //         layer.push(sq.into());

    //         layer.push(tx.into());
    //     }
    // }

    loop {
        let mut objs = objects.lock().await;

        let time_since = Utc::now() - start;
        let layer = objs.get_mut(&2).unwrap();

        layer.clear();

        let mins = time_since.num_minutes();
        let secs = time_since.num_seconds();
        // let millis = time_since.subsec_millis();

        let tx = Text {
            val: format!("{}:{}", mins, secs),
            pos: Pos::at(WIDTH / 2.0, HEIGHT / 2.0),
            size: gui::text::TextSize::Large,
            color: Color::WHITE,
            style: vec![Styling::Background(Color::CYAN)],
        };

        layer.push(tx.into());
        yield_now().await;
    }
}
