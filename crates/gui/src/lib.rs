extern crate sdl3;

pub mod shape;
pub mod text;

use chrono::Utc;
use sdl3::pixels::Color as SdlColor;
use sdl3::render::FRect;
use sdl3::video::Window;
use sdl3::{EventPump, Sdl, VideoSubsystem};
use sdl3::{event::Event, keyboard::Keycode, render::Canvas};
use std::collections::HashMap;
use std::sync::Arc;
use std::{collections::HashSet, time::Duration};
use tokio::sync::Mutex;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::yield_now;

use crate::shape::{Pixel, Rectangle, Square};
use crate::text::Text;

const MAX_FRAME_RATE: u64 = 240;
const SCALE: f32 = 1.0;

pub const WIDTH: f32 = 800.0 * SCALE;
pub const HEIGHT: f32 = 600.0 * SCALE;

pub enum Objects {
    Pixel(Pixel),
    Rectangle(Rectangle),
    Square(Square),
    Text(Text),
}

#[derive(Debug, Copy, Clone)]
pub enum Color {
    WHITE,
    BLACK,
    CYAN,
    PINK,
}

impl Into<SdlColor> for Color {
    fn into(self) -> SdlColor {
        match self {
            Color::WHITE => SdlColor::WHITE,
            Color::BLACK => SdlColor::BLACK,
            Color::CYAN => SdlColor::CYAN,
            Color::PINK => SdlColor::MAGENTA,
        }
    }
}

pub enum Signal {
    Quit,
}

pub struct Object;

// Any objects that need to be drawn need to be given this
pub trait Draw {
    fn draw(&self, target: &mut View);
}

pub struct View {
    canvas: Canvas<Window>,
}

#[derive(Clone, Copy, Debug)]
pub enum Failed {
    FailedToDrawRect,
    RenderText,
}

impl View {
    pub fn try_draw_rect(
        &mut self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        hollow: bool,
        color: impl Into<SdlColor>,
    ) -> Result<(), Failed> {
        self.canvas.set_draw_color(color);

        let r = FRect::new(x, y, w, h);

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
    pub x: f32,
    pub y: f32,
    pub relative: PosOrientation,
}

pub type Pos = Position;

impl Position {
    pub fn at(x: f32, y: f32) -> Self {
        Position {
            x: x,
            y: y,
            relative: PosOrientation::Center,
        }
    }
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
    _objects: Arc<Mutex<HashMap<usize, Vec<Objects>>>>,
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
    objects: Arc<Mutex<HashMap<usize, Vec<Objects>>>>,
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
            100.0,
        )
        .unwrap();

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
            let mut x: Vec<&usize> = objs.keys().collect();
            x.sort();
            x.iter().for_each(|k| {
                let olayer = objs.get(k);
                match olayer {
                    Some(layer) => {
                        layer.iter().for_each(|o| {
                            match o {
                                Objects::Square(w) => w.draw(&mut view),
                                Objects::Pixel(w) => w.draw(&mut view),
                                Objects::Rectangle(rectangle) => rectangle.draw(&mut view),
                                Objects::Text(text) => {
                                    if let Err(e) = text.draw(&mut view, &mut font) {
                                        dbg!("Failed to write text:{:?}", e);
                                    }
                                }
                            };
                        });
                    }
                    None => {}
                }
            });
        };

        if !view.present() {
            break 'view;
        };

        tokio::time::sleep(Duration::from_secs(1 / MAX_FRAME_RATE)).await;
    }

    Ok(())
}
