use once_cell::sync::Lazy;

mod api;

static IGDB: Lazy<api::IGDB> = Lazy::new(api::IGDB::new);

#[tauri::command]
async fn search_igdb(query: String) -> Vec<api::Game> {
    IGDB.search_games(&query).await
}

#[tauri::command]
async fn cover_igdb(cover_id: u32) -> String {
    IGDB.get_cover(cover_id).await.url
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![search_igdb, cover_igdb])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
