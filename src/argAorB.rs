#![allow(non_snake_case,unused_imports,unused_mut,unused_variables,dead_code, unused_assignments)]

// Limit a generic type to be either A or B [src](https://users.rust-lang.org/t/limit-a-generic-type-to-be-either-a-or-b/66367/8). Possible to do this with
  // an enum and
  // From/Into and
  // a sealed trait

#[derive(Debug)] pub struct 𝚻Alpha{a:i32}
#[derive(Debug)] pub struct 𝚻Beta {b:i32}
#[derive(Debug)] pub enum   EnAlphaOrBeta { EnAlpha(𝚻Alpha), EnBeta(𝚻Beta),}

impl From<𝚻Alpha> for EnAlphaOrBeta { fn from(arg_𝛂:𝚻Alpha) -> Self { Self::EnAlpha(arg_𝛂)} }
impl From<𝚻Beta > for EnAlphaOrBeta { fn from(arg_𝛃:𝚻Beta ) -> Self { Self::EnBeta (arg_𝛃 ) }}

mod sealed {
  pub trait Sealed {}
  impl Sealed for super::𝚻Alpha {}
  impl Sealed for super::𝚻Beta {}
}

pub fn foo<T: Into<EnAlphaOrBeta> + sealed::Sealed>(x: T) {
  let x = x.into();
  println!("x={:?}", x);
  // println!("x={:?},\nx.a={}", x, x.a);
  // rest of code here
}
// foo(Alpha) and foo(Beta) both work in addition to passing the enum.
// Nothing else can work due to the sealed trait.

pub fn arg_a_or_b() {
  let my_alpha:𝚻Alpha;
  my_alpha = 𝚻Alpha{a:2};
  foo(my_alpha); //x=EnAlpha(𝚻Alpha { a: 2 })
}



// Example of the above: limit type to be either str or i32

use std::string::String;
use core::primitive::i32;

#[derive(Debug)] pub enum   EnStrOrI32 { EnStr(String), EnI32(i32),}

impl From<String> for EnStrOrI32 { fn from(arg_𝛂:String) -> Self { Self::EnStr(arg_𝛂)} }
impl From<i32   > for EnStrOrI32 { fn from(arg_𝛃:i32   ) -> Self { Self::EnI32(arg_𝛃)} }

mod sealed_str_or_i32 {
  pub trait Sealed2 {}
  impl Sealed2 for super::String {}
  impl Sealed2 for super::i32 {}
}

pub fn fn_str_or_i32<T: Into<EnStrOrI32> + sealed_str_or_i32::Sealed2 + std::fmt::Debug>(x: T) {
  println!("got arg = {:?}", x);
  let y = x.into();
  println!("applied its From trait: arg.into = {:?}", y);
  match y {
    EnStrOrI32::EnStr(arg_𝛂)	=> println!("Matched arg as a string = {:?}", arg_𝛂),
    EnStrOrI32::EnI32(arg_𝛃)	=> println!("Matched arg as an i32   = {:?}", arg_𝛃),
  }
}
pub fn arg_str_or_i32() {
  let my_str:String = "my_alpha".to_string();
  fn_str_or_i32(my_str); //x=EnStr("my_alpha")
  let my_i32:i32 = 5;
  fn_str_or_i32(my_i32); //x=EnI32(5)
  // fn_str_or_i32(4.0); // err: trait Sealed2 is not implemented for {float}
}
