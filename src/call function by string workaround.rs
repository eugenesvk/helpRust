#![allow(non_snake_case,unused_imports,unused_mut,unused_variables,dead_code, unused_assignments)]
// https://users.rust-lang.org/t/my-function-name-is-a-string-if-i-want-to-implement-such-a-call-method-how-can-i-implement-it/64588/5
// If you're coming from a javascript background and using something like json to take in method calls from somewhere else, you can do something like this for a fixed number of functions that you would define in the enum. This way though, you would have to predefine the functions that you allow to be called.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "fname", rename_all = "lowercase")]
enum Method {Add{a:u64,b:u64},}
impl Method {
  fn call(&self) -> serde_json::Value {
    match self {Method::Add { a, b } => serde_json::json!(a + b),}
  }}

fn main() {
  let method_str = r#"{"fname":"add","a":1,"b":2}"#;
  let method: Method = serde_json::from_str(method_str).unwrap();
  let result = method.call();
  println!("{:?}", result);
}

// ugly hack
// https://users.rust-lang.org/t/using-a-string-as-a-type-name-in-macros-solved/11988/3
