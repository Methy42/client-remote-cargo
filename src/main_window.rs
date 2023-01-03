use tauri::{WindowBuilder, WindowUrl, App, Window};

pub fn create_main_window(app: &mut App) -> Result<Window, tauri::Error> {
    let mut window_url = WindowUrl::App("/main/index.html".into());
    return WindowBuilder::new(
        app,
        "external",
        window_url,
    )
    .title("主窗口")
    .inner_size(640.0, 480.0)
    .position(50.0, 100.0)
    .build();
}