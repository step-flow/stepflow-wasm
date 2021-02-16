use std::fmt::Display;
use stepflow::Error;
use stepflow_serde::SerdeError;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct WebError {
    error: SerdeError<serde_json::Error>,
}

#[wasm_bindgen]
impl WebError {
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        format!("{:?}", self.error)
    }
}

impl Display for WebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.error)
    }
}

impl From<SerdeError<serde_json::Error>> for WebError {
    fn from(error: SerdeError<serde_json::Error>) -> Self {
        WebError { error }
    }
}

impl From<Error> for WebError {
    fn from(error: Error) -> Self {
        WebError {
            error: error.into()
        }
    }
}