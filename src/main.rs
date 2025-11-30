#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros,redundant_semicolons)]
use helper::alias 	::*;
use helper::helper	::*;
use proc_macro    	::*;

#[macro_use] extern crate log;

// mod argAorB;
// mod fut;
// mod argSliceOrStr;
// mod cliColor;
pub mod fs                           	{ pub mod path; }
// pub mod deftype                   	{ pub mod enum_def_val; }
pub mod tests                        	{ pub mod test_target; }
pub mod logging                      	;
// pub mod parser                    	{ pub mod chumsky; }
// use crate::argAorB::              	{arg_a_or_b, arg_str_or_i32};
// use crate::fut::                  	{futures_ex};
// use crate::argSliceOrStr::        	{arg_slice_or_str};
// use crate::cliColor::             	{color_cli};
// use crate::cliColor::             	{color_cli};
// use crate::parser::chumsky::      	{chumsky_init};
// use crate::deftype::enum_def_val::	{enum_def_val};
// use crate::tests::test_target::   	{test_target};
pub mod sync 	;
use sync::                       	{sync};
// use crate::mysync::               	{sync};
// pub mod parser                    	{pub mod key;}
// use crate::parser::key::          	{key_enum_def_val};
use crate::logging::                 	{log_init,log_prints};

use std::process::{ExitCode, Termination};
#[repr(u8)] pub enum CustomExit {
  Ok   	= 0,
  Err  	= 1,
  Skip 	= 125,
  Abort	= 255,}

impl Termination for CustomExit {
  fn report(self) -> ExitCode {
    #[allow(unreachable_patterns)] match self {
      CustomExit::Ok   	=> {},
      CustomExit::Err  	=> p!("🛑 err",),
      CustomExit::Skip 	=> p!("🛑 skip",),
      CustomExit::Abort	=> p!("🛑 abort",),
      _                	=> p!("🛑",),
  };
    ExitCode::from(self as u8)}}

fn main() -> CustomExit {
  log_init();
  // arg_a_or_b();
  // arg_str_or_i32();
  // futures_ex();
  // arg_slice_or_str();
  // color_cli();
  // paths();
  // chumsky_init();
  // test_target();
  // enum_def_val();
  // key_enum_def_val();
  // log_prints();
  // info!("@main info Mary has a little lamb");
  // debug!("@main debug Mary has a little lamb");
  // warn!("@main warn Mary has a little lamb");
  // error!("@main error Mary has a little lamb");
  sync();
  CustomExit::Ok
}

fn tests() {
}

