use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::ops::Add;
use std::{default, error::Error, io::stdin, ops::Sub, time::Instant};

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

fn init_listener(mut x: Box<dyn KeyPressHandler>) {
    use rdev::{listen, Event};
    // This will block.
    if let Err(error) = listen(move |event: Event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                x.handle(key);
            }
            _ => (),
        };
    }) {
        println!("Error: {:?}", error)
    }
}

trait KeyPressHandler {
    fn handle(&mut self, key: rdev::Key) -> ();
}

struct PluginHandledKeyPressHandler {
    manager: AudioManager<CpalBackend>,
    sounds: HashMap<String, StaticSoundData>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Layout {
    defines: HashMap<String, Value>,
}

impl PluginHandledKeyPressHandler {
    fn new<'a>(plugin_path: &'a str) -> Self {
        use std::fs;
        use std::path;
        let bc = format!("plugins/{}", plugin_path);
        let plugin_base = path::Path::new("plugins");
        let plugin_base = plugin_base.join(plugin_path);

        let plugin_config = plugin_base.join("config.json");

        let requested =
            fs::read_to_string(&plugin_config).expect(&format!("Missing: {:?}", plugin_config));
        let result = serde_json::from_str::<Layout>(&requested).unwrap();
        dbg!(&result);

        let mut map: HashMap<String, StaticSoundData> = HashMap::new();
        for (key, value) in result.defines.iter() {
            if value.is_null() {
                continue;
            }

            let sound_path = if let Some(inner) = value.as_str() {
                if inner.len() == 0 {
                    // The path was null, but not filtered
                    continue;
                }
                plugin_base.join(inner)
            } else {
                continue;
            };

            let sound =
                StaticSoundData::from_file(&sound_path, StaticSoundSettings::default()).unwrap();
            let start = Instant::now();
            print!("Insert new sound: {:?} \t ...", &sound_path);
            println!("({:?})", Instant::now() - start);
            map.insert(key.into(), sound);
        }

        return PluginHandledKeyPressHandler {
            manager: AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap(),
            sounds: map,
        };
    }
}

impl KeyPressHandler for PluginHandledKeyPressHandler {
    fn handle(&mut self, key: rdev::Key) -> () {
        use rdev::Key;
        let x = match (key) {
            // TODO :: Finish: https://github.com/hainguyents13/mechvibes/blob/master/src/libs/keycodes.js#L99P
            Key::F1 => "59",
            Key::F2 => "60",
            Key::F3 => "61",
            Key::F4 => "62",
            Key::F5 => "63",
            Key::F6 => "64",
            Key::F7 => "65",
            Key::F8 => "66",
            Key::F9 => "67",
            Key::F10 => "68",
            Key::F11 => "87",
            Key::F12 => "88",
            Key::Num1 => "1",
            Key::Num2 => "2",
            Key::Num3 => "3",
            Key::Num4 => "4",
            Key::Num5 => "5",
            Key::Num6 => "6",
            Key::Num7 => "7",
            Key::Num8 => "8",
            Key::Num9 => "9",
            Key::KeyA => "30",
            Key::KeyB => "48",
            Key::KeyC => "46",
            Key::KeyD => "32",
            Key::KeyE => "18",
            Key::KeyF => "33",
            Key::KeyG => "34",
            Key::KeyH => "35",
            Key::KeyI => "23",
            Key::KeyJ => "36",
            Key::KeyK => "37",
            Key::KeyL => "38",
            Key::KeyM => "50",
            Key::KeyN => "49",
            Key::KeyO => "24",
            Key::KeyP => "25",
            Key::KeyQ => "16",
            Key::KeyR => "19",
            Key::KeyS => "31",
            Key::KeyT => "20",
            Key::KeyU => "22",
            Key::KeyV => "47",
            Key::KeyW => "17",
            Key::KeyX => "45",
            Key::KeyY => "21",
            Key::KeyZ => "44",
            Key::Escape => "28",
            Key::Space => "57",
            Key::UpArrow => "57416",
            Key::DownArrow => "57419",
            Key::RightArrow => "57421",
            Key::LeftArrow => "57424",
            // Key::Key; => "48",
            _ => "1",
        };
        println!("Szias {:?}", &key);
        let valami = self.sounds.get(x).unwrap();
        self.manager.play(valami.clone()).unwrap();
    }
}

fn main() -> () {
    // let x = PluginHandledKeyPressHandler::new("custom-sexy-voice");
    let x = PluginHandledKeyPressHandler::new("kalih-box-white");
    init_listener(Box::new(x));
}
