#![allow(dead_code, unused_variables, unused_imports, non_upper_case_globals, non_camel_case_types)]

use crate::helper	::{type_of,print_type_of};
use palette      	::{FromColor, Hsl, IntoColor, Lch, Srgb, Okhsv, Okhsl, OklabHue};
use std          	::println as p; // requires syntax theme override to retain syntax highlighting
use std::string  	::String;
use std::borrow  	::Cow;
use std::path    	::{Path, PathBuf}; // Path is a slice, PathBuf is like String owned, mutable
use plist        	::Value;
use shellexpand  	::tilde;

use colored::*;
// use associated constants on Enums
// parser
  // color input: let fields with `°` or `deg` be allowed in any order since their type is obvious, % is optional. Also in addition to pure numbers you should be able to use `r` or `r0°` for red and then `r±X°` for relative rotation or `r+1°` `r−1°`  `r1°`
  // hsv or hsl signalled globally and locally, local signal overrides
  // use L or V upper case with optional : or ␠
    // (it's our code, can do whatever) not sure how to do global signal, must have a proc macro, not enough o have a parser?
enum okWheel {
  okHSL(ColorWheelL),
  okHSV(ColorWheelV),
}
#[derive(Debug,Clone)] pub struct ColorWheelV;
#[derive(Debug,Clone)] pub struct ColorWheelL;
  // instead of 0–360° that you have to remember (and that are different in okHSL vs HSL) use mnemonic color names
  // r o y c g gg a aa b v m p = 12 colors with bands of 30° in HSV, but will be different in okHSL/okHSV
  // r   y   g    a    b   m   =  6 colors with only "max" R/G/B values
impl  ColorWheelDefaultL for ColorWheelL {}
impl  ColorWheelDefaultV for ColorWheelV {}
trait ColorWheelDefaultV { // use associated constants to store default values
  /* # Color              	RrGgBb	 Δ  HSV          	 Δ  okHSV     	okLCH    	okHSL     	ΔHSV	ΔokHSV	ΔokHSL 2
  r                       	ff    	30   0° 100% 100%	27  29 100 100	 29 56 57	 29 100 57	.   	.     	+26  +61
  o                       	ff80  	30  30° 100% 100%	24  53 100 100	 53 19 69	 53 100 69	+24 	+30   	+24
  y                       	ffff  	30  60° 100% 100%	57 110 100 100	110 21 96	110 100 96	+57 	+30   	+57  +81
  c  chartreuse           	80ff00	30  90° 100% 100%	26 136 100 100	136 26 87	136 100 87	+26 	+30   	+26
  g                       	  ff  	30 120° 100% 100%	 6 142 100 100	142 29 84	142 100 84	+ 6 	+30   	+ 6  +32
  gg Guppie green (spring)	  ff80	30 150° 100% 100%	 9 151 100 100	151 23 85	151 100 85	+ 9 	+30   	+ 9
  a  aqua                 	  ffff	30 180° 100% 100%	44 195 100 100	195 15 89	195 100 89	+44 	+30   	+44  +53
  aa Azure                	  80ff	30 210° 100% 100%	61 256 100 100	256 21 55	256 100 55	+61 	+30   	+61
  b                       	    ff	30 240° 100% 100%	 8 264 100 100	264 31 37	264 100 37	+ 8 	+30   	+ 8  +69
  v                       	80  ff	30 270° 100% 100%	30 294 100 100	294 29 46	294 100 46	+30 	+30   	+30
  m Magenta/Fuchsia       	ff  ff	30 300° 100% 100%	34 328 100 100	328 32 65	328 100 65	+34 	+30   	+34  +64
  p Deep Pink             	ff  80	30 330° 100% 100%	34   2 100 100	  2 26 59	  2 100 59	+34 	+30   	+35
  ———Dark Violet          	93  ff	   275° 100% 100%	   300 100 100	300 29 48	300 100 48	+15 	+15   	.
  i                       	4b  82	   275° 100%  51%	   302 100 100	302 18 24	302 100 24	    	      	.
  */
  const r : Okhsv = Okhsv::new_const(OklabHue::new( 29_f32), 1., 1.);
  const o : Okhsv = Okhsv::new_const(OklabHue::new( 53_f32), 1., 1.);
  const y : Okhsv = Okhsv::new_const(OklabHue::new(110_f32), 1., 1.);
  const c : Okhsv = Okhsv::new_const(OklabHue::new(136_f32), 1., 1.);
  const g : Okhsv = Okhsv::new_const(OklabHue::new(142_f32), 1., 1.);
  const gg: Okhsv = Okhsv::new_const(OklabHue::new(151_f32), 1., 1.);
  const a : Okhsv = Okhsv::new_const(OklabHue::new(195_f32), 1., 1.);
  const aa: Okhsv = Okhsv::new_const(OklabHue::new(256_f32), 1., 1.);
  const b : Okhsv = Okhsv::new_const(OklabHue::new(264_f32), 1., 1.);
  const v : Okhsv = Okhsv::new_const(OklabHue::new(294_f32), 1., 1.);
  const m : Okhsv = Okhsv::new_const(OklabHue::new(328_f32), 1., 1.);
  const p : Okhsv = Okhsv::new_const(OklabHue::new(  2_f32), 1., 1.);
}
trait ColorWheelDefaultL { // same, but using okHSL
  const _r : Okhsv = ColorWheelV::r;
  const _o : Okhsv = ColorWheelV::o;
  const _y : Okhsv = ColorWheelV::y;
  const _c : Okhsv = ColorWheelV::c;
  const _g : Okhsv = ColorWheelV::g;
  const _gg: Okhsv = ColorWheelV::g;
  const _a : Okhsv = ColorWheelV::a;
  const _aa: Okhsv = ColorWheelV::a;
  const _b : Okhsv = ColorWheelV::b;
  const _v : Okhsv = ColorWheelV::v;
  const _m : Okhsv = ColorWheelV::m;
  const _p : Okhsv = ColorWheelV::p;
  const  r : Okhsl = Okhsl::new_const(Self::_r .hue, Self::_r .saturation, Self::_r .value);
  const  o : Okhsl = Okhsl::new_const(Self::_o .hue, Self::_o .saturation, Self::_o .value);
  const  y : Okhsl = Okhsl::new_const(Self::_y .hue, Self::_y .saturation, Self::_y .value);
  const  c : Okhsl = Okhsl::new_const(Self::_c .hue, Self::_c .saturation, Self::_c .value);
  const  g : Okhsl = Okhsl::new_const(Self::_g .hue, Self::_g .saturation, Self::_g .value);
  const  gg: Okhsl = Okhsl::new_const(Self::_gg.hue, Self::_gg.saturation, Self::_gg.value);
  const  a : Okhsl = Okhsl::new_const(Self::_a .hue, Self::_a .saturation, Self::_a .value);
  const  aa: Okhsl = Okhsl::new_const(Self::_aa.hue, Self::_aa.saturation, Self::_aa.value);
  const  b : Okhsl = Okhsl::new_const(Self::_b .hue, Self::_b .saturation, Self::_b .value);
  const  v : Okhsl = Okhsl::new_const(Self::_v .hue, Self::_v .saturation, Self::_v .value);
  const  m : Okhsl = Okhsl::new_const(Self::_m .hue, Self::_m .saturation, Self::_m .value);
  const  p : Okhsl = Okhsl::new_const(Self::_p .hue, Self::_p .saturation, Self::_p .value);
}
trait ConstantIdDefault {
  const ID: i32 = 1;
}

struct Struct;
struct OtherStruct;

impl ConstantIdDefault for Struct {}

impl ConstantIdDefault for OtherStruct {
  const ID: i32 = 5;
}
// old : self = self()
  // print_type_of(&ColorWheelV::r); //mac_excel_plist::color_wheel::ColorWheelV
  // print_type_of(&ColorWheelV); //mac_excel_plist::color_wheel::ColorWheelV::{{constructor}}
  // print_type_of(&ColorWheelV::r.0); //palette::okhsv::Okhsv

pub fn test_color_wheel() {
  // const c_dummy:Okhsv = Okhsv::new_const(OklabHue::new(0.0_f32), 0.5, 0.5);
  // const c:ColorWheel = ColorWheel(c_dummy);
  use ColorWheelV as C;
  p!("hello test_color_wheel");
  p!("{:?}",ColorWheelV::r);
  print_type_of(&ColorWheelV)   	; //mac_excel_plist::color_wheel::ColorWheelV
  print_type_of(&ColorWheelV::r)	; //palette::okhsv::Okhsv
  p!("{:?}",C::r)               	; //Okhsv { hue: OklabHue(29.0), saturation: 1.0, value: 1.0 }
  p!("{:?}",Struct::ID);
  p!("{:?}",OtherStruct::ID);
  print_type_of(&Struct::ID);
}
