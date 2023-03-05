use std::{
    error::Error,
    io::stdin,
    ops::Sub,
    thread::sleep,
    time::{Duration, Instant},
};

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

/// This is sooo wasteful. It has 100%+ CPU Consumption for polling, and for event listening
fn _device_query_main() -> () {
    use device_query::{DeviceEvents, DeviceQuery, DeviceState, Keycode, MouseState};
    let device_state = DeviceState::new();
    device_state.on_key_down(|key| println!("Szia: {:?}", key));
    // let _guard = device_state.on_key_down(|key| {
    // println!("Keyboard key down: {:#?}", key);
    // });
    loop {
        // sleep(Duration::from_secs(5));
        let keys: Vec<Keycode> = device_state.get_keys();
        if keys.contains(&Keycode::LControl) && keys.contains(&Keycode::C) {
            // do your handling in here
        }
        sleep(Duration::from_millis(1));
    }
    // Ok(())
}

fn _gilrs_only_for_gamepads_main() {
    env_logger::init();

    let mut gilrs = gilrs_core::Gilrs::new().unwrap();
    loop {
        while let Some(ev) = gilrs.next_event() {
            println!("{:?}", ev);
        }
    }
}

/// Great solution ~0 CPU usage, but only works inside the terminal window
fn _crossterm_main() -> crossterm_input::Result<()> {
    use crossterm_input::{
        input, InputEvent, KeyEvent, MouseButton, MouseEvent, RawScreen, Result,
    };
    // Keep _raw around, raw mode will be disabled on the _raw is dropped
    let _raw = RawScreen::into_raw_mode()?;

    let input = input();
    input.enable_mouse_mode()?;

    let mut sync_stdin = input.read_sync();

    loop {
        if let Some(event) = sync_stdin.next() {
            match event {
                InputEvent::Keyboard(KeyEvent::Esc) => break,
                InputEvent::Keyboard(KeyEvent::Left) => println!("Left arrow"),
                InputEvent::Mouse(MouseEvent::Press(MouseButton::Left, col, row)) => {
                    println!("Left mouse button pressed at {}x{}", col, row);
                }
                _ => println!("Other event {:?}", event),
            }
        }
    }

    input.disable_mouse_mode()
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
