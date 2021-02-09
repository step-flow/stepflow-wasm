use std::collections::HashMap;
use stepflow::object::IdError;
use stepflow::data::StateData;
use stepflow::{Session, SessionId, AdvanceBlockedOn, Error};
use stepflow_serde::prelude::*;
use stepflow_serde::{SessionSerde, StateDataSerde};
use wasm_bindgen::prelude::*;
use crate::{WebError, WebAdvanceBlockedOn};


#[wasm_bindgen]
pub struct WebSession {
    session: Session,
}

impl WebSession {
    pub fn new(json: &str, session_id: SessionId) -> Result<WebSession, JsValue> {
        let mut session_serde: SessionSerde = serde_json::from_str(json).map_err(|e| e.to_string())?;
        session_serde.session_id = session_id;
        let session = Session::try_from(session_serde).map_err(|e| WebError::from(e))?;
        Ok(WebSession { session })
    }
}

#[wasm_bindgen]
impl WebSession {
    fn advance_internal(&mut self, state_data: Option<StateData>) -> Result<AdvanceBlockedOn, Error> {
        let current_step = self.session.current_step()?.clone();
        let step_output = state_data.map(|state_data| (&current_step, state_data));

        let result = self.session.advance(step_output);
        result
    }

    pub fn advance(&mut self, state_data: JsValue) -> Result<WebAdvanceBlockedOn, JsValue> {
        let state_data_serde: Option<HashMap<String, String>> = serde_wasm_bindgen::from_value(state_data)?;
        let state_data = state_data_serde.and_then(|data| Some(StateDataSerde::new(data).to_statedata(self.session.var_store()).ok()?));
        self.advance_internal(state_data)
            .and_then(|advance_result| {
                WebAdvanceBlockedOn::try_from(advance_result, &self.session.action_store())
            })
            .map_err(|e| WebError::from(e).into())
    }

    #[wasm_bindgen(method, getter)]
    pub fn statedata(&self) -> Result<JsValue, JsValue> /* HashMap<String, String> */ {
        let var_store = self.session.var_store();
        let result = self.session.state_data().iter_val()
            .map(|(var_id, val)| {
                let name = var_store.name_from_id(var_id).ok_or_else(|| {
                    let error = Error::VarId(IdError::IdMissing(var_id.clone()));
                    WebError::from(error)
                })?;
                Ok((name, val))
            })
            .collect::<Result<HashMap<_,_>, JsValue>>()?;
        Ok(serde_wasm_bindgen::to_value(&result).map_err(|_e| WebError::from(Error::Other))?)
    }

    pub fn vars(&self) -> Result<JsValue, JsValue> /*HashMap<String, u32>*/ {
        let result = self.session.var_store()
            .iter_names()
            .map(|(name, var_id)| { (name.clone(), var_id.val()) })
            .collect::<HashMap<String, u32>>();
        Ok(serde_wasm_bindgen::to_value(&result).map_err(|_e| WebError::from(Error::Other))?)
    }
}
