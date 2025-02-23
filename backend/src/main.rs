#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    sigma_launcher_lib::run();
}
