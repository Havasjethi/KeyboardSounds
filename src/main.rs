use std::{error::Error, io::stdin, ops::Sub, time::Instant};

use kira::{
    manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
use rdev::EventType;

fn play_sound() -> Result<(), Box<dyn Error>> {
    let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default())?;
    let start = Instant::now();
    let sound = StaticSoundData::from_file("./sound.mp3", StaticSoundSettings::default())?;
    let end = Instant::now();

    dbg!(&sound);
    println!(
        "Press enter to play a sound, (total readTime: {:?})",
        end.sub(start)
    );

    let mut some = None;
    loop {
        stdin().read_line(&mut "".into())?;
        let handle = manager.play(sound.clone())?;
        if let Some(mut old) = some.replace(handle) {
            old.stop(kira::tween::Tween::default())?;
        }
    }
}

fn main() -> () {
    use rdev::{listen, Event};
    // This will block.
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }

    fn callback(event: Event) {
        match event.event_type {
            EventType::KeyPress(key) => {
                println!("User wrote {:?}", key);
            }
            _ => (),
        }
    }
}
