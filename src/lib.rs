mod utils;
use std::sync::RwLock;
use once_cell::sync::OnceCell;
use stepflow::object::ObjectStore;
use stepflow::{Session, SessionId};
use stepflow_serde::SerdeError;
use wasm_bindgen::prelude::*;

mod session;
use session::WebSession;

mod error;
use error::WebError;

mod advance_blocked_on;
use advance_blocked_on::WebAdvanceBlockedOn;

static SESSIONS: OnceCell<RwLock<ObjectStore<Session, SessionId>>> = OnceCell::new();

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
}

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen]
pub fn set_panic_hook() {
    utils::set_panic_hook();
}

#[wasm_bindgen(js_name = createSession)]
pub fn create_session(json: &str) -> Result<WebSession, JsValue> {
    let _ = SESSIONS.set(RwLock::new(ObjectStore::with_capacity(1)));   // result doesn't matter in this case
    let session_id = SESSIONS.get()
        .ok_or_else(|| WebError::from(SerdeError::Other))?
        .write()
        .map_err(|_e| WebError::from(SerdeError::Other))?
        .reserve_id();
    
    WebSession::new(json, session_id)
}
