#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod main_window;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let __main_window = main_window::create_main_window(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
