//! resolve tauri api
pub mod emit;
pub mod netease;
pub mod log;

use std::sync::{Mutex, Arc};
use tauri::Manager;

use self::log::init_log;
use crate::applications;

/// global state managed by tauri
pub struct AppState {
    pub netease_app: Mutex<applications::netease::App>,
    pub netease_service: Mutex<netease::Service>,
    pub emit_service: Mutex<emit::Service>,
}

/// setup applications and services
/// 
/// app:
/// - netease::App
/// 
/// service:
///  - log
///  - emitter
pub fn setup(app: & mut tauri::App) {
    // setup log service
    init_log().unwrap();

    // setup global state
    let emit_service = emit::Service::new(app.app_handle());
    let netease_app = applications::netease::App::new();
    let netease_service = netease::Service::new();

    app.manage(AppState {
        netease_app: Mutex::new(netease_app),
        netease_service: Mutex::new(netease_service),
        emit_service: Mutex::new(emit_service),
    });
}