// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(rename_all = "snake_case")]
fn calculate_prices(distance: f64, price: f64, economy: f64, to_fill: f64) -> f64 {
    ((distance / 100f64) * economy + to_fill) * price
}

#[tauri::command(rename_all = "snake_case")]
fn calculate_gas_consumption(distance: f64, economy: f64) -> f64 {
    (distance / 100f64) * economy
}

#[tauri::command(rename_all = "snake_case")]
fn calculate_trip_cost(distance: f64, price: f64, economy: f64) -> f64 {
    ((distance / 100f64) * economy) * price
}

#[tauri::command(rename_all = "snake_case")]
fn calculate_range(fuel_capacity: f64, economy: f64) -> f64 {
    (1f64 / (economy / 100f64)) * fuel_capacity
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            calculate_prices,
            calculate_gas_consumption,
            calculate_trip_cost,
            calculate_range
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
