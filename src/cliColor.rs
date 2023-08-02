#![allow(dead_code, unused_variables, unused_imports, non_upper_case_globals)]

use colored::*;

fn colored_string() {
  println!("Hello world!");
  println!("{} {} !", "it".green(), "works".blue().bold());
  let t = "sdfasd";
  println!("{} {} !", t.green(), "works".blue().bold());

  // pushed colored string into another
  let mut s = String::with_capacity(5);
  let msg = "blue";
  s.push_str(&msg.blue().to_string());
  println!("{}", msg.blue());
  println!("{}", s);
}

pub fn color_cli() {
  colored_string();
}
