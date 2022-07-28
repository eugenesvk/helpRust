#![allow(non_snake_case,unused_imports,unused_mut,unused_variables,dead_code,uncommon_codepoints)]

use std::any::type_name; // for type_of

mod argAorB;
use crate::argAorB::{arg_a_or_b, arg_str_or_i32};


fn main() {
  arg_a_or_b();
  arg_str_or_i32();
}

fn tests() {
}

fn type_of      <T>(_: T) -> &'static str { type_name::<T>() }
fn print_type_of<T>(_:&T)                 { println!("{}", std::any::type_name::<T>());}
