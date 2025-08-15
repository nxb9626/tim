use chrono::{TimeDelta, Utc};
use sdl3::{EventPump, VideoSubsystem, event::Event, keyboard::Keycode};

use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task::yield_now,
};

use std::{collections::HashSet, time::Duration};

use components::{COMPONENT_LAYERS, Component};
use gui::{HEIGHT, Signal, View, WIDTH, draw_shapes};

const MAX_FRAME_RATE: u64 = 240;
const SCALE: f32 = 1.0;

pub async fn vis(
    video_subsystem: VideoSubsystem,
    mut receiver: UnboundedReceiver<Signal>,
) -> Result<(), ()> {
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

    'view: loop {
        if receiver.len() > 0 {
            if let Some(Signal::Quit) = receiver.recv().await {
                break 'view;
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
                        layer.values().for_each(|o| match o {
                            Component::Shapes(shapes) => {
                                draw_shapes(&mut font, &mut view, vec![shapes]);
                            }
                            Component::Debugger(dbgm) => draw_shapes(
                                &mut font,
                                &mut view,
                                dbgm.get_shapes().iter().map(|f| f).collect(),
                            ),
                            Component::Crosshair(crosshair) => draw_shapes(
                                &mut font,
                                &mut view,
                                crosshair.get_shapes().iter().map(|f| f).collect(),
                            ),
                        });
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
                let layer2 = match x.get_mut(&1) {
                    Some(layer2) => layer2,
                    None => continue,
                };

                match layer2.get_mut("debugger") {
                    Some(Component::Debugger(debugger)) => {
                        debugger.watch("FPS", components::debug::DbgVal::U64(framecount));
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

pub async fn input(mut events: EventPump, sender: UnboundedSender<Signal>) -> Result<(), ()> {
    let mut start_timestamp = Utc::now();
    let mut prev_buttons = HashSet::new();
    loop {
        for event in events.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => {
                    if let Err(e) = sender.send(Signal::Quit) {
                        panic!("Signal failed to quit: {e}")
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    dbg!(start_timestamp);
                    start_timestamp = Utc::now();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F3),
                    ..
                } => {
                    let mut x = COMPONENT_LAYERS.lock().await;
                    let layer2 = match x.get_mut(&1) {
                        Some(layer2) => layer2,
                        None => continue,
                    };

                    match layer2.get_mut("debugger") {
                        Some(Component::Debugger(debugger)) => {
                            debugger.toggle();
                        }
                        _ => {}
                    };
                }
                _ => {}
            }
        }

        // get a mouse state
        let state = events.mouse_state();

        // Create a set of pressed Keys.
        let buttons = state.pressed_mouse_buttons().collect();

        // Get the difference between the new and old sets.
        let new_buttons = &buttons - &prev_buttons;
        let old_buttons = &prev_buttons - &buttons;

        if !new_buttons.is_empty() || !old_buttons.is_empty() {
            println!(
                "X = {:?}, Y = {:?} : {:?} -> {:?}",
                state.x(),
                state.y(),
                new_buttons,
                old_buttons
            );
        }
        prev_buttons = buttons;
        yield_now().await;
    }
}
