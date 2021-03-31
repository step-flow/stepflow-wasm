use std::collections::HashMap;
use tinyjson::JsonValue;
use stepflow::{AdvanceBlockedOn, Session};
use stepflow::action::{SetDataAction, HtmlFormAction, StringTemplateAction, UriEscapedString, HtmlEscapedString};
use stepflow_json::json_value_from_val;


pub fn advance_blockedon_to_json_obj(advance_blockedon: &AdvanceBlockedOn, session: &Session) -> HashMap<String, JsonValue> {
  let mut block_on_json: HashMap<String, JsonValue> = HashMap::new();
  match advance_blockedon {
      AdvanceBlockedOn::ActionStartWith(action_id, val) => {
          block_on_json.insert("blockedOn".to_owned(), JsonValue::String("StartWith".to_owned()));
          if let Some(name) = session.action_store().name_from_id(&action_id) {
              block_on_json.insert("actionName".to_owned(), JsonValue::String(name.to_owned()));
          }

          if let Some(action) = session.action_store().get(&action_id) {
              let action_type = {
                if action.is::<StringTemplateAction<UriEscapedString>>() { "UriStringTemplate" }
                else if action.is::<StringTemplateAction<HtmlEscapedString>>() { "HtmlStringTemplate" }
                else if action.is::<SetDataAction>() { "SetData" }
                else if action.is::<HtmlFormAction>() { "HtmlForm" }
                else { "other" }
              };
              block_on_json.insert("actionType".to_owned(), JsonValue::String(action_type.to_owned()));
          }
          block_on_json.insert("startWith".to_owned(), json_value_from_val(&val));
      }
      AdvanceBlockedOn::ActionCannotFulfill => {
          block_on_json.insert("blockedOn".to_owned(), JsonValue::String("CannotFulfill".to_owned()));
      }
      AdvanceBlockedOn::FinishedAdvancing => {
          block_on_json.insert("blockedOn".to_owned(), JsonValue::String("FinishedAdvancing".to_owned()));
      }
  };

  block_on_json
}