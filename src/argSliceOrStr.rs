#![allow(non_snake_case,unused_imports,unused_mut,unused_variables,dead_code, unused_assignments)]

// users.rust-lang.org/t/make-a-function-accept-either-str-or-str-as-an-argument/79139/9

use crate::type_of;
use std::slice::from_ref;

use crate::print_type_of;

trait              	AsStrSlice                	{fn as_slice(&self) -> &[&str]; }
impl               	AsStrSlice for     &str   	{fn as_slice(&self) -> &[&str] {from_ref(self)}}
impl               	AsStrSlice for   &[&str]  	{fn as_slice(&self) -> &[&str] {         self }}
impl<const N:usize>	AsStrSlice for    [&str;N]	{fn as_slice(&self) -> &[&str] {         self }}
impl               	AsStrSlice for Vec<&str>  	{fn as_slice(&self) -> &[&str] {         self }}

fn print_as_slice<T>(slice:T) where T:AsStrSlice {
  let slice = slice.as_slice();
  println!(".as_slice = {:?}", slice);
  // println!("type_of   = {:?}",type_of(slice));
}

pub fn arg_slice_or_str() {
  let s_l:&str  	= "string literal";
  let s :String 	= "s".to_string();
  let s1:&String	= &s;
  let s2:&str   	= &s[..];
  let t_vec:Vec<&str> = vec!["4str_lit@Vec#1", "4str_lit@Vec#2"];
  print_as_slice(     "1string literal");
  print_as_slice(    ["2str_lit@Arr#1"]);
  print_as_slice(    ["3str_lit@Arr#1","3str_lit@Arr#2"]);
  print_as_slice(vec!["4str_lit@Vec#1","4str_lit@Vec#2"]);

  let ss:String = "5OwnedString".to_string();
  print_as_slice(&*ss);
    //   ss     :  String
    //  *ss     :  str (via Deref<Target=str>)
    // &*ss     : &str
    // & ss[..] :  &str
    // via deref coercion, but you must provide a hint to the compiler (via an explicit type)
      // let s_slice: &str = &ss;        // ok, an explicit type in a variable declaration
      // fn take_name(name: &str) {/**/} //     an explicit type in a function argument
      // take_name(&ss);                 // ok, due to ↑
      // let wrong = &ss;    // wrong, &String instead of &str since no type hints
}
