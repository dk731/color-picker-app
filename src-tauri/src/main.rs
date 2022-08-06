#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use screenshots::Screen;
use std::{fs, time::Instant};

use tauri::{Event, Manager};

fn main() {
    // let displays = Display::all().unwrap_or(vec![]);

    // if displays.len() == 0 {
    //     println!("Was not able to accuqire displays instances");
    // }

    tauri::Builder::default()
        .setup(|app| {
            //
            // let localDisplays = displays;

            app.listen_global("start_color_picker", |event: Event| {
                println!("Received start_color_picker event: {:?}", event.payload());

                let screens = Screen::all().unwrap();

                println!("Received Screens: {}", screens.len());

                for screen in screens {
                    println!("Starting Screen capture {:?}", screen.display_info.id);
                    let mut image = screen.capture().unwrap();

                    let mut buffer = image.buffer();
                    fs::write(
                        format!("{}.png", screen.display_info.id.to_string()),
                        &buffer,
                    )
                    .unwrap();
                    println!("Finished Screen capture {:?}\n", screen.display_info.id);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
