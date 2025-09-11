use chrono::{TimeDelta, Utc};
use input::{SC, get_input_receiver};
use sdl3::{VideoSubsystem, event::Event};

use tokio::{
    sync::mpsc::{UnboundedSender, unbounded_channel},
    task::yield_now,
};

use std::time::Duration;

use components::{COMPONENT_LAYERS, Component, DEBUGGER, debug::DbgVal};
use shapes::{HEIGHT, View, WIDTH, draw_shapes};

const MAX_FRAME_RATE: u64 = 240;
const SCALE: f32 = 1.0;

// signal used to kill the app
pub struct Quit {}

pub async fn vis(video_subsystem: VideoSubsystem) -> Result<(), ()> {
    let window = video_subsystem
        .window("Mouse", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let canvas = window.into_canvas();

    let ttf_context = sdl3::ttf::init().map_err(|e| e.to_string()).unwrap();

    let mut font = ttf_context
        .load_font(
            "/Users/noah/Projects/mine/tim/fonts/16020_FUTURAM.ttf",
            300.0,
        )
        .unwrap();

    let mut view = View { canvas };

    view.canvas.set_scale(SCALE as f32, SCALE as f32).unwrap();

    let mut framecount = 0;
    let mut time_of_last_update = Utc::now();

    let (kill_send, mut kill_recv) = unbounded_channel::<Quit>();

    spawn_input_loop(kill_send).await; // so the window can react to inputs

    'view: loop {
        if kill_recv.len() > 0 {
            if let Some(Quit {}) = kill_recv.recv().await {
                todo!("The app has exited")
            };
        }

        view.canvas.clear();
        {
            let objs = COMPONENT_LAYERS.lock().await;
            let mut x: Vec<&usize> = objs.keys().collect();
            x.sort();
            x.iter().for_each(|k| {
                let olayer = objs.get(k);
                match olayer {
                    Some(layer) => {
                        layer
                            .values()
                            .for_each(|o| draw_components(&mut font, &mut view, o));
                    }
                    None => {}
                }
            });
        };

        {
            let time_since_last_frame = Utc::now() - time_of_last_update;
            if time_since_last_frame >= TimeDelta::seconds(1) {
                time_of_last_update = Utc::now();
                let mut x = COMPONENT_LAYERS.lock().await;
                let layer2 = match x.get_mut(&DEBUGGER) {
                    Some(layer2) => layer2,
                    None => continue,
                };

                match layer2.get_mut("debugger") {
                    Some(Component::Debugger(debugger)) => {
                        debugger.watch("FPS", DbgVal::U64(framecount));
                    }
                    _ => {}
                };

                framecount = 0;
            }

            framecount += 1;
        }

        if !view.present() {
            break 'view;
        };

        tokio::time::sleep(Duration::from_secs(1 / MAX_FRAME_RATE)).await;
    }

    Ok(())
}

fn draw_components(font: &mut sdl3::ttf::Font, view: &mut View, o: &Component) {
    match o {
        Component::Shapes(shapes) => {
            draw_shapes(font, view, vec![shapes]);
        }
        Component::Debugger(dbgm) => {
            draw_shapes(font, view, dbgm.get_shapes().iter().map(|f| f).collect())
        }
        Component::Crosshair(crosshair) => draw_shapes(
            font,
            view,
            crosshair.get_shapes().iter().map(|f| f).collect(),
        ),
    }
}

pub async fn spawn_input_loop(kill_sender: UnboundedSender<Quit>) {
    // need send/recv
    let mut recv = get_input_receiver().await;

    // need register that send
    tokio::task::spawn(async move {
        while let Some(signal) = recv.recv().await {
            if signal.is_this_key(&SC::Escape) {
                if let Err(e) = kill_sender.send(Quit {}) {
                    dbg!(e);
                    panic!("literally can't quit")
                }
            };

            if signal.held_then_pressed(&SC::LGui, &SC::W) {
                if let Err(e) = kill_sender.send(Quit {}) {
                    dbg!(e);
                    panic!("literally can't quit")
                }
            }

            signal.events.iter().for_each(|a| match &a {
                Event::Quit { .. } => {
                    if let Err(e) = kill_sender.send(Quit {}) {
                        dbg!(e);
                        panic!("literally can't quit")
                    }
                }
                _ => {}
            });

            // .contains(Event::Quit { .. })
            yield_now().await;
        }
    });
}
