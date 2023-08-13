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

  }
}
// impl fmt::Debug   for ModiCombo	{fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {fmt::Debug  ::fmt(&self.0,f)}}
impl fmt::Display for ModiCombo   	{fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {fmt::Display::fmt(&self.0,f)}}
impl str::FromStr for ModiCombo   	{
  type Err = bitflags::parser::ParseError;
  fn from_str(flags: &str) -> Result<Self, Self::Err> {Ok(Self(flags.parse()?))}
}

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


  #[test] fn key_combos() {
    let mut modiDefAct:IndexMap<&str, kModiFlag> = IndexMap::new();
    // definition vs actual
    modiDefAct.insert("‹⇧ ⎈›"	, km::LShift | km::RCtrl);
    modiDefAct.insert("‹⇧ ⎈ "	, km::LShift | km::RCtrl);
    modiDefAct.insert("‹⇧ ⎈" 	, km::LShift | km::LCtrl);
    modiDefAct.insert(" ⇧ ⎈" 	, km::RShift | km::RCtrl);
    p!("{}{} compare a string of modifiers like {} to actual modifier flags like {}"
       ,"ƒ".yellow(),"key_combos","‹⇧ ⎈›".blue(),(km::LShift | km::RCtrl).to_string().blue());
    for (def_key_combo, act_modi) in modiDefAct.iter() {
      let (def_modi,_) = match parse_key_definition(&(def_key_combo.to_string()+"x"), &key2bit) {
        Ok((modi,key))	=> (modi,key),
        Err(e)        	=> {panic!("Couln't parse the key ‘{}’ to match it to ‘{}’ due to ‘{}’",def_key_combo.blue(),act_modi,e.red());},
      };
      p!("  def_modi {} from {}",def_modi.to_string().blue(),def_key_combo.blue());
      p!("  act_modi {}",act_modi.to_string().blue());
      assert_eq!{true,isModiDefAct(def_modi,*act_modi)};
    }
  }

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