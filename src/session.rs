use std::error;
use std::collections::HashMap;
use std::convert::TryInto;
use tinyjson::JsonValue;
use stepflow::{Error, SessionId, data::InvalidValue};
use stepflow::object::IdError;
use stepflow_json::{parse_session_json, parse_statedata_json, json_value_from_statedata, StepFlowParseError};
use crate::session_advance_blockedon::advance_blockedon_to_json_obj;

use super::session_store::{new_session, get_session_store, get_session_store_mut};


pub fn create_session(json: &str, allow_implicit_var: bool) -> Result<SessionId, Box<dyn error::Error>> {
  // FUTURE: pre-alloc the session size
  let session_id = new_session()?;
  let mut session_store = get_session_store_mut()?;
  let mut session = session_store.get_mut(&session_id).ok_or_else(|| Error::SessionId(IdError::IdMissing(session_id)))?;
  parse_session_json(&mut session, json, allow_implicit_var)?;
  Ok(session_id)
}

#[derive(Debug)]
pub enum AdvanceSessionError {
  ParseError(StepFlowParseError),
  Error(stepflow::Error),
  Other(Box<dyn std::error::Error>),
}

impl std::fmt::Display for AdvanceSessionError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self) // FUTURE: we can do better
  }
}

impl std::error::Error for AdvanceSessionError {}


fn sessionid_from_val(session_id_val: i32) -> Result<SessionId, Error> {
    let session_id_val: u16 = session_id_val.try_into().map_err(|_| Error::InvalidValue(InvalidValue::WrongValue))?;
    Ok(SessionId::new(session_id_val))
}

pub fn advance_session(session_id_val: i32, step_output_json: Option<&str>) -> Result<HashMap<String, JsonValue>, Box<dyn std::error::Error>>
{
  // get step output data (if any)
  let session_id = sessionid_from_val(session_id_val).map_err(|e| AdvanceSessionError::Error(e))?;
  let mut session_store = get_session_store_mut().map_err(|e| AdvanceSessionError::Other(e))?;
  let session = session_store.get_mut(&session_id).ok_or_else(|| AdvanceSessionError::Error(Error::SessionId(IdError::IdMissing(session_id))))?;
  let state_data = match step_output_json {
      None => None,
      Some(s) => {
          let parsed = parse_statedata_json(s, session.var_store())
              .map_err(|e| AdvanceSessionError::ParseError(e))?;
          Some(parsed)
      }
  };

  let current_step = session
      .current_step()
      .map_err(|e| AdvanceSessionError::Error(e))?
      .clone();
  let step_output = state_data.map(|state_data| {
      (&current_step, state_data)
  });

  let advance_result = session.advance(step_output).map_err(|e| AdvanceSessionError::Error(e))?;
  let block_on_json = advance_blockedon_to_json_obj(&advance_result, session);
  Ok(block_on_json)
}

pub fn get_statedata(session_id_val: i32) -> Result<JsonValue, Box<dyn std::error::Error>> {
    let session_id = sessionid_from_val(session_id_val).map_err(|e| AdvanceSessionError::Error(e))?;
    let session_store = get_session_store()?;
    let session = session_store.get(&session_id).ok_or_else(|| AdvanceSessionError::Error(Error::SessionId(IdError::IdMissing(session_id))))?;
    let json_value = json_value_from_statedata(session.state_data(), session.var_store())?;
    Ok(json_value)
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use tinyjson::JsonValue;
    use stepflow::data::{StringVar, StringValue};
    use stepflow::AdvanceBlockedOn;
    use stepflow::prelude::*;
    use super::create_session;
    use stepflow_json::statedata_from_jsonval_obj;
    use crate::session_store::{get_session_store, get_session_store_mut};

    const JSON: &str = r#"
    {
        "vars": {
            "first_name": "String",
            "last_name": "String",
            "email": "Email",
            "email_waited": "True",
            "nothing": "Bool"
        },
        "steps": {
           "$root": {
               "substeps": ["name", "email"],
               "outputs": ["first_name","last_name","email", "email_waited"]
           },
           "name": {
               "outputs": ["first_name","last_name"]
           },
           "email": {
               "outputs": ["email", "email_waited"]
           }
        },
        "actions": {
            "$all": {
                "type": "UriStringTemplate",
                "template": "/base-path/{{step}}"
            },
            "email": {
                "type": "SetData",
                "stateData": {
                    "email_waited": "true"
                },
                "afterAttempt": 2
            }
        }
    }"#;

    #[test]
    fn deserialize() {
        let session_id = create_session(JSON, false).unwrap();
        let mut session_store = get_session_store_mut().unwrap();
        let session = session_store.get_mut(&session_id).unwrap();
        let name_stepid = session.step_store().get_by_name("name").unwrap().id().clone();
        let email_stepid = session.step_store().get_by_name("email").unwrap().id().clone();
        let _firstname_var_id = session.var_store().get_by_name("first_name").unwrap().id().clone();
        let _email_waited_varid = session.var_store().get_by_name("email_waited").unwrap().id().clone();
        let uri_action_id = session.action_store().id_from_name("$all").unwrap().clone();

        // advance to first step (name)
        let name_advance = session.advance(None).unwrap();
        assert_eq!(name_advance, AdvanceBlockedOn::ActionStartWith(uri_action_id.clone(), "/base-path/name".parse::<StringValue>().unwrap().boxed()));

        // try advancing without setting name and fail
        let name_advance_fail = session.advance(None).unwrap();
        assert_eq!(
            name_advance_fail, 
            AdvanceBlockedOn::ActionStartWith(uri_action_id.clone(), "/base-path/name".parse::<StringValue>().unwrap().boxed()));

        // advance to next step (email) - fail setdata (attempt #1) so get URI action result
        let mut data_name = HashMap::new();
        data_name.insert("first_name".to_owned(), JsonValue::String("billy".to_owned()));
        data_name.insert("last_name".to_owned(), JsonValue::String("bob".to_owned()));
        let statedata_name = statedata_from_jsonval_obj(&data_name, session.var_store()).unwrap();
        let name_advance_success = session.advance(Some((&name_stepid,  statedata_name))).unwrap();
        assert_eq!(name_advance_success, AdvanceBlockedOn::ActionStartWith(uri_action_id.clone(), "/base-path/email".parse::<StringValue>().unwrap().boxed()));

        // put in email and try advancing -- fail setdata (attempt #2) because email waited setdata action hasn't fired so get URI action result
        let mut data_email = HashMap::new();
        data_email.insert("email".to_owned(), JsonValue::String("a@b.com".to_owned()));
        let statedata_email = statedata_from_jsonval_obj(&data_email, session.var_store()).unwrap();
        let name_advance_success = session.advance(Some((&email_stepid,  statedata_email))).unwrap();
        assert_eq!(name_advance_success, AdvanceBlockedOn::ActionStartWith(uri_action_id.clone(), "/base-path/email".parse::<StringValue>().unwrap().boxed()));

        // try advancing again -- success with setdata firing and we're finished
        let name_advance_success = session.advance(None).unwrap();
        assert_eq!(name_advance_success, AdvanceBlockedOn::FinishedAdvancing);
    }

    #[test]
    fn session_ids() {
        let session_id1 = create_session(JSON, false).unwrap();
        let session_id2 = create_session(JSON, false).unwrap();
        assert_ne!(session_id1, session_id2);
    }

    #[test]
    fn implicit_vars() {
        let json = r#"
        {
            "steps": {
                "$root": {
                    "substeps": ["step1"],
                    "outputs": ["test_output"]
                },
                "step1": { "inputs": ["test_input"], "outputs": ["test_output"] }
            },
            "actions": {
                "$all": { "type": "HtmlForm" }
            }
        }
        "#;
        let json = json.to_string();

        // expect error when we don't allow implicit var
        assert!(matches!(create_session(&json[..], false), Err(_)));

        // create session
        let session_id = create_session(&json[..], true).unwrap();
        let session_store = get_session_store().unwrap();
        let session = session_store.get(&session_id).unwrap();

        assert_eq!(session.var_store().iter_names().count(), 2);
        let input_var = session.var_store().get_by_name("test_input").unwrap();
        assert!(input_var.is::<StringVar>());
        let output_var = session.var_store().get_by_name("test_output").unwrap();
        assert!(output_var.is::<StringVar>());
    }
}