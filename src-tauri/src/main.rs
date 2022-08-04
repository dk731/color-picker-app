#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

extern crate scrap;

use scrap::Display;
use tauri::{Event, Manager};

fn on_color_picker_start(event: Event) {
    println!("Received start_color_picker event: {:?}", event.payload());
}

fn main() {
    let displays = Display::all().unwrap_or(vec![]);

    if displays.len() == 0 {
        println!("Was not able to accuqire displays instances");
    }

    for (i, display) in displays.iter().enumerate() {
        println!(
            "Display {} [{}x{}]",
            i + 1,
            display.width(),
            display.height()
        );
    }

    tauri::Builder::default()
        .setup(|app| {
            //

            app.listen_global("start_color_picker", on_color_picker_start);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
