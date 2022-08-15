#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use base64;
use screenshots::Screen;
use tauri::PhysicalPosition;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
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

const FREEZE_POSITION: WinPoint = WinPoint { x: 300, y: 300 };

static mut IS_FREEZED: bool = false;
static mut BEFORE_FREEZE_MOUSE_POS: WinPoint = WinPoint { x: 0, y: 0 };

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

// Function to freeze mouse cursor
#[tauri::command]
fn freeze_mouse(window: tauri::Window, current_pos: WinPoint) -> Result<WinPoint, tauri::Error> {
    let set_result = window.set_cursor_position(PhysicalPosition {
        x: FREEZE_POSITION.x,
        y: FREEZE_POSITION.y,
    });

    // If error occured do not update state variables
    if set_result.is_err() {
        return Err(set_result.err().unwrap());
    }

    unsafe {
        BEFORE_FREEZE_MOUSE_POS.x = current_pos.x;
        BEFORE_FREEZE_MOUSE_POS.y = current_pos.y;

        IS_FREEZED = true;
    }

    Ok(FREEZE_POSITION)
}

#[tauri::command]
fn freeze_mouse_update(window: tauri::Window) -> Result<(), String> {
    unsafe {
        if !IS_FREEZED {
            return Err("Cannot Get Mouse Movement while mouse is not freezed!".to_string());
        }
    }

    if window
        .set_cursor_position(PhysicalPosition {
            x: FREEZE_POSITION.x,
            y: FREEZE_POSITION.y,
        })
        .is_err()
    {
        return Err("Cannot Get Mouse Movement while mouse is not freezed!".to_string());
    }

    Ok(())
}

// Function to freeze mouse cursor, return base coordinates
#[tauri::command]
fn unfreeze_mouse(window: tauri::Window) {
    unsafe {
        IS_FREEZED = false;
        window
            .set_cursor_position(PhysicalPosition {
                x: BEFORE_FREEZE_MOUSE_POS.x,
                y: BEFORE_FREEZE_MOUSE_POS.y,
            })
            .expect("qwe")
    }
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
        .invoke_handler(tauri::generate_handler![
            start_color_picking,
            freeze_mouse,
            freeze_mouse_update,
            unfreeze_mouse
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
