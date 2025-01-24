use anyhow::Result;
use kira::manager::{AudioManager, AudioManagerSettings, DefaultBackend};
use std::{ops::Not, sync::mpsc::Receiver};
use tauri::{AppHandle, Manager};

use crate::{key_listener::KeyWrapper, state::AppState};

// TODO: remove this
//
// fn _play_sound(
//     stream_handle: &OutputStreamHandle,
//     buf: &Buffered<Decoder<BufReader<File>>>,
//     volume: u16,
// ) -> anyhow::Result<()> {
//     let sink = Sink::try_new(stream_handle)?;
//
//     sink.set_volume(f32::from(volume) * 0.01);
//     sink.append(buf.clone());
//     sink.detach();
//
//     Ok(())
// }

pub fn start(rx: Receiver<KeyWrapper>, handle: AppHandle) -> Result<()> {
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;

    loop {
        if let Ok(key) = rx.recv() {
            let state = handle.state::<AppState>();
            let pack = state.current_pack.read().unwrap();
            let muted = state.muted.read().unwrap();

            let pack = &*pack;

            if muted.not() {
                if let Some(pack) = pack {
                    let sound_data = pack
                        .keys
                        .get(&key.to_lowercase())
                        .unwrap_or_else(|| pack.keys.get("unknown").unwrap());

                    manager.play(sound_data.clone())?;
                }
            }
        }
    }
}
