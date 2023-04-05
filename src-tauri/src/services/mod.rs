//! resolve tauri api
pub mod emit;
pub mod netease;
pub mod log;

use tauri::Manager;

use self::log::init_log;
use crate::applications;

/// global state managed by tauri
pub struct AppState {
    pub netease_app: applications::netease::App,
    pub netease_service: netease::Service,
    pub emit_service: emit::Service,
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
    let mut netease_app = applications::netease::App::new();
    netease_app.load_cookie();

    let netease_service = netease::Service::new();

    app.manage(AppState {
        netease_app: netease_app,
        netease_service: netease_service,
        emit_service: emit_service,
    });
}