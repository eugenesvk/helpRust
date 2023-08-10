#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros)]
use crate::helper::alias::*;
use crate::helper::helper::*;

pub mod helper {
  pub mod alias;
  pub mod helper;
}
// mod argAorB;
// mod fut;
// mod argSliceOrStr;
// mod cliColor;
pub mod fs                           	{ pub mod path; }
// pub mod deftype                   	{ pub mod enum_def_val; }
pub mod tests                        	{ pub mod test_target; }
// pub mod parser                    	{ pub mod chumsky; }
// use crate::argAorB::              	{arg_a_or_b, arg_str_or_i32};
// use crate::fut::                  	{futures_ex};
// use crate::argSliceOrStr::        	{arg_slice_or_str};
// use crate::cliColor::             	{color_cli};
// use crate::cliColor::             	{color_cli};
// use crate::parser::chumsky::      	{chumsky_init};
// use crate::deftype::enum_def_val::	{enum_def_val};
// use crate::tests::test_target::   	{test_target};
pub mod parser                       	{pub mod key;}
use crate::parser::key::             	{key_enum_def_val};

fn main() {
  // arg_a_or_b();
  // arg_str_or_i32();
  // futures_ex();
  // arg_slice_or_str();
  // color_cli();
  // paths();
  // chumsky_init();
  // test_target();
  // enum_def_val();
  key_enum_def_val();
}

fn tests() {
}

