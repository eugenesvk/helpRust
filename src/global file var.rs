#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros)]

extern crate helper;
use helper        	::*; // gets macros
use helper::alias 	::*;
use helper::helper	::*;

use std      	::env;
use std::fs  	::{self,File,OpenOptions};
use std::io  	::{self,prelude::*,BufWriter, Write};
use std::path	::Path;

const ppath :&str = "C:/Dev/repo/Rust/winAPIconst/t.txt";

use once_cell::sync::Lazy;
use chrono::Utc;
static GLOBAL_DATA: Lazy<String> = Lazy::new(||Utc::now().to_string());
static gFFF: Lazy<File> = Lazy::new(||File::open(Path::new(ppath)).unwrap());

fn cat(path: &Path) -> io::Result<String> { // A simple implementation of `% cat path`
  let mut f = File::open(path)?;
  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}
fn catf(mut f: &File) -> io::Result<String> { // A simple implementation of `% cat path`
  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok (_) => Ok (s),
    Err(e) => Err(e),
  }
}
fn main() {
  pb!("now={} GLOBAL_DATA={}", Utc::now().to_string(),*GLOBAL_DATA);
  // pb!("{}",cat(Path::new(ppath)).unwrap());
  pb!("{}",catf(&gFFF).unwrap());
}
