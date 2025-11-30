                fn function() {println!("called `function()`");}
  mod cool {pub fn function() {println!("called `cool::function()`");}}
mod my     {    fn function() {println!("called `my::function()`");}
  mod cool {pub fn function() {println!("called `my::cool::function()`");}}
  pub fn indirect_call() {
    print!("called `my::indirect_call()`, that\n> "); // Let's access all the functions named `function` from this scope!
    self::function(); //=↓ self=current module scope (`my`)
    function(); // =↑
    self::cool::function(); // `self` to access another module inside `my`: my::cool::function()
    super::function();      // `super` = parent scope (outside the `my` module)
    {use crate::cool::function as root_function; // bind to the `cool::function` in the *crate* scope. In this case the crate scope is the outermost scope
      root_function();}
  }
}
fn main() {
  my::indirect_call();
}
