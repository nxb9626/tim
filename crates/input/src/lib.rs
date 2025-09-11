use std::{collections::HashSet, sync::LazyLock};

use chrono::Utc;
use sdl3::{
    EventPump,
    event::Event,
    keyboard::{Keycode, Scancode},
    mouse::MouseButton,
};

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
    pub fn is_this_key(&self, key: &Keycode) -> bool {
        let scancode = match keycode_to_scancode(&key) {
            Some(s) => s,
            None => {
                dbg!("Checked key doesn't not mapped {:?}", &key);
                return false;
            }
        };

        match self
            .keeb
            .new
            .iter()
            .find(|(a, pressed)| (a, pressed) == (&&scancode, &true))
        {
            Some(_) => return true,
            None => return false,
        }
    }

    pub fn held_then_pressed(&self, key1: &Keycode, key2: &Keycode) -> bool {
        let held = &self.keeb.held;
        let new = &self.keeb.new;

        let first = match keycode_to_scancode(key1) {
            Some(f) => f,
            None => return false,
        };

        let second = match keycode_to_scancode(key2) {
            Some(s) => s,
            None => return false,
        };

        held.contains(&first) && new.contains(&(second, true))
    }
}

pub type Key = Keycode;
fn keycode_to_scancode(key: &Keycode) -> Option<Scancode> {
    match key {
        Keycode::ScancodeMask => None,
        Keycode::Unknown => None,
        Keycode::Return => None,
        Keycode::Escape => Some(Scancode::Escape),
        Keycode::Backspace => None,
        Keycode::Tab => None,
        Keycode::Space => None,
        Keycode::Exclaim => None,
        Keycode::DblApostrophe => None,
        Keycode::Hash => None,
        Keycode::Dollar => None,
        Keycode::Percent => None,
        Keycode::Ampersand => None,
        Keycode::Apostrophe => None,
        Keycode::LeftParen => None,
        Keycode::RightParen => None,
        Keycode::Asterisk => None,
        Keycode::Plus => None,
        Keycode::Comma => None,
        Keycode::Minus => None,
        Keycode::Period => None,
        Keycode::Slash => None,
        Keycode::_0 => None,
        Keycode::_1 => None,
        Keycode::_2 => None,
        Keycode::_3 => None,
        Keycode::_4 => None,
        Keycode::_5 => None,
        Keycode::_6 => None,
        Keycode::_7 => None,
        Keycode::_8 => None,
        Keycode::_9 => None,
        Keycode::Colon => None,
        Keycode::Semicolon => None,
        Keycode::Less => None,
        Keycode::Equals => None,
        Keycode::Greater => None,
        Keycode::Question => None,
        Keycode::At => None,
        Keycode::LeftBracket => None,
        Keycode::Backslash => None,
        Keycode::RightBracket => None,
        Keycode::Caret => None,
        Keycode::Underscore => None,
        Keycode::Grave => None,
        Keycode::A => None,
        Keycode::B => None,
        Keycode::C => None,
        Keycode::D => None,
        Keycode::E => None,
        Keycode::F => None,
        Keycode::G => None,
        Keycode::H => None,
        Keycode::I => None,
        Keycode::J => None,
        Keycode::K => None,
        Keycode::L => None,
        Keycode::M => None,
        Keycode::N => None,
        Keycode::O => None,
        Keycode::P => None,
        Keycode::Q => None,
        Keycode::R => None,
        Keycode::S => None,
        Keycode::T => None,
        Keycode::U => None,
        Keycode::V => None,
        Keycode::W => Some(Scancode::W),
        Keycode::X => None,
        Keycode::Y => None,
        Keycode::Z => None,
        Keycode::LeftBrace => None,
        Keycode::Pipe => None,
        Keycode::RightBrace => None,
        Keycode::Tilde => None,
        Keycode::Delete => None,
        Keycode::PlusMinus => None,
        Keycode::CapsLock => None,
        Keycode::F1 => None,
        Keycode::F2 => None,
        Keycode::F3 => Some(Scancode::F3),
        Keycode::F4 => None,
        Keycode::F5 => None,
        Keycode::F6 => None,
        Keycode::F7 => None,
        Keycode::F8 => None,
        Keycode::F9 => None,
        Keycode::F10 => None,
        Keycode::F11 => None,
        Keycode::F12 => None,
        Keycode::PrintScreen => None,
        Keycode::ScrollLock => None,
        Keycode::Pause => None,
        Keycode::Insert => None,
        Keycode::Home => None,
        Keycode::PageUp => None,
        Keycode::End => None,
        Keycode::PageDown => None,
        Keycode::Right => None,
        Keycode::Left => None,
        Keycode::Down => None,
        Keycode::Up => None,
        Keycode::NumLockClear => None,
        Keycode::KpDivide => None,
        Keycode::KpMultiply => None,
        Keycode::KpMinus => None,
        Keycode::KpPlus => None,
        Keycode::KpEnter => None,
        Keycode::Kp1 => None,
        Keycode::Kp2 => None,
        Keycode::Kp3 => None,
        Keycode::Kp4 => None,
        Keycode::Kp5 => None,
        Keycode::Kp6 => None,
        Keycode::Kp7 => None,
        Keycode::Kp8 => None,
        Keycode::Kp9 => None,
        Keycode::Kp0 => None,
        Keycode::KpPeriod => None,
        Keycode::Application => None,
        Keycode::Power => None,
        Keycode::KpEquals => None,
        Keycode::F13 => None,
        Keycode::F14 => None,
        Keycode::F15 => None,
        Keycode::F16 => None,
        Keycode::F17 => None,
        Keycode::F18 => None,
        Keycode::F19 => None,
        Keycode::F20 => None,
        Keycode::F21 => None,
        Keycode::F22 => None,
        Keycode::F23 => None,
        Keycode::F24 => None,
        Keycode::Execute => None,
        Keycode::Help => None,
        Keycode::Menu => None,
        Keycode::Select => None,
        Keycode::Stop => None,
        Keycode::Again => None,
        Keycode::Undo => None,
        Keycode::Cut => None,
        Keycode::Copy => None,
        Keycode::Paste => None,
        Keycode::Find => None,
        Keycode::Mute => None,
        Keycode::VolumeUp => None,
        Keycode::VolumeDown => None,
        Keycode::KpComma => None,
        Keycode::KpEqualsAs400 => None,
        Keycode::AltErase => None,
        Keycode::SysReq => None,
        Keycode::Cancel => None,
        Keycode::Clear => None,
        Keycode::Prior => None,
        Keycode::Return2 => None,
        Keycode::Separator => None,
        Keycode::Out => None,
        Keycode::Oper => None,
        Keycode::ClearAgain => None,
        Keycode::CrSel => None,
        Keycode::ExSel => None,
        Keycode::Kp00 => None,
        Keycode::Kp000 => None,
        Keycode::ThousandsSeparator => None,
        Keycode::DecimalSeparator => None,
        Keycode::CurrencyUnit => None,
        Keycode::CurrencySubunit => None,
        Keycode::KpLeftParen => None,
        Keycode::KpRightParen => None,
        Keycode::KpLeftBrace => None,
        Keycode::KpRightBrace => None,
        Keycode::KpTab => None,
        Keycode::KpBackspace => None,
        Keycode::KpA => None,
        Keycode::KpB => None,
        Keycode::KpC => None,
        Keycode::KpD => None,
        Keycode::KpE => None,
        Keycode::KpF => None,
        Keycode::KpXor => None,
        Keycode::KpPower => None,
        Keycode::KpPercent => None,
        Keycode::KpLess => None,
        Keycode::KpGreater => None,
        Keycode::KpAmpersand => None,
        Keycode::KpDblAmpersand => None,
        Keycode::KpVerticalBar => None,
        Keycode::KpDblVerticalBar => None,
        Keycode::KpColon => None,
        Keycode::KpHash => None,
        Keycode::KpSpace => None,
        Keycode::KpAt => None,
        Keycode::KpExclam => None,
        Keycode::KpMemStore => None,
        Keycode::KpMemRecall => None,
        Keycode::KpMemClear => None,
        Keycode::KpMemAdd => None,
        Keycode::KpMemSubtract => None,
        Keycode::KpMemMultiply => None,
        Keycode::KpMemDivide => None,
        Keycode::KpPlusMinus => None,
        Keycode::KpClear => None,
        Keycode::KpClearEntry => None,
        Keycode::KpBinary => None,
        Keycode::KpOctal => None,
        Keycode::KpDecimal => None,
        Keycode::KpHexadecimal => None,
        Keycode::LCtrl => None,
        Keycode::LShift => None,
        Keycode::LAlt => None,
        Keycode::LGui => Some(Scancode::LGui),
        Keycode::RCtrl => None,
        Keycode::RShift => None,
        Keycode::RAlt => None,
        Keycode::RGui => None,
        Keycode::Mode => None,
        Keycode::Sleep => None,
        Keycode::Wake => None,
        Keycode::ChannelIncrement => None,
        Keycode::ChannelDecrement => None,
        Keycode::MediaPlay => None,
        Keycode::MediaPause => None,
        Keycode::MediaRecord => None,
        Keycode::MediaFastForward => None,
        Keycode::MediaRewind => None,
        Keycode::MediaNextTrack => None,
        Keycode::MediaPreviousTrack => None,
        Keycode::MediaStop => None,
        Keycode::MediaEject => None,
        Keycode::MediaPlayPause => None,
        Keycode::MediaSelect => None,
        Keycode::AcNew => None,
        Keycode::AcOpen => None,
        Keycode::AcClose => None,
        Keycode::AcExit => None,
        Keycode::AcSave => None,
        Keycode::AcPrint => None,
        Keycode::AcProperties => None,
        Keycode::AcSearch => None,
        Keycode::AcHome => None,
        Keycode::AcBack => None,
        Keycode::AcForward => None,
        Keycode::AcStop => None,
        Keycode::AcRefresh => None,
        Keycode::AcBookmarks => None,
        Keycode::SoftLeft => None,
        Keycode::SoftRight => None,
        Keycode::Call => None,
        Keycode::EndCall => None,
    }
}
