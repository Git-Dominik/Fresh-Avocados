use toml;
use std::{fs::{self, File}, io::Read};
use serde::{Serialize, Deserialize};

mod achievements;
pub mod emulators;

#[derive(Serialize, Deserialize, Clone)]
pub struct LibraryItem {
    pub executable: String,
    pub igdb_id: u32,
    pub time_played: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LibraryFile {
    #[serde(default)]
    pub items: Vec<LibraryItem>,
}

#[derive(Clone)]
pub struct LibraryManager {
    pub file: LibraryFile,
}

impl LibraryManager {
    pub fn new() -> LibraryManager {
        if let Ok(exists) = fs::exists ("library.toml") {
            if exists {
                let mut file = File::open("library.toml").expect("Failed to open library file");
                let mut file_content = String::new();
                file.read_to_string(&mut file_content).expect("Failed to read library file");

                match toml::from_str(&file_content) {
                    Ok(parsed_file) => LibraryManager { file: parsed_file },
                    Err(e) => {
                        eprintln!("Failed to parse library file, creating new one: {}", e);
                        LibraryManager::create_empty()
                    }
                }
            } else {
                LibraryManager::create_empty()
            }
        } else {
            LibraryManager::create_empty()
        }
    }

    fn save(&self) {
        let toml_string = toml::to_string(&self.file)
            .expect("Failed to serialize library");
        fs::write("library.toml", toml_string)
            .expect("Failed to write library file");
    }

    fn create_empty() -> LibraryManager {
        let library = LibraryManager {
            file: LibraryFile {
                items: Vec::new(),
            }
        };
        library.save();
        library
    }

    pub fn get(&self, igdb_id: u32) -> Option<LibraryItem> {
        self.file.items.iter().find(|item| item.igdb_id == igdb_id).cloned()
    }

    pub fn add(&mut self, item: LibraryItem) {
        self.file.items.push(item);
        self.save();
    }

    pub fn remove(&mut self, igdb_id: u32) {
        self.file.items.retain(|item| item.igdb_id != igdb_id);
        self.save();
    }
}