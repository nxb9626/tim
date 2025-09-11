use chrono::Utc;
use components::{
    BACKGROUND, COMPONENT_LAYERS, Component, DEBUGGER, FOREGROUND, crosshair::Crosshair,
    debug::Debugger,
};
use shapes::{
    Color, H_CENTER, HEIGHT, Pos, PosOrientation, Position, W_CENTER, WIDTH,
    shape::{Rectangle, Shapes},
    text::{Styling, Text},
};

use tokio::task::yield_now;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = shapes::init();

    let video_subsystem = sdl_context
        .video()
        .expect("Failed to start sdl3 video system.");

    let event_pump = sdl_context
        .event_pump()
        .expect("Failed to start sdl3 event (Input) system.");

    // eventually need two way channels for events to go from filesystem to gui
    tokio::select! {
        run_res = vis::vis(video_subsystem) => run_res.unwrap(), // just exit for now
        update_res = physics() => update_res.unwrap(), // just exit for now
        input = input::input_loop(event_pump) => input.unwrap(), // just exit for now
    };

    Ok(())
}

pub trait Phys {
    fn update_position(&mut self, pos: Position);
    fn get_position(&mut self) -> Position;
    fn has_gravity(&self) -> bool;
}

pub async fn physics() -> Result<(), ()> {
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

        let mut objs = COMPONENT_LAYERS.lock().await;

        // background always in the back
        {
            let background = objs.get_mut(&BACKGROUND).unwrap();
            background.insert("background".to_string(), Shapes::Rectangle(bg).into());
        }
        {
            let foreground = objs.get_mut(&FOREGROUND).unwrap();
            foreground.insert("crosshair".to_string(), Component::Crosshair(Crosshair {}));
        }
        // debugger always on top layer
        {
            let layer_last = objs.get_mut(&DEBUGGER).unwrap();
            layer_last.insert(
                "debugger".to_string(),
                Component::Debugger(Debugger::new().await),
            );
        }
    }

    let start = Utc::now();

    loop {
        let mut objs = COMPONENT_LAYERS.lock().await;

        let time_since = Utc::now() - start;
        let layer = objs.get_mut(&FOREGROUND).unwrap();

        let val = time::format_timedelta(time_since);
        let tx = Text {
            val,
            pos: Pos::at(W_CENTER, H_CENTER),
            size: shapes::text::TextSize::Large,
            color: Color::WHITE,
            style: vec![Styling::Background(Color::CYAN)],
        };

        layer.insert("time".to_string(), Component::Shapes(tx.into()));
        yield_now().await;
    }
}
