#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros)]
// use crate::helper::alias::*;
// use crate::helper::helper::*;

use helper::alias 	::*;
use helper::helper	::*;


macro_rules! count { // stackoverflow.com/questions/34304593/counting-length-of-repetition-in-macro/34324856#34324856
  ()                	=> (0usize                  );
  ($x:tt $($xs:tt)*)	=> (1usize + count!($($xs)*));
}
#[macro_export] macro_rules! impl_associated_str_const_and_iterable_array {
  // 1. #[derive(Debug,Clone)]#[non_exhaustive] pub struct MyStructName;
  // 2. impl  MyStructName { // use associated constants to store default values
  //   pub const MyConst1	: &str = "MyConstStrVal1";
  //   pub const MyConst2	: &str = "MyConstStrVal2";
  //   ... }
  // 3. pub const MyStructNameArr: [&str; #count)] = [MyStructName::MyConst1,MyStructName::MyConst2,...];
  ($struct_name:ident,
   $array_name :ident,
   $(             $const_name:ident = $const_value:literal),+
   $(,)?	/* ignore trailing commas*/ ) => {
    #[derive(Debug,Clone)]#[non_exhaustive]
    pub struct $struct_name;
    impl       $struct_name {
      $(pub const $const_name:&str  = $const_value;       ) +     }
    pub const $array_name:[&str; count!($($const_name)*)] = [$($struct_name::$const_name),+];
  };
}
// THIS allows iteration over associated constants
impl_associated_str_const_and_iterable_array!(
  UserAgentOS,UserAgentOS_arr
  , Win  	= "Winimpl_associated_str_const_and_iterable_array"
  , Mac  	= "Macimpl_associated_str_const_and_iterable_array"
  , Linux	= "Linuximpl_associated_str_const_and_iterable_array"
  , Unix 	= "Uniximpl_associated_str_const_and_iterable_array"
  );

#[derive(Debug,Clone)]#[non_exhaustive] pub struct UserAgentOS1;
impl  UserAgentOS1 { // use associated constants to store default values
  const Win  	: &str = "windows_ass_const_1impl";
  const Mac  	: &str = "default_ass_const_1impl";
  const Linux	: &str = "default_ass_const_1impl";
  const Unix 	: &str = "default_ass_const_1impl";
}

// to prevent direct instantiation of MyEnum you can tag it with #[non_exhaustive] //indicates that a type or variant may have more fields or variants added in the future. It can be applied to structs, enums, and enum variants
#[derive(Debug,Clone)] pub struct UserAgentOS3;
impl  UserAgentOSDefault<'_> for UserAgentOS3 {}
trait UserAgentOSDefault<'a> { // use associated constants to store default values
  const Win  	: &'a str = "windows_ass_const";
  const Mac  	: &'a str = "default_ass_const";
  const Linux	: &'a str = "default_ass_const";
  const Unix 	: &'a str = "default_ass_const";
}

// THIS allows iteration over enum values to get, but needs a separate crate
use enum_iterator::{all, cardinality, first, last, next, previous, reverse_all, Sequence};
#[derive(Debug,Clone,PartialEq,Sequence)] pub enum UserAgentOS2_enum {Win,Mac,Linux,Unix,NA}
impl UserAgentOS2_enum {
  const fn val(&self) -> &'static str {
    use UserAgentOS2_enum::*;
    match self {
      NA   	=> "unknown",
      Win  	=> "windows_enum_match",
      Mac  	=> "mac_enum_match",
      Linux	=> "default_enum_match",
      Unix 	=> "default_enum_match",
    }
  }
}
// test the same ↑ with a None valuer
#[derive(Debug,Clone,PartialEq,Sequence)] pub enum UserAgentOS_enum_opt {Win,Mac,Linux,Unix,NA}
impl UserAgentOS_enum_opt {
  const fn val(&self) -> Option<&'static str> {
    use UserAgentOS_enum_opt::*;
    match self {
      NA   	=> None,
      Win  	=> Some("windows_enum_match"),
      Mac  	=> Some("mac_enum_match"),
      Linux	=> Some("default_enum_match"),
      Unix 	=> Some("default_enum_match"),
    }
  }
}
#[derive(Debug,Clone,PartialEq,Eq)]
struct UserAgentOS4<'a> {
  pub V:&'a str
}
impl UserAgentOS4<'_>{
  const RIGHT	: Self = Self{V:"RIGHT klj;saldk"};
  const DOWN 	: Self = Self{V:"DOWN klj;saldk"};
  const LEFT 	: Self = Self{V:"LEFT klj;saldk"};
  const UP   	: Self = Self{V:"UP klj;saldk"};
}


pub fn enum_def_val() {
  p!("hello test_color_wheel");

  p!("UserAgentOS4::RIGHT ¦{:?}¦",UserAgentOS4::RIGHT);
  p!("UserAgentOS4::RIGHT.V ¦{:?}¦",UserAgentOS4::RIGHT.V);
  let trymatch = UserAgentOS4::RIGHT;
  match trymatch {
    UserAgentOS4::RIGHT => p!("matched UserAgentOS4::RIGHT"),
    _ => p!("did not matched UserAgentOS4::RIGHT"),
  }
  // let ourOS = "windows";
  // let a = arr_from_consts!(pub const MyConstants = [
  //   pub const MyConstant1:&str = "Hello";
  //   pub const MyConstant2:&str = "Stackoverflow";
  // ]);
  // p!("{a}");
  // for (i,myconst) in UserAgentOS_arr.iter().enumerate() {p!("UserAgentOS const #{i}: {myconst}")};

  use UserAgentOS2_enum as OS;
  // use UserAgentOS_enum_opt as OS;
  let myostr = "mac_enum_match";
  let mut myOS = UserAgentOS2_enum::NA;
  // possible to add an iter??
  for os in all::<UserAgentOS2_enum>().collect::<Vec<_>>() {
    let os_s = os.val();
    p!("OS in Enum: ¦{:?}¦ and it's value is¦{}¦",os,os_s);
    let is_match = myostr.to_lowercase()
      .matches(os_s.to_lowercase().as_str())
      .next().is_some();
    if is_match {myOS = os; break}
  };
  p!("OS in Enum found: ¦{:#?}¦ and it's value is¦{}¦",myOS,myOS.val());
  if myOS == OS::Mac {p!("myOS in Enum Matched ‘UserAgentOS2_enum::Mac’ ¦{:#?}¦ and it's value is¦{}¦",myOS,myOS.val());}

  use UserAgentOS_enum_opt as OSO;
  let mut myOS2 = UserAgentOS_enum_opt::NA;
  for os in all::<UserAgentOS_enum_opt>().collect::<Vec<_>>() {
    let os_s = os.val().unwrap();
    p!("OS in Enum: ¦{:?}¦ and it's value is¦{}¦",os,os_s);
    let is_match = myostr.to_lowercase()
      .matches(os_s.to_lowercase().as_str())
      .next().is_some();
    if is_match {myOS2 = os; break}
  };
  p!("OS in UserAgentOS_enum_opt found: ¦{:#?}¦ and it's value is¦{}¦",myOS2,myOS2.val().unwrap());
  if myOS2 == OSO::Mac {
    p!("myOS2 in Enum Matched ‘UserAgentOS_enum_opt::Mac’ ¦{:#?}¦ and it's value is¦{}¦",myOS2,myOS2.val().unwrap());
  }


  // p!("{:?}",UserAgentOS1::Win);
  // p!("{:?}",UserAgentOS::Win);
  // p!("{:?}",UserAgentOS2_enum::Win);
  // p!("{:?}",UserAgentOS2_enum::Win.val());
  // for os in OS.iter() {
    // p!("{os}");
  // }
  // let res = match ourOS {
  //   for os in OS.iter() {
  //     let is_match = agent_s.to_lowercase()
  //       .matches(os.to_lowercase().as_str())
  //       .next().is_some();
  //     if is_match {set_value(true);break;}
  //     }}
  //   Err(_) => p!("Failed to determine browser OS!"),
  // }
  // p!("{:?}",OS::Win);
  // print_type_of(&UserAgentOS)   	; //mac_excel_plist::color_wheel::UserAgentOS
  // print_type_of(&UserAgentOS::r)	; //palette::okhsv::Okhsv
  // p!("{:?}",C::r)               	; //Okhsv { hue: OklabHue(29.0), saturation: 1.0, value: 1.0 }
  // p!("{:?}",Struct::ID);
  // p!("{:?}",OtherStruct::ID);
  // print_type_of(&Struct::ID);
}


/*
  enum T<'a> {
    A(&'a str),
  }
  let t = T::A("b");
  match t {
    T::A(value) => match value {
        "a" => println!("a"),
        "b" => println!("b"),
        _ => println!("something else"),
    },
  }
*/
*