use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use kira::{
    manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
use rdev::EventType;

mod util_functions;

fn init_listener(mut x: Box<dyn KeyPressHandler>) {
    use rdev::{listen, Event};
    // This will block.

    println!("[Info] Ready to type.");

    if let Err(error) = listen(move |event: Event| {
        match event.event_type {
            EventType::KeyPress(key) => x.handle(key),
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
    fn new<'a>(plugin_path: &'a str) -> Result<Self, String> {
        use std::fs;
        use std::path;

        println!("[Info] Reading configuration...");

        let plugin_base = path::Path::new(plugin_path);

        let plugin_config = plugin_base.join("config.json");

        let requested = fs::read_to_string(&plugin_config)
            .map_err(|_| format!("Config file not found @ {:?}", &plugin_config))?;

        let result = serde_json::from_str::<Layout>(&requested)
            .map_err(|_| "Unable to parse config file!")?;

        let mut map: HashMap<String, StaticSoundData> = HashMap::new();

        for (key, value) in result.defines.iter() {
            if value.is_null() {
                continue;
            }

            let sound_path = if let Some(inner) = value.as_str() {
                if inner.len() == 0 || map.contains_key(key) {
                    // The path was null, but not filtered || Or already registered
                    continue;
                }
                plugin_base.join(inner)
            } else {
                continue;
            };

            let sound = StaticSoundData::from_file(&sound_path, StaticSoundSettings::default())
                .map_err(|error| {
                    format!(
                        "Unable to load Sound file @ {:?}\nError: {}",
                        sound_path, error
                    )
                })?;

            map.insert(key.into(), sound);
        }

        return Ok(PluginHandledKeyPressHandler {
            manager: AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap(),
            sounds: map,
        });
    }
}

impl KeyPressHandler for PluginHandledKeyPressHandler {
    fn handle(&mut self, key: rdev::Key) -> () {
        let key_code = util_functions::key_to_key_code(key);
        if let Some(sound_item) = self.sounds.get(key_code) {
            // Note: Cloning the sound data will not use any extra memory.
            self.manager.play(sound_item.clone()).unwrap();
        }
    }
}

fn main() -> Result<(), String> {
    let vars = std::env::args().collect::<Vec<String>>();

    if vars.len() < 2 {
        println!("Havas Mechanic Keyboard Imitator _ 1.0.0");
        println!("Usage: rust_sound <confuration path>");
        println!("\n\t!! Provide configuration folder path as argument !!\n");
        println!("U can browse sound packs here: https://docs.google.com/spreadsheets/d/1PimUN_Qn3CWqfn-93YdVW8OWy8nzpz3w3me41S8S494");
        return Ok(());
    }

    let config_folder = vars.get(1).unwrap();

    Ok(match PluginHandledKeyPressHandler::new(config_folder) {
        Ok(inner) => {
            init_listener(Box::new(inner));
            ()
        }
        Err(message) => {
            println!("Unable to use the app further.");
            println!("Error occured: {}", message);
            ()
        }
    })
}
