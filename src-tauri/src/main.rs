// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use elevated_pies::{ Unit, Measurement };
fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![convert_unit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn convert_unit(num: String, from_uom: String, to_uom: String) -> String {
    let dim: f32 = num.trim().parse().unwrap_or_default();
    let unit: Result<Unit, &str> = Unit::new(dim, &from_uom);

    match unit {
        Ok(value) => {
            let measurement: Measurement = Measurement {
                dimension: "height",
                uom: value,
            };
            let conversion: f32 = measurement.to_specific(&to_uom).unwrap_or_default();

            format!("{conversion}")
        }
        Err(value) => String::from(value),
    }
}