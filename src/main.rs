#![allow(non_snake_case,unused_imports,unused_mut,unused_variables,dead_code,uncommon_codepoints)]

use std::any::type_name; // for type_of

mod argAorB;
use crate::argAorB::{arg_a_or_b, arg_str_or_i32};
mod fut;
use crate::fut::    	{futures_ex};
mod argSliceOrStr;
use crate::argSliceOrStr::	{arg_slice_or_str};


fn main() {
  arg_a_or_b();
  arg_str_or_i32();
  futures_ex();
  arg_slice_or_str();
}

fn tests() {
}

fn type_of      <T>(_: T) -> &'static str { type_name::<T>() }
fn print_type_of<T>(_:&T)                 { println!("{}", std::any::type_name::<T>());}
