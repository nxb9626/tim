extern crate sdl3;

pub mod ascii;
pub mod shape;

use chrono::Utc;
use sdl3::pixels::Color as SdlColor;
use sdl3::video::Window;
use sdl3::{EventPump, Sdl, VideoSubsystem};
use sdl3::{event::Event, keyboard::Keycode, rect::Rect, render::Canvas};
use std::collections::HashMap;
use std::sync::Arc;
use std::{collections::HashSet, time::Duration};
use tokio::sync::Mutex;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::yield_now;

const MAX_FRAME_RATE: u64 = 240;
const SCALE: u32 = 2;

pub const WIDTH: u32 = 640 * SCALE;
pub const HEIGHT: u32 = 480 * SCALE;

pub type Color = SdlColor;

pub enum Signal {
    Quit,
}

// any objects that need to be drawn need to be given this
pub trait Draw {
    fn draw(&self, target: &mut View);
}

pub struct View {
    canvas: Canvas<Window>,
}

#[derive(Clone, Copy, Debug)]
pub enum Failed {
    FailedToDrawRect,
}

impl View {
    pub fn try_draw_rect(
        &mut self,
        x: i32,
        y: i32,
        w: u32,
        h: u32,
        hollow: bool,
        color: SdlColor,
    ) -> Result<(), Failed> {
        // let mut color_lookup = HashMap::new();

        // color_lookup.insert(0, Color::RGBA(0, 0, 0, 0));
        // color_lookup.insert(1, Color::WHITE);
        // color_lookup.insert(2, Color::BLACK);

        // let c = color_lookup.get(&color).unwrap();

        self.canvas.set_draw_color(color);

        let r = Rect::new(x, y, w, h);
        if hollow {
            if let Err(_) = self.canvas.draw_rect(r.into()) {
                return Err(Failed::FailedToDrawRect);
            };
        } else {
            if let Err(_) = self.canvas.fill_rect(r) {
                return Err(Failed::FailedToDrawRect);
            }
            if let Err(_) = self.canvas.draw_rect(r.into()) {
                return Err(Failed::FailedToDrawRect);
            };
        }

        return Ok(());
    }
    pub fn present(&mut self) -> bool {
        self.canvas.present()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub relative: PosOrientation,
}

#[derive(Clone, Copy, Debug)]
pub enum PosOrientation {
    TopLeft,
    Center,
}

pub fn init() -> Sdl {
    let sdl_context = sdl3::init().unwrap();
    return sdl_context;
}

pub async fn input(
    mut events: EventPump,
    objects: Arc<Mutex<HashMap<usize, Vec<Box<dyn Draw>>>>>,
    sender: UnboundedSender<Signal>,
) -> Result<(), ()> {
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

pub async fn vis(
    video_subsystem: VideoSubsystem,
    objects: Arc<Mutex<HashMap<usize, Vec<Box<dyn Draw>>>>>,
    mut receiver: UnboundedReceiver<Signal>,
) -> Result<(), ()> {
    let window = video_subsystem
        .window("Mouse", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let canvas = window.into_canvas();
    let mut view = View { canvas };
    view.canvas.set_scale(SCALE as f32, SCALE as f32).unwrap();

    'view: loop {
        if receiver.len() > 0 {
            if let Some(Signal::Quit) = receiver.recv().await {
                break 'view;
            };
        }

        view.canvas.clear();
        {
            let objs = objects.lock().await;
            objs.values().for_each(|layer| {
                // dbg!(layer.len());
                layer.iter().for_each(|o| {
                    o.draw(&mut view);
                });
            });
        };

        if !view.present() {
            break 'view;
        };

        tokio::time::sleep(Duration::from_secs(1 / MAX_FRAME_RATE)).await;
    }

    Ok(())
}
