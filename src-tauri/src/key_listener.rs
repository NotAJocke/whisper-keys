use std::{fmt::Display, sync::mpsc::Sender};

use rdev::{EventType, Key};

#[derive(Debug)]
pub struct KeyWrapper(pub rdev::Key);

impl KeyWrapper {
    pub fn to_lowercase(&self) -> String {
        match self.0 {
            Key::Unknown(_) => "unknown".to_string(),
            _ => format!("{self}"),
        }
    }
}

impl Display for KeyWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub fn start(tx: Sender<KeyWrapper>) {
    let mut last_key: Option<Key> = None;
    let mut last_event_type: Option<EventType> = None;

    rdev::listen(move |event| {
        if let EventType::KeyPress(key) = event.event_type {
            if last_key == Some(key) && matches!(last_event_type, Some(EventType::KeyPress(_))) {
               return;
            }

            let wrapped_key = KeyWrapper(key);
            if let Err(e) = tx.send(wrapped_key) {
                eprintln!("Failed to send key from key listener: {e}");
            }

            last_key = Some(key);
        }

        last_event_type = Some(event.event_type);
    })
    .unwrap();
}
