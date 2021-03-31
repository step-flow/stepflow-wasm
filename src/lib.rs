use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_void};
use tinyjson::JsonValue;

mod cstr;
mod session_store;
mod session_advance_blockedon;
mod result;
use result::StepFlowResult;


/*
** GLOBAL ALLOCATOR
 */

extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/*
** SESSION
 */

mod session;
use session::{create_session, advance_session, get_statedata};

// FUTURE: can we remove these unwrap() calls?

#[no_mangle]
pub extern fn createSession(data: *mut c_char) -> *mut c_char {
  let json = str_from_cstr!(data);
  let session_id = create_session(json, true);
  let result = match session_id {
      Ok(id) => StepFlowResult::Ok(JsonValue::Number(id.val().into())),
      Err(e) => StepFlowResult::Err(e),
  };
  let cstring = CString::try_from(result).unwrap();
  cstring.into_raw()
}

#[no_mangle]
pub extern fn advanceSession(session_id_val: i32, step_output_json: *mut c_char) -> *mut c_char {
  let result: StepFlowResult = advance_session(session_id_val, str_from_cstr_or_null!(step_output_json)).into();
  let cstring = CString::try_from(result).unwrap();
  cstring.into_raw()
}

#[no_mangle]
pub extern fn getStateData(session_id_val: i32) -> *mut c_char {
    let result: StepFlowResult = get_statedata(session_id_val).into();
    let cstring = CString::try_from(result).unwrap();
    cstring.into_raw()
}


/*
** MEMORY
 */

#[no_mangle]
pub extern fn alloc(num_bytes: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(num_bytes);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    ptr
}

#[no_mangle]
pub unsafe extern fn dealloc(ptr: *mut c_void, num_bytes: usize) {
    let _ = Vec::from_raw_parts(ptr, 0, num_bytes);
}

#[no_mangle]
pub extern fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}
