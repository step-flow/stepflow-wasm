#[macro_export]
macro_rules! str_from_cstr {
  ($cstr_ptr:ident) => {
      {
          let s: &str;
          unsafe {
              let data = CStr::from_ptr($cstr_ptr);
              s = data.to_str().unwrap();
          }
          s
      }
  }
}

#[macro_export]
macro_rules! str_from_cstr_or_null {
  ($cstr_ptr:ident) => {
      {
          if $cstr_ptr.is_null() {
              None
          } else {
              Some(str_from_cstr!($cstr_ptr))
          }
      }
  };
}