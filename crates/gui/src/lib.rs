extern crate sdl3;

pub mod ascii;
pub mod shape;

use chrono::Utc;
use sdl3::video::Window;
use sdl3::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas};
use shape::Square;
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use crate::shape::Rectangle;

// any objects that need to be drawn need to be given this
pub trait Draw {
    fn draw(&self, target: &mut View);
}

pub struct Buff(pub Vec<Vec<Option<u8>>>, Position);

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
        color: u8,
    ) -> Result<(), Failed> {
        let mut color_lookup = HashMap::new();

        color_lookup.insert(0, Color::RGBA(0, 0, 0, 0));
        color_lookup.insert(1, Color::WHITE);
        color_lookup.insert(2, Color::BLACK);

        let c = color_lookup.get(&color).unwrap();

        self.canvas.set_draw_color(c.clone());

        let r = Rect::new(x, y, w, h);
        if hollow {
            if let Err(_) = self.canvas.fill_rect(r) {
                return Err(Failed::FailedToDrawRect);
            }
            if let Err(_) = self.canvas.draw_rect(r.into()) {
                return Err(Failed::FailedToDrawRect);
            };
        } else {
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

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub fn run() -> Result<(), ()> {
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

    let mut events = sdl_context.event_pump().unwrap();

    let mut prev_buttons = HashSet::new();
    let mut start_timestamp = Utc::now();
    let mut bg = Square {
        color: 2,
        size: WIDTH,
        hollow: true,
        pos: Position {
            x: -25,
            y: -25,
            relative: PosOrientation::TopLeft,
        },
    };
    let rct = Rectangle {
        color: 2,
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

        let mut objects: Vec<Box<&dyn Draw>> = vec![Box::new(&bg)];

        let sq2 = Square {
            color: 1,
            size: ((Utc::now() - start_timestamp).num_milliseconds()).wrapping_div(100) as u32,
            hollow: true,
            pos: Position {
                x: WIDTH.div_ceil(2) as i32,
                y: HEIGHT.div_ceil(2) as i32,
                relative: PosOrientation::Center,
            },
        };

        match objects.get_mut(1) {
            Some(a) => *a = Box::new(&sq2),
            None => objects.push(Box::new(&sq2)),
        };


        objects.iter().for_each(|o| {
            o.draw(&mut view);
        });

        if !view.present() {
            break 'running;
        };

        std::thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
