// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use elevated_pies::{ Unit, Measurement };
use tauri::State;
use std::{ sync::{ Arc, Mutex }};

#[derive(Default)]
struct Dimension(Arc<Mutex<f32>>);

fn main() {
    tauri::Builder
        ::default()
        .manage(Dimension(Default::default()))
        .invoke_handler(tauri::generate_handler![convert_unit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn convert_unit(uom: String, num: String, tracker: State<'_, Dimension>) -> String {
    let mut val = tracker.0.lock().unwrap();
    let dim: f32 = num.trim().parse().unwrap_or_default();
    let unit: Result<Unit, &str> = Unit::new(dim, &uom);

    match unit {
        Ok(value) => {
            let measurement: Measurement = Measurement {
                dimension: "height",
                uom: value,

            };

            let c_unit: f32 = measurement
                .to_centimeter()
                .unwrap_or_else(|error| { panic!("{}", error) });
            *val = dim;
            format!("{c_unit}")
        }
        Err(value) => String::from(value),
    }
}