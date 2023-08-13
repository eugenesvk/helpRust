use phf::phf_map;
#[derive(Clone,Debug)] pub enum Keyword {Loop,Continue,Break,Fn,Extern,}
static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
  "loop"    	=> Keyword::Loop,
  "continue"	=> Keyword::Continue,
  "break"   	=> Keyword::Break,
  "fn"      	=> Keyword::Fn,
  "extern"  	=> Keyword::Extern,
};
static BITFLAGS: phf::Map<&'static str, ModiCombo> = phf_map! {
  "loop"	=> ModiCombo::LShift,
};
pub fn key_enum_def_val() {
  for   kside in &KEYWORDS {
    p!("{:?}",kside);
  }
  for   kside in &BITFLAGS {
    p!("{:?}",kside);

  }


// way too big a table to do manually???
// use phf::{phf_map,phf_ordered_map};
// static BITFLAGS: phf::OrderedMap<&'static str, ModiCombo> = phf_ordered_map! {
//   "leftshift"=>ModiCombo::LShift,"left_shift" =>ModiCombo::LShift,"lshift"=>ModiCombo::LShift,"‹shift"=>ModiCombo::LShift,
//   "left⇧"    =>ModiCombo::LShift,"left_⇧"     =>ModiCombo::LShift,"l⇧"   =>ModiCombo::LShift,"‹⇧"   =>ModiCombo::LShift,
//   "rightshift"=>ModiCombo::RShift,"right_shift"=>ModiCombo::RShift,"rshift"=>ModiCombo::RShift,"shift›"=>ModiCombo::RShift,
//   "right⇧"   =>ModiCombo::RShift,"right_⇧"    =>ModiCombo::RShift,"r⇧"   =>ModiCombo::RShift,"⇧›"   =>ModiCombo::RShift,
//   "shift"     =>ModiCombo::Shift,"shift_"       =>ModiCombo::Shift,
//   "⇧"        =>ModiCombo::Shift,
//   "leftctrl"=>ModiCombo::LCtrl,"left_ctrl"    =>ModiCombo::LCtrl,"lctrl"=>ModiCombo::LCtrl,"‹ctrl"=>ModiCombo::LCtrl,
//   "left⎈"    =>ModiCombo::LCtrl,"left_⎈"     =>ModiCombo::LCtrl,"l⎈"   =>ModiCombo::LCtrl,"‹⎈"   =>ModiCombo::LCtrl,
//   "rightctrl"=>ModiCombo::RCtrl,"right_ctrl"  =>ModiCombo::RCtrl,"rctrl"=>ModiCombo::RCtrl,"ctrl›"=>ModiCombo::RCtrl,
//   "right⎈"   =>ModiCombo::RCtrl,"right_⎈"    =>ModiCombo::RCtrl,"r⎈"   =>ModiCombo::RCtrl,"⎈›"   =>ModiCombo::RCtrl,
//   "ctrl"     =>ModiCombo::Ctrl,"ctrl_"       =>ModiCombo::Ctrl,
//   "⎈"        =>ModiCombo::Ctrl,
// };

  // for   kside in &BITFLAGS {
  //   p!("{:?}",kside);
  // }
