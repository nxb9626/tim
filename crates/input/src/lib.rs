use std::{collections::HashSet, sync::LazyLock};

use chrono::Utc;
use sdl3::{EventPump, event::Event, keyboard::Scancode, mouse::MouseButton};

use tokio::{
    sync::{
        Mutex,
        mpsc::{UnboundedReceiver, UnboundedSender},
    },
    task::yield_now,
};

pub static SIGNAL_TARGETS: LazyLock<Mutex<Vec<UnboundedSender<Signal>>>> =
    LazyLock::new(|| Mutex::new(vec![]));

#[derive(Clone, Debug)]
pub struct Signal {
    pub keeb: KeebUpdate,
    pub mouse: MouseUpdate,
    pub events: Vec<Event>,
}

#[derive(Clone, Debug)]
pub struct KeebUpdate {
    pub old: HashSet<(Scancode, bool)>,
    pub held: HashSet<Scancode>,
    pub new: HashSet<(Scancode, bool)>,
}

#[derive(Clone, Debug)]
pub struct MouseUpdate {
    pub old: HashSet<MouseButton>,
    pub new: HashSet<MouseButton>,
}

pub async fn get_input_receiver() -> UnboundedReceiver<Signal> {
    let (signal_sender, signal_receiver) = tokio::sync::mpsc::unbounded_channel::<Signal>();
    let mut signals = SIGNAL_TARGETS.lock().await;
    signals.push(signal_sender);

    signal_receiver
}

pub async fn input_loop(mut events: EventPump) -> Result<(), ()> {
    let mut _start_timestamp = Utc::now();
    let mut prev_mouse_buttons = HashSet::new();
    let mut prev_keeb_buttons = HashSet::new();
    let mut held = HashSet::new();

    loop {
        let mut polled_events = Vec::new();
        for e in events.poll_iter() {
            polled_events.push(e);
        }

        // Create a set of pressed Keys.
        let keebstate = events.keyboard_state();
        let keeb_buttons = keebstate.scancodes().collect();

        // Get the difference between the new and old sets.
        let new_keeb_buttons = &keeb_buttons - &prev_keeb_buttons;
        let old_keeb_buttons = &prev_keeb_buttons - &keeb_buttons;

        for (sc, pressed) in &new_keeb_buttons {
            match pressed {
                true => held.insert(sc.clone()),
                false => held.remove(sc),
            };
        }

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
            // println!(
            //     "X = {:?}, Y = {:?} : \n {:?} -> {:?} \n {:?} -> {:?} \n {:?}",
            //     state.x(),
            //     state.y(),
            //     &new_mouse_buttons,
            //     &old_mouse_buttons,
            //     &new_keeb_buttons,
            //     &old_keeb_buttons,
            //     &polled_events
            // );

            let keeb_update = KeebUpdate {
                old: old_keeb_buttons,
                held: held.clone(),
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

            spread_signal(input_update).await;
        }

        prev_mouse_buttons = mouse_buttons;
        prev_keeb_buttons = keeb_buttons;
        yield_now().await;
    }
}

pub async fn spread_signal(signal: Signal) {
    let mut signals = SIGNAL_TARGETS.lock().await;
    signals.iter_mut().for_each(|channel| {
        if let Err(e) = channel.send(signal.clone()) {
            dbg!("input channel send failed: {:?}", e);
        };
    });
}

pub async fn allow_all(s: Signal) -> Option<Signal> {
    Some(s)
}

impl Signal {
    pub fn is_this_key(&self, key: &Scancode) -> bool {
        match self
            .keeb
            .new
            .iter()
            .find(|(a, pressed)| (a, pressed) == (&&key, &true))
        {
            Some(_) => return true,
            None => return false,
        }
    }

    pub fn held_then_pressed(&self, key1: &Scancode, key2: &Scancode) -> bool {
        let held = &self.keeb.held;
        let new = &self.keeb.new;

        held.contains(&key1) && new.contains(&(key2.clone(), true))
    }
}

// pub type Key = Keycode;
pub type SC = Scancode;
