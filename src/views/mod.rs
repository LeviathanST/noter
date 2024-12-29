use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};

pub mod todo;

#[derive(Serialize, Debug)]
pub struct NoterKeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}
impl<'de> Deserialize<'de> for NoterKeyEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        pub struct RawNoterKeyEvent {
            code: String,
            modifiers: String,
        }

        let raw = RawNoterKeyEvent::deserialize(deserializer)?;

        let code = match raw.code.to_lowercase().as_str() {
            "a" => KeyCode::Char('a'),
            "b" => KeyCode::Char('b'),
            "c" => KeyCode::Char('c'),
            "d" => KeyCode::Char('d'),
            "e" => KeyCode::Char('e'),
            "f" => KeyCode::Char('f'),
            "g" => KeyCode::Char('g'),
            "h" => KeyCode::Char('h'),
            "i" => KeyCode::Char('i'),
            "j" => KeyCode::Char('j'),
            "k" => KeyCode::Char('k'),
            "l" => KeyCode::Char('l'),
            "m" => KeyCode::Char('m'),
            "n" => KeyCode::Char('n'),
            "o" => KeyCode::Char('o'),
            "p" => KeyCode::Char('p'),
            "q" => KeyCode::Char('q'),
            "r" => KeyCode::Char('r'),
            "s" => KeyCode::Char('s'),
            "t" => KeyCode::Char('t'),
            "u" => KeyCode::Char('u'),
            "v" => KeyCode::Char('v'),
            "w" => KeyCode::Char('w'),
            "x" => KeyCode::Char('x'),
            "y" => KeyCode::Char('y'),
            "z" => KeyCode::Char('z'),
            "enter" => KeyCode::Enter,
            "backspace" => KeyCode::Backspace,
            "tab" => KeyCode::Tab,
            "esc" => KeyCode::Esc,
            "space" => KeyCode::Char(' '),
            c => {
                return Err(serde::de::Error::custom(format!(
                    "Unsupported KeyCode: {c}"
                )))
            }
        };

        let modifiers = match raw.modifiers.to_uppercase().as_str() {
            "NONE" => KeyModifiers::NONE,
            "SHIFT" => KeyModifiers::SHIFT,
            "CONTROL" => KeyModifiers::CONTROL,
            "ALT" => KeyModifiers::ALT,
            _ => return Err(serde::de::Error::custom("Unsupported KeyModifiers")),
        };

        Ok(Self { code, modifiers })
    }
}

#[derive(Deserialize, Debug)]
pub struct KeyConfig {
    pub key_binding: HashMap<String, NoterKeyEvent>,
}

pub fn key_match(ev: &KeyEvent, binding: &NoterKeyEvent) -> bool {
    ev.code == binding.code && ev.modifiers == binding.modifiers
}
