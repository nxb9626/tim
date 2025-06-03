use std::{collections::HashMap, sync::Arc, time::Duration};

use gui::{
    Color, Draw, HEIGHT, PosOrientation, Position, Signal, WIDTH, input,
    shape::{Pixel, Rectangle, Square},
};
use tokio::{sync::Mutex, task::yield_now};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let objects: Arc<Mutex<HashMap<usize, Vec<Box<dyn Draw>>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let sdl_context = gui::init();
    let video_subsystem = sdl_context.video().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    let (quit_sender, quit_receiver) = tokio::sync::mpsc::unbounded_channel::<Signal>();

    // eventually need two way channels for events to go from filesystem to gui
    tokio::select! {
        run_res = gui::vis(video_subsystem,objects.clone(), quit_receiver) => run_res.unwrap(), // just exit for now
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

pub struct Crate {
    pos: Position,
    size: usize,
}

impl Draw for Crate {
    fn draw(&self, target: &mut gui::View) {
        Square {
            color: Color::WHITE,
            size: self.size,
            pos: self.pos,
            hollow: false,
        }
        .draw(target);
    }
}

impl Phys for Crate {
    fn update_position(&mut self, pos: Position) {
        self.pos = pos
    }

    fn has_gravity(&self) -> bool {
        true
    }

    fn get_position(&mut self) -> Position {
        self.pos
    }
}

pub async fn physics(objects: Arc<Mutex<HashMap<usize, Vec<Box<dyn Draw>>>>>) -> Result<(), ()> {
    let mut layer_count = LayerTracker { id: 0 };
    {
        let bg = Rectangle {
            color: Color::BLACK,
            width: WIDTH,
            height: HEIGHT,
            hollow: false,
            pos: Position {
                x: 0,
                y: 0,
                relative: PosOrientation::TopLeft,
            },
        };
        let mut objs = objects.lock().await;

        objs.insert(layer_count.new_layer(), Vec::with_capacity(100_000_000));
        let layer = objs.get_mut(&1).unwrap();
        layer.push(Box::new(bg));
    }

    // loop {
    let range_x: Vec<i32> = (0..(WIDTH / 5)).map(|x| (x * 5) as i32).collect();
    let range_y: Vec<i32> = (0..(HEIGHT / 5)).map(|x| (x * 5) as i32).collect();

    for x in range_x.into_iter() {
        for y in range_y.clone().into_iter() {
            {
                let mut objs = objects.lock().await;
                let sq = Pixel {
                    color: gui::Color::CYAN,
                    position: Position {
                        x: x,
                        y: y,
                        relative: PosOrientation::Center,
                    },
                };
                let layer = objs.get_mut(&1).unwrap();
                layer.push(Box::new(sq));
                // tokio::time::sleep(Duration::from_nanos(1)).await;
            }
        }
    }

    loop {
        let mut objs = objects.lock().await;
        let layer = objs.get_mut(&1).unwrap();
        layer.iter_mut().for_each(|f| {});

        yield_now().await;
    }
    // }
}
