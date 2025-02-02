#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod library;
use library::Library;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let mut lib = Library::new().unwrap();

    lib.add(library::LibraryItem {
        name: "test".to_string(),
        igdb_id: 123,
    });

    // lib.remove("test".to_string());

    sigma_launcher_lib::run();
}
