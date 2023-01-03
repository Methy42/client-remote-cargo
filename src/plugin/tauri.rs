use tauri::{WindowBuilder, WindowUrl, App, Window};

pub struct Tauri {
    main_tauir_app: Option<App>,
    main_window: Option<Window>
}

pub trait EventTrait {

}

impl Tauri {
    pub fn new() -> Self {
        Self { main_tauir_app: None, main_window: None }
    }

    pub fn init_tauri_app(&mut self) {
        self.main_tauir_app = Some(tauri::Builder::default().build(tauri::generate_context!()).expect("error while building tauri application"));
    }
    pub fn init_main_window(&mut self) {
        let window_url = WindowUrl::App("/main/index.html".into());
        
        match &self.main_tauir_app {
            Some(something) => {
                self.main_window = Some(WindowBuilder::new(
                    something,
                    "external",
                    window_url,
                )
                .title("主窗口")
                .inner_size(640.0, 480.0)
                .position(50.0, 100.0)
                .build().unwrap());
            },
            None => {}
        }

        
    }
}