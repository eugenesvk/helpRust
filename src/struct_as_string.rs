#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros)]

pub fn struct_as_str(){

  pub struct val {}
  pub struct kModiFlagg {}
  impl kModiFlagg { const a: val = val {};}
  use std::fmt;
  impl fmt::Display for val	{fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
    // fmt::Display::fmt(&self,f)}
    write!(f, "(value a: )")
  }}

  p!("{}",kModiFlagg::a);

let flg = kModiFlag::LShift;

macro_rules! MyDisplay {
($struct:ident) => {(type_of($struct))};
}
    println!("{}", MyDisplay!(flg));
    println!("{}",  flg);
struct A {a:i32}
  let my_identifier = A {a:1};
  p!("¦{}¦",stringify!({},flg));
  p!("¦{:?}¦",flg);
  // let var = 1;
  // let st = fmt::Sprintf("%#v", var);
  // p!("{st}");


use helper::key	::kModiFlag;
  let flg = kModiFlag::LShift | kModiFlag::LCtrl;
  println!(":?¦{:?}¦", flg); //:?¦kModiFlag::LShift | kModiFlag::LCtrl¦
  let flg = kModiFlag::Shift;
  println!(":?¦{:?}¦", flg); //:?¦kModiFlag::LShift | kModiFlag::RShift¦
  println!("¦{}¦", flg); //
  // how to print as ::Shift


  // #[derive(AsStrr)]  struct ABC { a:i32}
  // impl ABC { const c :i32=1;}
  // let abc = ABC {a:1} ;
  // let abcc = ABC::c;
  // println!("abcc¦{}¦", abcc);
  // println!("flg¦{}¦ :?¦{:?}¦ type¦{}¦", flg, flg, type_of(flg));
  // println!("type_of(ABC::c¦{}¦", type_of(ABC::c));
  // println!("¦{}¦", abc.as_str());
  // // println!("¦{}¦", flg.as_strr()); // need to define my method,maybe usig bitflags::parser::to_writer
  // println!("flags.to_string¦{}¦", flg.to_string());
  // // println!("¦{}¦", bitflags::parser::to_writer(&flg, fmt::Formatter).unwrap());
  // // println!("¦{}¦", abcc.as_str()); // error as it's i32
  // println!("————————————————————————————————————");

}
