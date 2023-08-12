use helper::alias::*;
use helper::helper::*;

fn paths() {
  use std::path::{Path, PathBuf}; // Path is a slice, PathBuf is like String owned, mutable
  let string     	= String::from("foo.txt");
  let from_string	= Path::new(&string);
  let from_path  	= Path::new(&from_string);
  // assert_eq!(from_string, from_path);
  let from_combo	= Path::new("/etc").join("passwd");
  p!("{}", from_combo.display())
}
