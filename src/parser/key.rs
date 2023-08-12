#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros)]
use helper::alias 	::*;
use helper::helper	::*;


/*
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
*/

use std::{fmt, str};
use bitflags::bitflags;
bitflags! { #[repr(transparent)] #[derive(Default,Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Hash)]
  // If your default value is equal to 0 (which is the same value as calling empty() on the generated struct), you can simply derive Default:
  pub struct ModiCombo: u32 {
    // const None     	= 0b_____________0; //   0 0-flags: treated as always present; ignored when testing for emptiness
    const LShift      	= 0b_____________1; //   1 ‹⇧
    const LCtrl       	= 0b____________10; //   2 ‹⎈
    const LWinCmd     	= 0b___________100; //   4 ‹◆ = ❖ Windows or ⌘ Mac  aka Super, hijacked Meta's symbol for for either Win or Cmd
    const LAlt        	= 0b__________1000; //   8 ‹⎇
    const RShift      	= 0b_________10000; //  16 ⇧›
    const RCtrl       	= 0b________100000; //  32 ⎈›
    const RWinCmd     	= 0b_______1000000; //  64 ◆› = ❖› Windows or ⌘› Mac
    const RAlt        	= 0b______10000000; // 128 ⎇›
    // const LHyper   	= 0b_____100000000; // 256 ‹✦ ‹✧
    // const LMeta    	= 0b____1000000000; // 512 ‹◆
    // const RHyper   	= 0b___10000000000; //1024 ✦› ✧›
    // const RMeta    	= 0b__100000000000; //2048 ◆›
    // const Caps_lock	= 0b_1000000000000; //4096 ⇪
    // const Num_lock 	= 0b10000000000000; //8192 🔢
    const Shift       	= Self::LShift.bits() 	| Self::RShift.bits();
    const Ctrl        	= Self::LCtrl.bits()  	| Self::RCtrl.bits();
    const WinCmd      	= Self::LWinCmd.bits()	| Self::RWinCmd.bits();
    const Alt         	= Self::LAlt.bits()   	| Self::RAlt.bits();
    const Left        	= Self::LShift.bits() 	| Self::LCtrl.bits() | Self::LWinCmd.bits() | Self::LAlt.bits();
    const Reft        	= Self::RShift.bits() 	| Self::RCtrl.bits() | Self::RWinCmd.bits() | Self::RAlt.bits();
    // const Hyper    	= Self::LHyper.bits() 	| Self::RHyper.bits();
    // const Meta     	= Self::LMeta.bits()  	| Self::RMeta.bits();
  }
}
// impl fmt::Debug   for ModiCombo	{fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {fmt::Debug  ::fmt(&self.0,f)}}
impl fmt::Display for ModiCombo   	{fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {fmt::Display::fmt(&self.0,f)}}
impl str::FromStr for ModiCombo   	{
  type Err = bitflags::parser::ParseError;
  fn from_str(flags: &str) -> Result<Self, Self::Err> {Ok(Self(flags.parse()?))}
}

// todo : add a symbol for AltCmd since it's the same physical location
const symLeft_atL 	:[&str;4]	= ["left","left_","left ","‹"];
const symRight_atL	:[&str;3]	= ["right","right_","right "];
const symRight_atR	:[&str;1]	= ["›"];
const symShift    	:[&str;3]	= ["sh","shift","⇧"];
const symCtrl     	:[&str;5]	= ["ctrl","control","⎈","⌃","^"];
const symWinCmd   	:[&str;5]	= ["cmd","command","◆","⌘","❖"];
const symAlt      	:[&str;5]	= ["alt","opt","option","⎇","⌥"];


use indexmap	::{IndexMap, IndexSet};
pub fn prefill_key2bit() -> IndexMap<String, ModiCombo> {
  let mut key2bit:IndexMap<String, ModiCombo> = IndexMap::new();
  for symKey in symShift {
    {const key_bit:ModiCombo = ModiCombo::LShift;
    for symSide  in symLeft_atL  { key2bit.insert(symSide.to_string() + symKey , key_bit );}}
    {const key_bit:ModiCombo = ModiCombo::RShift;
    for symSide  in symRight_atL { key2bit.insert(symSide.to_string() + symKey , key_bit );}
    for symSide  in symRight_atR { key2bit.insert(symKey .to_string() + symSide, key_bit );}}
    {const key_bit:ModiCombo = ModiCombo::Shift;
                                   key2bit.insert(symKey .to_string()          , key_bit ); }  }
  for symKey in symCtrl {
    {const key_bit:ModiCombo = ModiCombo::LCtrl;
    for symSide  in symLeft_atL  { key2bit.insert(symSide.to_string() + symKey , key_bit );}}
    {const key_bit:ModiCombo = ModiCombo::RCtrl;
    for symSide  in symRight_atL { key2bit.insert(symSide.to_string() + symKey , key_bit );}
    for symSide  in symRight_atR { key2bit.insert(symKey .to_string() + symSide, key_bit );}}
    {const key_bit:ModiCombo = ModiCombo::Ctrl;
                                   key2bit.insert(symKey .to_string()          , key_bit ); }  }
  for symKey in symWinCmd {
    {const key_bit:ModiCombo = ModiCombo::LWinCmd;
    for symSide  in symLeft_atL  { key2bit.insert(symSide.to_string() + symKey , key_bit );}}
    {const key_bit:ModiCombo = ModiCombo::RWinCmd;
    for symSide  in symRight_atL { key2bit.insert(symSide.to_string() + symKey , key_bit );}
    for symSide  in symRight_atR { key2bit.insert(symKey .to_string() + symSide, key_bit );}}
    {const key_bit:ModiCombo = ModiCombo::WinCmd;
                                   key2bit.insert(symKey .to_string()          , key_bit ); }  }
  for symKey in symAlt {
    {const key_bit:ModiCombo = ModiCombo::LAlt;
    for symSide  in symLeft_atL  { key2bit.insert(symSide.to_string() + symKey , key_bit );}}
    {const key_bit:ModiCombo = ModiCombo::RAlt;
    for symSide  in symRight_atL { key2bit.insert(symSide.to_string() + symKey , key_bit );}
    for symSide  in symRight_atR { key2bit.insert(symKey .to_string() + symSide, key_bit );}}
    {const key_bit:ModiCombo = ModiCombo::Alt;
                                   key2bit.insert(symKey .to_string()          , key_bit ); }  }
  // for (k,v) in key2bit.iter() { p!("{k}={v}");  }
  // key2bit= right⎇=RAlt right_⎇=RAlt ⎇›=RAlt ... ⌥=LAlt | RAlt ordered from left/right keys to either
  key2bit
}

pub fn key_enum_def_val() {
  let key2bit = prefill_key2bit();

  // todo: add a proper parser without having to replace substrings?
  const key_seq_user_def	: &str = " ⇧› control+alt,command- X ";
  // let mut parse_key_seq_user_def = key_seq_user_def.trim(); // clean up: "⇧› control+alt,command- X";
  let mut parse_key_seq_user_def:String = key_seq_user_def.chars().filter(|c| !c.is_whitespace()).collect(); // remove any Unicode whitespace
  p!("parse_key_seq_user_def¦{parse_key_seq_user_def}¦"); //todo del

  // 1. find the final key (could also be the same as a modifier key)
  let mut key_seq_user_def_key:&str = ""; // todo replace with key crate nenum value
  for (k,v) in key2bit.iter() {
    if parse_key_seq_user_def.ends_with(k) {key_seq_user_def_key = k; break;} }
  p!("1 key_seq_user_def_key¦{key_seq_user_def_key}¦");
  let mut tmp = [0u8; 4]; // avoid heap allocation, also no 'temp value dropped' stackoverflow.com/a/67898224/20361194
  if key_seq_user_def_key.is_empty() { // key isn't a modifier, so use the last symbol
    key_seq_user_def_key = match parse_key_seq_user_def.pop() {
      Some(c)	=> c.encode_utf8(&mut tmp),
      None   	=> "",
    }
  }
  if key_seq_user_def_key.is_empty() { p!("could't find the key, returning"); return } // todo add appropriate return type error
  p!("2 key_seq_user_def_key¦{key_seq_user_def_key}¦");
  p!("2 parse_key_seq_user_def¦{parse_key_seq_user_def}¦"); //todo del

  let mut key_seq_user_def_mod:ModiCombo = Default::default();

  p!("key_seq_user_def      ¦{key_seq_user_def}¦");

  p!("   ××pre¦{}¦", key_seq_user_def_mod);
  for (k,v) in key2bit.iter() {
    if parse_key_seq_user_def.contains(k) {key_seq_user_def_mod |= *v; p!("contains¦{v}¦");}
  }
  p!("   ××pos¦{}¦", key_seq_user_def_mod);

  // todo:
  // proper match: 2 mandatory
    // actual ⊂ defined
    // Left only defined  NOT > actual ()
    // Right only defined NOT > actual
  // find a way to separate last key in the definition which can be a modifier key from left-side modifiers before matching!


  // key2bit.iter().find(|&&f) f == key_seq_user_def)
  // let desired = "control";
  // let to_eat = Ctrl.iter().find(|&&f| f == desired); //Some("control")
  // p!("{:?}", to_eat);
  // let desired = "ctrl1";
  // let to_eat = Ctrl.iter().find(|&&f| f == desired); //None
  // p!("{:?}", to_eat);
  // p!("¦{}¦", "⎈".to_lowercase()); //⎈



  // let anyshift:ModiCombo = ModiCombo::LShift | ModiCombo::RShift;
  // p!("anyshift=key_seq_user_def_mod¦{}¦", key_seq_user_def_mod==anyshift);
  // key_seq_user_def_mod = ModiCombo::LShift | ModiCombo::RAlt;
  // for (st,combo) in key_seq_user_def_mod.iter_names(){
  //   p!("st=¦{st}¦{} combo=¦{combo}¦{}",type_of(st),type_of(combo));
  //   // pt(combo);
  // }
  // for st in key_seq_user_def_mod.iter(){
  //   p!("st1=¦{st}¦{}",type_of(st));
  // }
  // p!("frombit¦{}¦", ModiCombo::from_bits(0b00000001).unwrap());
  // p!("from_name¦{:?}¦", ModiCombo::from_name("LShift").unwrap());
  // // p!("name¦{:?}¦", ModiCombo::LShift.name()); /fails, name is only for Flags, and you get those
  // print_type_of(&ModiCombo::LShift);


}


fn test_enums() {
  /*
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
  */
}