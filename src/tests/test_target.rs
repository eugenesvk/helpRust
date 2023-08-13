// #![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros)]
// // use crate::helper::alias::*;
// // use crate::helper::helper::*;

// // then move to actually my wasm crate
// //
// use crate::helper::alias::*;
// use crate::helper::helper::*;

// #[derive(Debug,Clone)] pub struct UserAgentOS;
// impl  UserAgentOSDefault<'_> for UserAgentOS {}
// trait UserAgentOSDefault<'a> { // use associated constants to store default values
//   const Win	: &'a str = "windows";
// }
//    // $($t:ident=$t2:literal),+	// pairs passed to other macro that would
//    // $const_name:ident=$tconst_value:literal,	// pairs passed to other macro that would
//       // $($rest:expr),*


// #[macro_export] macro_rules! arr_from_consts_t {
//   ($nv:vis const $ni:ident = [$( $(#[doc = $doc:expr])* $v:vis const $i:ident: $t:ty = $e:expr; )*]) => {
//     $($v const  $i: $t = $e;)*
//     $nv  const $ni: [&str; count!($($i)*)] = [ $($i),*];
//   }
// }
// macro_rules! count {
//   () => (0usize);
//   ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
// }
// #[macro_export] macro_rules! arr_from_consts_str_impl {
//   // #[derive(Debug,Clone)]#[non_exhaustive] pub struct MyStructName;
//   // impl  MyStructName { // use associated constants to store default values
//   //   const A1	: &str = "val1";
//   //   const A2	: &str = "val2";
//   //   ... }
//   ($struct_name:ident,
//    $array_name:ident,
//    $(      $const_name:ident= $const_value:literal),+
//    $(,)?	// ignore trailing commas
//     ) => {
//     #[derive(Debug,Clone)]#[non_exhaustive] pub struct $struct_name;
//     impl $struct_name {
//       $(pub const $const_name:&str = $const_value;       ) +
//      }
//     pub const $array_name:[&str; count!($($const_name)*)] = [$($struct_name::$const_name),+];
//   };
// }
// // #[macro_export] macro_rules! const_str_assign {
// //   ($(      $const_name:ident= $const_value:literal),+) => {
// //    $(const $const_name:&str = $const_value;       ) +};}
//     // pub const UserAgentOS1_array:[i32;3] = [1,2,3];

// pub fn test_target() {
//   // p!("hello test_target ×××");
//   // const_str_assign!(a1="a2", t1="t2");
//   arr_from_consts_str_impl!(UserAgentOS1,UserAgentOS1_array, Win="windows_macro",Mac="mac_macro");
//   p!("{}",UserAgentOS1_array[1]);
//   // https://github.com/rust-lang/rust/issues/29599 not possible to add a suffix `_array` to an identifier
//   // arr_from_consts_t!(pub const MyConstants = [
//     // pub const MyConstant1:&str = "Hello";
//     // pub const MyConstant2:&str = "Stackoverflow";
//   // ]);
//   // for myc in MyConstants {p!("{myc}")};
//   // p!("{a}");
//   // for os in OS.iter() {
//     // p!("{os}");
//   // }
//   // let res = match ourOS {
//   //   for os in OS.iter() {
//   //     let is_match = agent_s.to_lowercase()
//   //       .matches(os.to_lowercase().as_str())
//   //       .next().is_some();
//   //     if is_match {set_value(true);break;}
//   //     }}
//   //   Err(_) => p!("Failed to determine browser OS!"),
//   // }
//   // p!("{:?}",OS::Win);
//   // print_type_of(&UserAgentOS)   	; //mac_excel_plist::color_wheel::UserAgentOS
//   // print_type_of(&UserAgentOS::r)	; //palette::okhsv::Okhsv
//   // p!("{:?}",C::r)               	; //Okhsv { hue: OklabHue(29.0), saturation: 1.0, value: 1.0 }
//   // p!("{:?}",Struct::ID);
//   // p!("{:?}",OtherStruct::ID);
//   // print_type_of(&Struct::ID);
// }


