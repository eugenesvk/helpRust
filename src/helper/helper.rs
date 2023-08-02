use std     	::println as p; // requires text editor's syntax theme override to retain syntax highlighting
use std::any	::type_name; // for type_of

pub fn type_of      <T>(_: T) -> &'static str {          type_name::<T>() }
pub fn print_type_of<T>(_:&T)                 { p!("{}", type_name::<T>());}
