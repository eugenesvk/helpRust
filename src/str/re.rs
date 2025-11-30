#![allow(dead_code, unused_variables, unused_imports, non_upper_case_globals)]
use helper::alias::*;
use helper::helper::*;

use std::sync::OnceLock;

static REGEX: OnceLock<Regex> = OnceLock::new(); // ensure that the regex is only compiled once

pub fn re_is_hello(text:&str) -> bool {
  let regex = REGEX.get_or_init(|| Regex::new("^hello+$"));
  regex.is_match(text)
}

fn global_state() -> &'static MyState { // accessor function to avoid get_or_init on every call (lazycell allows doing that without an extra function)
  static CELL: OnceCell<MyState> = OnceCell::new();
  CELL.get_or_init(|| {
    // initialization code...
  })
}


use std::sync::OnceLock;
pub fn is_thread_state (       ) -> &'static bool {set_thread_state(false)}
pub fn set_thread_state(is:bool) -> &'static bool { // accessor function to avoid get_or_init on every call (lazycell allows doing that without an extra function)
  static CELL:OnceLock<bool> = OnceLock::new();
  CELL.get_or_init(|| {is})
}
