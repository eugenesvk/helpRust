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

fn list_pahs<P: AsRef<Path>>(paths:&[P]) { // for args: use generic to accept Path,PathBuf,&str,Strings
  // &[P] accepts Vec due to deref coercions
  /* Rust Path vs PathBuf
  - Path   	≈&str   or &[]	reference to the path string data but doesn't own this data (immutable, can't outlive actual data)
  - PathBuf	≈String or Vec	owns the string data itself. Allocation
  For arguments: use generic ↑ or
    - PathBuf if you need to store it somewhere
    - Path otherwise
  For return types:
    - Path: if return subpath of arg, e.g., Path[Buf].parent()
    - PathBuf: If create a new path/combine paths
  */
}
