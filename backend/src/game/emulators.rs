use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct EmulatorConfig {
    name: String,
    paths: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    emulators: Vec<EmulatorConfig>,
}

pub async fn get_paths(app_id: u32) -> HashMap<String, Vec<PathBuf>> {
    let config_str: String = reqwest::get("https://pastebin.com/raw/ZNzNF9if")
        .await.unwrap()
        .text()
        .await.unwrap();
    let config: Config = toml::from_str(&config_str)
        .expect("Failed to parse emulator config");

    let mut paths = HashMap::new();

    for emulator in config.emulators {
        paths.insert(
            emulator.name.to_string(),
            emulator.paths.into_iter()
                .map(|p| PathBuf::from(p.replace("{app_id}", &app_id.to_string())))
                .collect()
        );
    }

    println!("{:#?}", paths);

    paths
}
