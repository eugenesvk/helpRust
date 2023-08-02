#![allow(non_snake_case,unused_imports,unused_mut,unused_variables,dead_code,uncommon_codepoints)]

use std::any::type_name; // for type_of

mod argAorB;
mod fut;
mod argSliceOrStr;
use crate::argAorB::      	{arg_a_or_b, arg_str_or_i32};
use crate::fut::          	{futures_ex};
use crate::argSliceOrStr::	{arg_slice_or_str};


fn main() {
  // arg_a_or_b();
  // arg_str_or_i32();
  // futures_ex();
  arg_slice_or_str();
}

fn tests() {
}

