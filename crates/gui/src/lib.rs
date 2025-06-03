extern crate sdl3;

pub mod ascii;
pub mod shape;

use crate::shape::Rectangle;
use chrono::Utc;
use sdl3::pixels::Color as SdlColor;
use sdl3::video::Window;
use sdl3::{event::Event, keyboard::Keycode, rect::Rect, render::Canvas};
use shape::Square;
use std::collections::HashMap;
use std::sync::Arc;
use std::{collections::HashSet, time::Duration};
use tokio::sync::Mutex;
use tokio::task::yield_now;

const MAX_FRAME_RATE: u64 = 240;
const SCALE: u32 = 2;

pub const WIDTH: u32 = 640 * SCALE;
pub const HEIGHT: u32 = 480 * SCALE;

pub type Color = SdlColor;

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

pub async fn run(objects: Arc<Mutex<HashMap<usize, Vec<Box<dyn Draw>>>>>) -> Result<(), ()> {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Mouse", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    let canvas = window.into_canvas();
    let mut view = View { canvas };
    view.canvas.set_scale(SCALE as f32, SCALE as f32).unwrap();

    let mut events = sdl_context.event_pump().unwrap();

    yield_now().await;

    // let mut prev_buttons = HashSet::new();
    let mut start_timestamp = Utc::now();
    let mut bg = Square {
        color: Color::WHITE,
        size: WIDTH,
        hollow: true,
        pos: Position {
            x: -25,
            y: -25,
            relative: PosOrientation::TopLeft,
        },
    };
    let rct = Rectangle {
        color: Color::WHITE,
        width: 50,
        height: 500,
        hollow: true,
        pos: Position {
            x: -25,
            y: -25,
            relative: PosOrientation::TopLeft,
        },
    };

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    start_timestamp = Utc::now();
                }
                _ => {}
            }
        }

        // get a mouse state
        let state = events.mouse_state();

        // Create a set of pressed Keys.
        // let buttons = state.pressed_mouse_buttons().collect();

        // Get the difference between the new and old sets.
        // let new_buttons = &buttons - &prev_buttons;
        // let old_buttons = &prev_buttons - &buttons;

        // if !new_buttons.is_empty() || !old_buttons.is_empty() {
        //     println!(
        //         "X = {:?}, Y = {:?} : {:?} -> {:?}",
        //         state.x(),
        //         state.y(),
        //         new_buttons,
        //         old_buttons
        //     );
        // }
        // prev_buttons = buttons;
        view.canvas.clear();
        {
            let objs = objects.lock().await;
            objs.values().for_each(|layer| {
                dbg!(layer.len());
                layer.iter().for_each(|o| {
                    o.draw(&mut view);
                });
            });
        };
        if !view.present() {
            break 'running;
        };

        tokio::time::sleep(Duration::from_secs(1 / MAX_FRAME_RATE)).await;
        // yield_now().await;
    }

    Ok(())
}
