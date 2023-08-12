#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros)]

use helper::alias::*;
use helper::pb;
use helper::helper::*;

use std      	::env;
use std::fs  	::File;
use std::io  	::{BufWriter, Write};
use std::path	::Path;

fn main() {


  let     path	= Path     ::new(&env::var("OUT_DIR").unwrap()).join("key2bit_codegen.rs");
  let mut file	= BufWriter::new(File::create(&path).unwrap());
  pb!("path={:?}",&path.as_os_str());

  write!(&mut file, "static KEYWORDS: phf::Map<&'static str, Keyword> = {}",
    phf_codegen::Map::new()
      .entry("loop"    	, "Keyword::Loop")
      .entry("continue"	, "Keyword::Continue")
      .entry("break"   	, "Keyword::Break")
      .entry("fn"      	, "Keyword::Fn")
      .entry("extern"  	, "Keyword::Extern")
      .build()
  )
  .unwrap();
  write!(&mut file, ";\n").unwrap();
}
