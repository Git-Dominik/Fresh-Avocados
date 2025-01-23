use toml;
use std::{fs::{self, File, OpenOptions}, io::{Read, Write}};
use serde::{Serialize, Deserialize};

use anyhow::{ Error, Result };

// public = gebruik van buitenaf bestand

#[derive(Serialize, Deserialize)]
pub struct LibraryItem {
    pub name: String,
    pub igdb_id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct LibraryFile {
    items: Vec<LibraryItem>,
}

pub struct Library {
    file: LibraryFile,
}

impl Library {
    pub fn new() -> Result<Library, Error> {
        match fs::exists("library.toml") {
            Ok(true) => {
                let mut file = File::open("library.toml").unwrap();
                let mut file_content = String::new();
                file.read_to_string(&mut file_content).unwrap();

                Ok(Library {
                    file: toml::from_str(&file_content).unwrap()
                })
            },
            Ok(false) => {
                let toml_string = toml::to_string(&LibraryFile {
                    items: Vec::new(),
                }).unwrap();

                let mut file = File::create("library.toml").unwrap();
                file.write_all(toml_string.as_bytes()).unwrap();

                Ok(Library {
                    file: LibraryFile {
                        items: Vec::new(),
                    }
                })
            },
            Err(e) => Err(e.into()),
        }
    }

    pub fn add(&mut self, item: LibraryItem) {
        self.file.items.push(item);
        self.save();
    }

    pub fn remove(&mut self, name: String) {
        self.file.items.retain(|item| item.name != name);
        self.save();
    }

    fn save(&self) {
        let mut file = OpenOptions::new().write(true).open("library.toml").unwrap();
        let data = toml::to_string(&self.file).unwrap();
        file.write_all(data.as_bytes()).unwrap();
    }
}