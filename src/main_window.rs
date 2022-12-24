use tauri::{WindowBuilder, WindowUrl, App, Window};

pub fn create_main_window(app: &mut App) -> Result<Window, tauri::Error> {
    return WindowBuilder::new(
        app,
        "external",
        WindowUrl::External("http://www.baidu.com/".parse().unwrap()),
    )
    .title("菩提村下的杨过")
    .inner_size(640.0, 480.0)
    .position(50.0, 100.0)
    .build();
}