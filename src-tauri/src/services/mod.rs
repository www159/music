pub mod emit;
pub mod netease;
pub mod log;

use std::sync::Mutex;
use tauri::Manager;

use self::log::init_log;
use crate::applications;

pub struct AppState {
    pub netease_app: Mutex<applications::netease::App>,
    pub emit_service: Mutex<emit::Service>
}

pub fn setup(app: & mut tauri::App) {
    // setup log service
    init_log().unwrap();

    // setup global state
    let netease_app = applications::netease::App::new();
    let emit_service = emit::Service::new(app.app_handle());
    app.manage(AppState {
        netease_app: std::sync::Mutex::new(netease_app),
        emit_service: std::sync::Mutex::new(emit_service),
    });
}