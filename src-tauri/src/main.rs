#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use base64;
use screenshots::Screen;

#[derive(Debug, serde::Serialize)]
struct WinPoint {
    x: i32,
    y: i32,
}

#[derive(Debug, serde::Serialize)]
struct WinSize {
    width: u32,
    height: u32,
}

#[derive(Debug, serde::Serialize)]
struct StartColorPickerRes {
    display_id: String,
    buffer: String,
    position: WinPoint,
    size: WinSize,
}

#[tauri::command]
async fn start_color_picking() -> Result<Vec<StartColorPickerRes>, String> {
    println!("Received start_color_picker event");

    let screens = Screen::all().unwrap();
    let mut buffer_vec: Vec<StartColorPickerRes> = vec![];

    println!("Received Screens: {}", screens.len());

    for screen in screens {
        println!("Starting Screen capture {:?}", screen.display_info.id);
        let mut image = screen.capture().unwrap();

        let buffer = image.buffer();

        let window_pos = WinPoint {
            x: screen.display_info.x,
            y: screen.display_info.y,
        };

        let window_size = WinSize {
            width: screen.display_info.width,
            height: screen.display_info.height,
        };

        buffer_vec.push(StartColorPickerRes {
            display_id: screen.display_info.id.to_string(),
            buffer: base64::encode(buffer),
            position: window_pos,
            size: window_size,
        });
        println!("Finished Screen capture {:?}\n", screen.display_info.id);
    }

    Ok(buffer_vec)
}

fn main() {
    // let displays = Display::all().unwrap_or(vec![]);

    // if displays.len() == 0 {
    //     println!("Was not able to accuqire displays instances");
    // }

    tauri::Builder::default()
        .setup(|app| {
            //
            // let localDisplays = displays;

            // app.get_window("").unwrap().web

            // app.listen_global("start_color_picker", |event: Event| {});

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![start_color_picking])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
