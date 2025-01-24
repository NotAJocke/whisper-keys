use std::{sync::mpsc, thread, time::Duration};

use kira::sound::static_sound::StaticSoundData;
use whisper_keys_lib::{audio_manager, key_listener};

fn main() {
    println!("Salut");

    thread::spawn(|| {
        // key_listener::start();
    });

    loop {
        thread::sleep(Duration::from_secs(2));
    }

    // let (tx, rx) = mpsc::channel::<StaticSoundData>();
    //
    // thread::spawn(move || {
    //     audio_manager::start(rx).unwrap();
    // });
    //
    // let data = StaticSoundData::from_file("/Users/jocke/WhisperKeys/Tofu65/altpitch.mp3").unwrap();
    //
    // loop {
    //     tx.send(data.clone()).unwrap();
    //     thread::sleep(Duration::from_secs(1));
    // }
}
