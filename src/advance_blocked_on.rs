use stepflow::prelude::*;
use stepflow::data::BaseValue;
use stepflow::{ActionObjectStore, AdvanceBlockedOn, Error};
use wasm_bindgen::prelude::*;


fn jsvalue_from_val(val: &Box<dyn Value>) -> JsValue {
  let baseval = val.get_baseval();
  match baseval {
      BaseValue::String(s) => JsValue::from(s),
      BaseValue::Boolean(b) => JsValue::from(b),
      BaseValue::Float(float) => JsValue::from(float),
  }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum WebAdvanceBlockedOnType {
    ActionStartWith,
    ActionCannotFulfill,
    FinishedAdvancing,
}

#[wasm_bindgen]
pub struct WebAdvanceBlockedOn {
    pub blocked_on: WebAdvanceBlockedOnType,
    action_name: Option<String>,
    start_with: Option<Box<dyn Value>>,
}

#[wasm_bindgen]
impl WebAdvanceBlockedOn {
    pub fn action(&self) -> JsValue {
        match &self.action_name {
            None => JsValue::NULL,
            Some(action_name) => action_name.into(),
        }
    }

    pub fn start_with(&self) -> JsValue {
        match &self.start_with {
            None => JsValue::NULL,
            Some(val) => jsvalue_from_val(val),
        }
    }
}

impl WebAdvanceBlockedOn {
  pub fn try_from(advance_result: AdvanceBlockedOn, action_store: &ActionObjectStore) -> Result<WebAdvanceBlockedOn, Error> {
    let result = match advance_result {
        AdvanceBlockedOn::ActionStartWith(action_id, val) => {
            let action_name = action_store.name_from_id(&action_id)?;
            WebAdvanceBlockedOn {
                blocked_on: WebAdvanceBlockedOnType::ActionStartWith,
                action_name: Some(action_name),
                start_with: Some(val),
            }
        }
        AdvanceBlockedOn::ActionCannotFulfill => {
            WebAdvanceBlockedOn {
                blocked_on: WebAdvanceBlockedOnType::ActionCannotFulfill,
                action_name: None,
                start_with: None,
            }
        }
        AdvanceBlockedOn::FinishedAdvancing => {
            WebAdvanceBlockedOn {
                blocked_on: WebAdvanceBlockedOnType::FinishedAdvancing,
                action_name: None,
                start_with: None,
            }
        }
    };
    Ok(result)
  }
}