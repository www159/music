//! emit `event` to frontend
//! 
//! encapsulation `emit_to` of [`tauri::AppHandle`]

use std::fmt::Debug;

use serde::Serialize;
use tauri::Manager;

use crate::applications::LOG_TARGET;

pub enum EmitterField {
    MainWindow
}

#[derive(Debug)]
// main window emitter
pub struct Service {
    app_handle: tauri::AppHandle
}

impl Service {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self {
            app_handle
        }
    }

    pub fn emit<S>(&self, field: EmitterField, event: &str, payload: S)
    where S: Serialize + Clone + Debug {
        match field {
            EmitterField::MainWindow => {
                log::debug!(target: LOG_TARGET, "try to emit to `main` window. event: {}, payload: {:#?}", event, payload);
                if let Err(err) =  self.app_handle.emit_to("main", event, payload) {
                    log::error!(target: LOG_TARGET, "failed to emit: {}", err.to_string());
                }

            }
        }
    }
}