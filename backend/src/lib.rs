use once_cell::sync::Lazy;
use std::{collections::HashMap, path::PathBuf, sync::Mutex};
use std::process::Command;

mod api;
mod game;

static IGDB: Lazy<api::IGDB> = Lazy::new(api::IGDB::new);
static LIBRARY: Lazy<Mutex<game::LibraryManager>> = Lazy::new(|| Mutex::new(game::LibraryManager::new()));

#[tauri::command]
async fn search_igdb(query: String) -> Vec<api::Game> {
    IGDB.search_games(&query).await
}

#[tauri::command(async)]
async fn get_game_by_id(game_id: u32) -> api::Game {
    IGDB.get_game_by_id(game_id).await.unwrap()
}

#[tauri::command]
async fn cover_igdb(cover_id: u32) -> String {
    IGDB.get_cover(cover_id).await.url
}

#[tauri::command(async)]
async fn get_emulator_paths(app_id: u32) -> HashMap<String, Vec<PathBuf>> {
    game::emulators::get_paths(app_id).await
}

#[tauri::command(async)]
async fn get_installed_games() -> Vec<game::LibraryItem> {
    let library = LIBRARY.lock().unwrap();
    library.file.items.clone()
}

#[tauri::command]
async fn launch_game(game_id: u32) {
    let library = LIBRARY.lock().unwrap();
    let game = library.get(game_id);

    match game {
        Some(game) => {
            println!("Launching game: {}", game.executable);
            let result = Command::new("powershell")
                .args([
                    "-Command",
                    &format!(
                        "Start-Process -FilePath '{}' -Verb RunAs",
                        game.executable
                    ),
                ])
                .spawn();

            match result {
                Ok(_) => println!("Game launched successfully"),
                Err(e) => println!("Failed to launch game: {}", e)
            }
        },
        None => println!("Game not found")
    }
}

#[tauri::command(async)]
async fn add_game(id: u32) {
    let mut library = LIBRARY.lock().unwrap();
    match library.get(id) {
        Some(_) => {
            println!("Game already exists");
            return;
        },
        None => ()
    }

    let file = rfd::FileDialog::new()
        .add_filter("executable file", &["exe"])
        .add_filter("batch file", &["bat"])
        .add_filter("shortcut files", &["ink"])
        .set_directory(".")
        .pick_file();

    match file {
        Some(file) => {
            library.add(game::LibraryItem {
                executable: file.into_os_string().into_string().expect("Failed to convert file name to string"),
                igdb_id: id,
                time_played: 0,
            })
        },
        None => println!("No file selected")
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![search_igdb, cover_igdb, add_game, get_emulator_paths, get_installed_games, get_game_by_id, launch_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
