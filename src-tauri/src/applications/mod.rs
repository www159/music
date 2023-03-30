use std::sync::Mutex;

pub mod netease;

pub struct AppState {
    pub netease_app: Mutex<netease::App>
}