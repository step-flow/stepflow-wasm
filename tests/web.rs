//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use stepflow_wasm::create_session;

#[wasm_bindgen_test]
fn test_create_session() {
    let json = r#"
    {
        "rootStep": "root",
        "vars": {
          "first_name": "String",
          "last_name": "String",
          "remember": "Bool",
          "email": "Email"
        },
        "steps": {
          "root": {
              "substeps": ["name", "email"],
              "outputVars": ["first_name", "last_name", "email"]
          },
          "name": {
              "outputVars": ["first_name", "last_name", "remember"]
          },
          "email": {
              "outputVars": ["email"]
          }
        },
        "stepActions": {
          "$global": {
              "type": "htmlForm",
              "prefixHtml": "<label for='{{name}}'>{{name}}</label>",
              "stringHtml": "<input name='{{name}}'>",
              "boolHtml": "<input name='{{name}}' type='checkbox'>",
              "wrapTag": "div"
          }
        }
      }"#;

    let session = create_session(json);
    assert!(matches!(session, Ok(_)));
}

#[wasm_bindgen_test]
fn test_create_session_fail() {
    let json = r#"
    {
        "rootStep": "root",
        "vars": {
          "first_name": "Stering",
        },
        "steps": {
          "root": {
              "outputVars": ["first_name"]
          },
        },
        "stepActions": {
          "$global": {
            "type": "url",
            "baseUrl": "/base-path"
          }
        }
    }"#;
    let session = create_session(json);
    assert!(matches!(session, Err(_)));
}  
