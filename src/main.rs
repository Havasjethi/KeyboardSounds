use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Instant;

mod util_functions;

use kira::{
    manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
use rdev::EventType;

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
        let plugin_base = path::Path::new(plugin_path);

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
        let key_code = util_functions::keyToKeyCode(key);
        let valami: StaticSoundData = self.sounds.get(key_code).unwrap().clone();
        self.manager.play(valami).unwrap();
    }
}

fn main() -> () {
    let vars = std::env::args().collect::<Vec<String>>();

    if vars.len() < 2 {
        println!("Havas-KeyLogger 0.1.0");
        println!("Usage: rust_sound <confuration path>");
        println!("\n\t!! Provide configuration folder path as argument !!\n");
        println!("U can browse sound packs here: https://docs.google.com/spreadsheets/d/1PimUN_Qn3CWqfn-93YdVW8OWy8nzpz3w3me41S8S494");
        return;
    }

    let config_folder = vars.get(1).unwrap();
    let x = PluginHandledKeyPressHandler::new(config_folder);

    init_listener(Box::new(x));
}
