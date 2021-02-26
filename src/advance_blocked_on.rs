use stepflow::prelude::*;
use stepflow::object::{ObjectStore, IdError};
use stepflow::action::{ActionId, HtmlFormAction, SetDataAction};
use stepflow::data::BaseValue;
use stepflow::{AdvanceBlockedOn, Error};
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
#[derive(Copy, Clone)]
pub enum ActionType {
//    StringTemplate,   TODO: support this
    SetData,
    HtmlForm,
    Other,
}

impl From<&Box<dyn Action + Sync + Send>> for ActionType {
    fn from(action: &Box<dyn Action + Sync + Send>) -> Self {
//        if action.is::<StringTemplateAction>() {
//            ActionType::StringTemplate
//        } else
        if action.is::<SetDataAction>() {
            ActionType::SetData
        } else if action.is::<HtmlFormAction>() {
            ActionType::HtmlForm
        } else {
            ActionType::Other
        }
    }
}

#[wasm_bindgen]
pub struct WebAdvanceBlockedOn {
    #[wasm_bindgen(js_name = blockedOn)]
    pub blocked_on: WebAdvanceBlockedOnType,
    action_name: Option<String>,
    #[wasm_bindgen(js_name = actionType)]
    pub action_type: Option<ActionType>,
    start_with: Option<Box<dyn Value>>,
}

#[wasm_bindgen]
impl WebAdvanceBlockedOn {
    // implement getter to use different name
    #[wasm_bindgen(method, getter)]
    pub fn action(&self) -> JsValue {
        match &self.action_name {
            None => JsValue::NULL,
            Some(action_name) => action_name.into(),
        }
    }

    #[wasm_bindgen(method, getter)]
    pub fn start_with(&self) -> JsValue {
        match &self.start_with {
            None => JsValue::NULL,
            Some(val) => jsvalue_from_val(val),
        }
    }
}

impl WebAdvanceBlockedOn {
  pub fn try_from(advance_result: AdvanceBlockedOn, action_store: &ObjectStore<Box<dyn Action + Sync + Send>, ActionId>) -> Result<WebAdvanceBlockedOn, Error> {
    let result = match advance_result {
        AdvanceBlockedOn::ActionStartWith(action_id, val) => {
            let action = action_store.get(&action_id).ok_or_else(|| Error::ActionId(IdError::IdMissing(action_id)))?;
            let action_type = ActionType::from(action);
            let action_name = action_store.name_from_id(&action_id).ok_or_else(|| Error::ActionId(IdError::IdMissing(action_id)))?;
            WebAdvanceBlockedOn {
                blocked_on: WebAdvanceBlockedOnType::ActionStartWith,
                action_name: Some(action_name.to_owned()),
                action_type: Some(action_type),
                start_with: Some(val),
            }
        }
        AdvanceBlockedOn::ActionCannotFulfill => {
            WebAdvanceBlockedOn {
                blocked_on: WebAdvanceBlockedOnType::ActionCannotFulfill,
                action_name: None,
                action_type: None,
                start_with: None,
            }
        }
        AdvanceBlockedOn::FinishedAdvancing => {
            WebAdvanceBlockedOn {
                blocked_on: WebAdvanceBlockedOnType::FinishedAdvancing,
                action_name: None,
                action_type: None,
                start_with: None,
            }
        }
    };
    Ok(result)
  }
}