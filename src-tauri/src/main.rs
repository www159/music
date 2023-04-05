// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


mod applications;
mod services;
mod utils;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {

    // FIXME use `parking_lot` primitives
    let builder = tauri::Builder::default()
        .setup(|app| Ok(services::setup(app)))
        .invoke_handler(tauri::generate_handler![
            greet,
            services::netease::create_qrcode_session,
            services::netease::abort_qrcode_session,
            services::netease::get_qrcode,
            services::netease::list_playlist,
            services::netease::test_cookie,
            services::netease::test_cookie_load,
        ]);

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|app_hander, event| match event {
        _ => {}
    });
}
