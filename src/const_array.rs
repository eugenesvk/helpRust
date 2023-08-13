
const K1: [i32; 8] = [100, 100, 101, 102, 104, 106, 108, 900];
const K2: [i32; 8] = array_i32_mul(-1, K1);
const K3: [i32; 8] = array_reverse(K2);

const fn array_i32_mul<const N: usize>(factor: i32, mut a: [i32; N]) -> [i32; N] {
  let mut i = 0;
  while i < N {
    a[i] *= factor;
    i    += 1;
  }
  a
}
const fn array_reverse<T: Copy, const N: usize>(mut a: [T; N]) -> [T; N] {
  let mut i = 0;
  while i < N / 2 {
    let from_end = N - i - 1;
    (a[i], a[from_end]) = (a[from_end], a[i]);
    i += 1;
  }
  a
}


// can't combine Strings in const

const symLeft_atL  :[&str;3]	= ["left","left_","‹"];
const symLeft_atR  :[&str;0]	= [];
const symShift     :[&str;2]	= ["shift","⇧"];
const symLeftShift :[&str;6]	= ["","","","","","",];
// const symRight:[[&str;2]; [&str;1]] = [["right","right_"],["›"]];
const fn comboshift<T:Copy, const N1:usize, const N2:usize, const NN:usize>(mut a:[T;N1], mut b:[T;N2], mut c:[T;NN]) -> [T;NN]
  where T:std::fmt::Display {
  let mut i = 0;
  let mut j = 0;
  while   i < N1 {
    while j < N2 {
      c[i] = (a[i].to_string() + b[j]).as_str();
      j += 1;
    }
    i += 1;
  }
  c
}
const symLeftShift2 :[&str;6]	= comboshift(symLeft_atL,symShift,symLeftShift);



// #[derive(Debug,Clone,PartialEq,Sequence)] pub enum ModiName {
//  LShift
//  // ,ModiCombo::LCtrl,ModiCombo::LWinCmd,ModiCombo::LAlt,ModiCombo::RShift,ModiCombo::RCtrl,ModiCombo::RWinCmd,ModiCombo::RAlt,
// }
// // todo: convert to https://doc.rust-lang.org/reference/const_eval.html with keys as string names hashes and values as bits
// impl ModiName {
//   const fn str(&self) -> &'static str {
//     match self { // order matters for iteration, so sides should go first
//       Self::LShift 	=> ["LShift","","‹⇧"],
//       Self::RShift 	=> ["RShift","","⇧›"],
//       Self:: Shift 	=> [" Shift","", "⇧"],
//       Self::LCtrl  	=> ["LCtrl","‹⎈","‹⌃","‹^"],
//       Self::RCtrl  	=> ["RCtrl","⎈›","⌃›","^›"],
//       Self:: Ctrl  	=> [" Ctrl", "⎈", "⌃", "^"],
//       Self::LWinCmd	=> "‹◆",
//       Self::RWinCmd	=> "◆›",
//       Self:: WinCmd	=> "◆",
//       Self::LAlt   	=> "‹⎇",
//       Self::RAlt   	=> "⎇›",
//       Self:: Alt   	=>  "⎇",
//     }
//   }
//   // const fn bit(&self) -> ModiName { use ModiName::*;
//   //   match self {
//   //     LShift               	=> ModiName::LShift,
//   //     // ModiCombo::RShift 	=> "a",
//   //     // ModiCombo:: Shift 	=> "a",
//   //     // ModiCombo::LCtrl  	=> "a",
//   //     // ModiCombo::RCtrl  	=> "a",
//   //     // ModiCombo:: Ctrl  	=> "a",
//   //     // ModiCombo::LWinCmd	=> "a",
//   //     // ModiCombo::RWinCmd	=> "a",
//   //     // ModiCombo:: WinCmd	=> "a",
//   //     // ModiCombo::LAlt   	=> "a",
//   //     // ModiCombo::RAlt   	=> "a",
//   //     // ModiCombo:: Alt   	=> "a",
//   //   }
//   // }
// }

// THIS allows iteration over enum values to get, but needs a separate crate
use enum_iterator::{all, cardinality, first, last, next, previous, reverse_all, Sequence};
#[derive(Debug,Clone,PartialEq,Sequence)] pub enum keySideName {Left,Right}
#[derive(Debug,Clone,PartialEq,Sequence)] pub enum keySideLoc {Left,Right}
#[derive(Debug,Clone,PartialEq,Sequence)] pub enum keyName {Shift,Ctrl,WinCmd,Alt}

