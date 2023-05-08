// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use elevated_pies::{ Distance, Volume};
fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![convert_distance,convert_volume])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn convert_distance(num: String, from_uom: String, to_uom: String) -> String {
    let dim: f32 = num.trim().parse().unwrap_or_default();
    let unit: Result<Distance, &str> = Distance::new(dim, &from_uom);

    match unit {
        Ok(distance) => {
            let conversion: f32 = distance.to_specific(&to_uom).unwrap_or_default();

            format!("{conversion}")
        }
        Err(value) => String::from(value),
    }
}

#[tauri::command]
fn convert_volume(num: String, from_uom: String, to_uom: String) -> String {
    let dim: f32 = num.trim().parse().unwrap_or_default();
    let unit: Result<Volume, &str> = Volume::new(dim, &from_uom);

    match unit {
        Ok(distance) => {
            let conversion: f32 = distance.to_specific(&to_uom).unwrap_or_default();

            format!("{conversion}")
        }
        Err(value) => String::from(value),
    }
}