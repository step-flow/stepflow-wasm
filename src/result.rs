use std::ffi::CString;
use std::collections::HashMap;
use std::convert::TryFrom;
use tinyjson::{JsonValue, JsonGenerateError};

#[derive(Debug)]
pub enum StepFlowResult {
    Err(Box<dyn std::error::Error>),
    Ok(JsonValue),
}

impl From<Result<JsonValue, Box<dyn std::error::Error>>> for StepFlowResult {
    fn from(result: Result<JsonValue, Box<dyn std::error::Error>>) -> Self {
        match result {
            Ok(val) => StepFlowResult::Ok(val),
            Err(e) => StepFlowResult::Err(e),
        }
    }
}

impl From<Result<HashMap<String, JsonValue>, Box<dyn std::error::Error>>> for StepFlowResult {
    fn from(result: Result<HashMap<String, JsonValue>, Box<dyn std::error::Error>>) -> Self {
        match result {
            Ok(val) => StepFlowResult::Ok(JsonValue::Object(val)),
            Err(e) => StepFlowResult::Err(e),
        }
    }
}

impl TryFrom<StepFlowResult> for CString {
    type Error = JsonGenerateError;

    fn try_from(result: StepFlowResult) -> Result<Self, Self::Error> {
        let mut output: HashMap<String, JsonValue> = HashMap::new();
        match result {
            StepFlowResult::Err(e) => {
                output.insert("err".to_owned(), JsonValue::String(e.to_string()));
            }
            StepFlowResult::Ok(json_value) => {
                output.insert("ok".to_owned(), json_value);
            }
        };
        let json_string = JsonValue::Object(output).stringify()?;
        Ok(CString::new(json_string).unwrap())
    }
}


