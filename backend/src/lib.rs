mod igdb;

use igdb::client::Client;
use igdb::apicalypse_builder::ApicalypseBuilder;
use igdb::igdb::GameResult;

#[tauri::command]
async fn get_games() {
    // get client + secret from env extracted from dotenv
    let mut client = Client::default();

    let query = ApicalypseBuilder::default()
        .filter("id > 1337")
        .limit(55)
        .offset(66)
        .fields("*")
        .exclude("id,name")
        .sort("id desc");

    let results = client.request::<GameResult>(&query).await.unwrap();
    for game in &results.games {
        println!("{}", game.id);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_games])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
