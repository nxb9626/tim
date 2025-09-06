use chrono::{TimeDelta, Utc};
use sdl3::{EventPump, VideoSubsystem, event::Event, keyboard::Scancode, mouse::MouseButton};

use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    task::yield_now,
};

use std::{collections::HashSet, time::Duration};

use components::{COMPONENT_LAYERS, Component, DEBUGGER, debug::DbgVal};
use shapes::{HEIGHT, View, WIDTH, draw_shapes};

const MAX_FRAME_RATE: u64 = 240;
const SCALE: f32 = 1.0;

pub struct Signal {
    keeb: KeebUpdate,
    mouse: MouseUpdate,
    events: Vec<Event>,
}

pub struct KeebUpdate {
    old: HashSet<Scancode>,
    new: HashSet<Scancode>,
}

pub struct MouseUpdate {
    old: HashSet<MouseButton>,
    new: HashSet<MouseButton>,
}

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
            if let Some(Signal {
                keeb,
                mouse: _,
                events: _,
            }) = receiver.recv().await
            {
                if keeb.new.contains(&Scancode::Escape) {
                    break 'view;
                }
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

pub async fn input(mut events: EventPump, sender: UnboundedSender<Signal>) -> Result<(), ()> {
    let mut _start_timestamp = Utc::now();
    let mut prev_mouse_buttons = HashSet::new();
    let mut prev_keeb_buttons = HashSet::new();

    loop {
        let mut polled_events = Vec::new();
        for e in events.poll_iter() {
            polled_events.push(e);
        }
        //     match event {
        //         Event::KeyDown {
        //             keycode: Some(Keycode::Escape),
        //             ..
        //         }
        //         | Event::Quit { .. } => {
        //             if let Err(e) = sender.send(Signal::) {
        //                 panic!("Signal failed to quit: {e}")
        //             }
        //         }
        //         Event::KeyDown {
        //             keycode: Some(Keycode::Space),
        //             ..
        //         } => {
        //             _start_timestamp = Utc::now();
        //         }
        //         Event::KeyDown {
        //             keycode: Some(Keycode::F3),
        //             ..
        //         } => {
        //             let mut x = COMPONENT_LAYERS.lock().await;
        //             let layer2 = match x.get_mut(&DEBUGGER) {
        //                 Some(layer2) => layer2,
        //                 None => continue,
        //             };

        //             match layer2.get_mut("debugger") {
        //                 Some(Component::Debugger(debugger)) => {
        //                     debugger.toggle();
        //                 }
        //                 _ => {}
        //             };
        //         }
        //         _ => {}
        //     }
        // }

        // Create a set of pressed Keys.
        let keebstate = events.keyboard_state();
        let keeb_buttons = keebstate.pressed_scancodes().collect();

        // Get the difference between the new and old sets.
        let new_keeb_buttons = &keeb_buttons - &prev_keeb_buttons;
        let old_keeb_buttons = &prev_keeb_buttons - &keeb_buttons;

        let state = events.mouse_state();

        // Create a set of pressed Keys.
        let mouse_buttons = state.pressed_mouse_buttons().collect();

        // Get the difference between the new and old sets.
        let new_mouse_buttons = &mouse_buttons - &prev_mouse_buttons;
        let old_mouse_buttons = &prev_mouse_buttons - &mouse_buttons;

        // Create a set of pressed Keys.
        // let events = events.poll_iter().collect();

        // Get the difference between the new and old sets.
        // let new_events = &events - &prev_events;
        // let old_events = &prev_events - &events;

        if !new_mouse_buttons.is_empty()
            || !old_mouse_buttons.is_empty()
            || !new_keeb_buttons.is_empty()
            || !old_keeb_buttons.is_empty()
            || !polled_events.is_empty()
        {
            println!(
                "X = {:?}, Y = {:?} : \n {:?} -> {:?} \n {:?} -> {:?} \n {:?}",
                state.x(),
                state.y(),
                &new_mouse_buttons,
                &old_mouse_buttons,
                &new_keeb_buttons,
                &old_keeb_buttons,
                &polled_events
            );

            let keeb_update = KeebUpdate {
                old: old_keeb_buttons,
                new: new_keeb_buttons,
            };

            let mouse_update = MouseUpdate {
                old: old_mouse_buttons,
                new: new_mouse_buttons,
            };

            let input_update = Signal {
                keeb: keeb_update,
                mouse: mouse_update,
                events: polled_events,
            };

            if let Err(e) = sender.send(input_update) {
                panic!("Failed to send Input Update: {e}")
            };
        }

        prev_mouse_buttons = mouse_buttons;
        prev_keeb_buttons = keeb_buttons;
        yield_now().await;
    }
}
