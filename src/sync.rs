#![allow(dead_code, unused_variables, unused_imports, non_upper_case_globals)]
use parking_lot::OnceState;
use helper::alias::*;
use helper::helper::*;

use parking_lot::Once;
static g_once: Once = Once::new();
static g_1thread: Once = Once::new();
use std::thread;
use std::time::Duration;

pub fn sync_once() {
  let mut a:i8 = 0;
  assert!(OnceState::New == g_once.state(), "g_once.state pre call_once = {:?}", g_once.state());
  p!("g_once.state pre call_once = {:?} a={:?}",g_once.state(),a);
  g_once.call_once(|| {a = 2;});
  assert!(OnceState::Done == g_once.state(), "g_once.state pos call_once = {:?}", g_once.state());
  p!("g_once.state pos call_once = {:?} a={:?}",g_once.state(),a);
}
pub fn thread() {
  let handle = thread::spawn(move || { //need to move since thread can live longer then the function
    for i in 1..4 {println!("@spawned thread i = {}",i);thread::sleep(Duration::from_millis(1));}
  });
  // for i in 1..5 {println!("@main thread i = {}", i);thread::sleep(Duration::from_millis(1));}
  for i in 1..5 {println!("@main thread i = {}", i);thread::sleep(Duration::from_millis(1));}
  handle.join().unwrap();
}

pub fn sync_arc() { // thread-safe (Send+Sync if data is), but not mutable
  // makes it thread safe to have multiple ownership of the same data, but it doesn’t add thread safety to its data
  use std::sync::Arc;
  let a_arc:Arc<i8> = Arc::new(45); //(heap) A thread-safe reference-counting pointer ‘Atomically Reference Counted’
  let a = a_arc.clone(); // new Arc instance, points to the same allocation on the heap as the source Arc, while increasing a reference count
  let b = Arc::clone(&a_arc); // a, b, a_arc are Arcs that point to the same memory location
  // 'inner' value dropped when the last Arc pointer is destroyed
  // no copy trait for Arc, so can't use a here sync it's moved in the thread above
  let five = Arc::new(5); // no copy trait
  for _ in 0..10 { let five = Arc::clone(&five); // so need to clone
    thread::spawn(move || {println!("{five:?}");});}
}


pub fn sync_thread_once() {
  use std::sync::OnceLock; // sync primitive which can be written to only once, thread-safe OnceCell, can be used in statics
  struct DeepThought {answer:String}
  impl DeepThought {
    fn great_question() -> String {"42".to_string()}
    fn new() -> Self { Self { // M3 Ultra takes about 16 million years in --release config
      answer: Self::great_question(),}}  }
  fn computation() -> &'static DeepThought { // n.b. static items do not call [`Drop`] on program termination, so if [`DeepThought`] impls Drop, that will not be used for this instance
    static COMPUTATION:OnceLock<DeepThought> = OnceLock::new();
    COMPUTATION.get_or_init(|| DeepThought::new())
  }
  let _ = computation().answer; // DeepThought is built, stored in OnceLock and returned
  let _ = computation().answer; // DeepThought is retrieved   from OnceLock and returned

  // writing from a different thread
  static CELL:OnceLock<usize> = OnceLock::new();
  assert!(CELL.get().is_none()); // OnceLock has not been written to yet
  std::thread::spawn(|| { // Spawn a thread and write to OnceLock
    // let value = CELL.get_or_init(|| 12345);
    // assert_eq!(value, &12345);}).join().unwrap();
    let value = 12345;
    let _ = CELL.set(value);
    assert_eq!(value, 12345);}).join().unwrap();
  assert_eq!(CELL.get(),Some(&12345),); // OnceLock now contains the value
  // let value = CELL.get_mut().unwrap();
}

fn unbox<T>(value:Box<T>) -> T {*value}
pub fn sync_thread_once_dyn() {
  struct InputEvent {code:u32, up:bool}
  let in_ev = InputEvent {code:32, up:true};
  type HookFn = dyn FnMut(InputEvent) -> bool;

  use std::sync::OnceLock; // sync primitive which can be written to only once, thread-safe OnceCell, can be used in statics
  // static CELL:OnceLock<usize> = OnceLock::new();
  // static CELL:OnceLock<Option<Box<HookSFn>>> = OnceLock::new(); // ERR: no Sync for FnMut
  // use std::sync::Arc;
  // static CELL:OnceLock<Option<Arc<Box<HookSFn>>>> = OnceLock::new(None); // ERR no Sync
  // static HOOK_S: Lazy<Mutex<Option<Box<HookSFn>>>> = Lazy::new(|| Mutex::new(None)); // store thread-safe hook callback (can be called from an external process)

  type HookSSFn = dyn FnMut(InputEvent) -> bool + Send + Sync + 'static;
  // use core::fmt;
  // impl fmt::Debug for HookSSFn { // only traits defined in the current crate can be implemented for arbitrary types
    // fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {write!(f,"dyn FnMut(InputEvent)")}
  // }
  // static CELL:OnceLock<Option<Box<HookSSFn>>> = OnceLock::new(); // ERR: no Sync for FnMut
  static CELL:OnceLock<Option<Box<HookSSFn>>> = OnceLock::new(); // ERR: no Sync for FnMut
  assert!(CELL.get().is_none()); // OnceLock has not been written to yet
  std::thread::spawn(|| { // Spawn a thread and write to OnceLock
    // let value = CELL.get_or_init(|| 12345);
    // assert_eq!(value, &12345);}).join().unwrap();
    let callback = move |input_event:InputEvent| {return false};
    // let _ = CELL.set(Some(Box::new(callback)));
    // assert_eq!(value, 12345);
    // p!("{:?}",CELL.get().unwrap()(in_ev))
    // print_type_of(&CELL);
  }).join().unwrap();
  // let tttbox = CELL.get().unwrap(); //get(&self) -> Option<&T>
  // let ttt = tttbox.as_ref();
  // print_type_of(&ttt);
  // let tttfun: &Box<dyn FnMut(InputEvent) -> bool + Send + Sync> = ttt.unwrap();
  // let _ = tttbox(in_ev);
  // Define a boxed closure that takes an i32 and returns an i32
  let mut closure:Option<Box<HookSSFn>> = Some(Box::new(|input_event:InputEvent| true));
  let closure2:Option<Box<HookSSFn>> = Some(Box::new(|input_event:InputEvent| true));
  let result = closure.unwrap()(in_ev); // Call the closure with an argument
  p!("{:?}",result) ;
  print_type_of(&closure2);
  // let hook = CELL.take().expect("no recurse");



  // let callback = move |input_event:InputEvent| {return false};
  // let _ = CELL.set(Some(Box::new(callback)));
  let CELLL:OnceLock<String> = OnceLock::new();
  CELLL.set("hello".to_string()).unwrap();
  let c = CELLL.into_inner();
  // let b = CELLSTAT.into_inner(); // fails with static
  static CELLSTAT:OnceLock<Option<String>> = OnceLock::new(); //
  CELLSTAT.set(Some("hello".to_string())).unwrap();
  let b = CELLSTAT.get().expect("inner opt");
  // let a = <Option<String> as Clone>::clone(&b).expect("no recurse"); //
  let a = b.as_ref().unwrap(); // borrow type content
  p!("{:?}",a);




  let in_ev3 = InputEvent {code:32, up:true};
  let closure3:Box<HookSSFn> = Box::new(|input_event:InputEvent| true);
  static CELLSTATFN:OnceLock<Box<HookSSFn>> = OnceLock::new(); //
  if let Ok(set_closure) = CELLSTATFN.set(closure3) { //fn set(&self, value: T) -> Result<(), T>
    // .unwrap() fails since fmt::Debug isn't implemented and can't implement it on dyn fnmut
    p!("everything is ok, even though I can't unwrap");
    print_type_of(&set_closure);
  };
  let c = CELLSTATFN.get(); // print_type_of(&c); //core::option::Option<&alloc::boxed::Box<dyn core::ops::function::FnMut(various::sync::sync_thread_once_dyn::InputEvent) -> bool + core::marker::Send + core::marker::Sync>>
  // let d = c.expect("no recurse"); // print_type_of(&d); //&alloc::boxed::Box<dyn core::ops::function::FnMut(various::sync::sync_thread_once_dyn::InputEvent) -> bool + core::marker::Send + core::marker::Sync>
  // let result = <Option<Box<dyn FnMut(InputEvent) -> bool + Send + Sync>> as Clone>::clone(&d).unwrap()(in_ev3); // Call the closure with an argument
  // ↓↓ can't get &mut
  // let closure_ref = d(in_ev3); //&dyn core::ops::function::FnMut(various::sync::sync_thread_once_dyn::InputEvent) -> bool + core::marker::Send + core::marker::Sync
  // let closure_ref: &mut (dyn FnMut(InputEvent) -> bool + Send + Sync) = d.as_ref(); //&dyn core::ops::function::FnMut(various::sync::sync_thread_once_dyn::InputEvent) -> bool + core::marker::Send + core::marker::Sync
  // print_type_of(&closure_ref); //
  // let closure_deref = closure_ref(in_ev3);
  // // print_type_of(&closure_deref);
  // // p!("{:?}",d());







  /*
  let in_ev6 = InputEvent {code:32, up:true};
  let closure6:Box<HookSSFn> = Box::new(|input_event:InputEvent| true);
  static CELLSTATFN:OnceLock<Box<HookSSFn>> = OnceLock::new(); //
  if let Ok(set_closure6) = CELLSTATFN.set(closure6) { //fn set(&self, value: T) -> Result<(), T>
  // .unwrap() fails since fmt::Debug isn't implemented and can't implement it on dyn fnmut
    p!("everything is ok, even though I can't unwrap");
    print_type_of(&set_closure6);
  };
  let c6 = CELLSTATFN.into_inner(); //→Opt<T>  ✗ cannot move out of static item, OnceLoc<> doesn't implement the Copy trait
  */










  // let in_ev1 = InputEvent {code:32, up:true};
  // p!("{:?}",closure3.unwrap()(in_ev1));

  // let tttinner = CELL.into_inner(); //into_inner(self) -> Option<T>
  // if let Some(inner) = CELL.into_inner() {
    // let aaa = inner.unwrap()(in_ev);
  // };
  // print_type_of(&tttinner);
  // let tttfunres = *tttfun(in_ev);

  // let callback_out = move |input_event:InputEvent| {return false};
  // let five = Some(Box::new(callback_out));
  // print_type_of(&five);



  // assert_eq!(CELL.get(),Some(&12345),); // OnceLock now contains the value

}
pub fn sync() {
  // sync_once();
  // sync_arc();
  // sync_thread_once();
  // sync_thread_once_dyn();
  ttt1();
}

pub fn set_input_cb(cb:impl FnMut(char)) {
  p!("@set_input_cb");
  let a = '✗';
  // cb(a)
}
pub fn set_input_cb_box(cb_box:Box<impl FnMut(char)>) {
  p!("@set_input_cb_box");
  let a = '✗';
  let mut unbox_mut = *cb_box;
  unbox_mut(a);
}
pub fn set_input_cb_opt_box(cb_box:Option<Box<impl FnMut(char)>>) {
  p!("@set_input_cb_opt_box");
  let a = '✗';
  let mut unbox_mut = *cb_box.unwrap();
  unbox_mut(a);
}


use once_cell::sync	::Lazy;
use parking_lot    	::Mutex;
use std::sync      	::Arc;
// static CB_MUTEX: Lazy<Arc<Mutex<Option<Box<HookSSFn>>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));
type HookSSFn = dyn FnMut(char) -> bool + Send + Sync + 'static;
static CB_MUTEX: Mutex<Option<Box<HookSSFn>>> = Mutex::new(None);
pub fn set_cb(cb:impl FnMut(char) -> bool + Send + Sync + 'static) {
  p!("@set_cb: boxing and mutexing");
  // let mut h_cbl = CB_MUTEX.lock(); //lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex, core::option::Option<alloc::boxed::Box<dyn core::ops::function::FnMut(char) -> bool + core::marker::Send + core::marker::Sync>>>
  let mut h_cbl = CB_MUTEX.lock(); //only std mutex needs unwrap
  assert!(h_cbl.take().is_none(),"Only 1 external listener is allowed!");
  *h_cbl = Some(Box::new(cb)); // erases the type by boxing
  // print_type_of(&h_cbl); //std::sync::mutex::MutexGuard<core::option::Option<alloc::boxed::Box<dyn core::ops::function::FnMut(char) -> bool + core::marker::Send + core::marker::Sync>>>
}
pub fn run_cb(c:char) {
  p!("@run_cb: unboxing executing");
  let mut h_cbl = CB_MUTEX.lock(); //only std mutex needs unwrap
  print_type_of(&h_cbl);
  let cbopt = h_cbl.take(); // replace value with None, just .unwrap fails due to a lack of Copy trait on dyn fnMut
  print_type_of(&cbopt);
  let mut cbbox = cbopt.unwrap(); //
  print_type_of(&cbbox);
  cbbox(c);
  *h_cbl = Some(cbbox); // put it back
}

pub fn ttt1() {
  let mut my_string = String::from("sdf");
  let a = '✗';
  let     ref_it = || p!("{}",my_string);
  // let mut_it = |c| {my_string.push(c); p!("{}",my_string);}; // mutates my_string, so FnMut
  // mut_it(a); // ✗ requires 'let mut` since closure captures x by mutable reference – it has to to be able to modify it. When the closure is called, it receives self by mutable reference – it has to to be able to mutate x through the self pointer
  // let mut mut_it = |c| {my_string.push(c); p!("{}",my_string);}; // mutates my_string, so FnMut
  // mut_it(a);

  let mut_it = |c| {my_string.push(c); p!("{}",my_string);}; // mutates my_string, so FnMut
  // let box_mut = Box::new(mut_it);
  // let mut unbox_mut = *box_mut;
  // unbox_mut(a);

  // let box_mut = Box::new(mut_it);
  // set_input_cb_box(box_mut);

  let opt_box_mut = Some(Box::new(mut_it));
  // set_input_cb_opt_box(opt_box_mut);

  // set_input_cb(mut_it);
  // print_type_of(&mut_it);
  let     drp_it = || drop(my_string);

  struct InputEvent {code:u32, up:bool}
  let in_ev = InputEvent {code:32, up:true};
  // static AAPRESSED_KEYS: Lazy<Mutex<HashSet<i8>>> = Lazy::new(|| Mutex::new(HashSet::default()));
  // static AAPRESSED_KEYS1: Lazy<Mutex<Option<i8>>> = Lazy::new(|| Mutex::new(None));
  // static AAPRESSED_KEYS2: Lazy<Mutex<Option<Box<i8>>>> = Lazy::new(|| Mutex::new(None));
  // static AAPRESSED_KEYS3: Lazy<Mutex<Option<Box<HookSSFn>>>> = Lazy::new(|| Mutex::new(None));
  // static hook_mutex: Lazy<Mutex<HookFn> = Lazy::new(|| Mutex::new(None));
  let _kbhook = move |input_event:char| {
    p!("I'm _kbhook");
    true
  };
  set_cb(_kbhook);
  run_cb('✗');
  run_cb('✗');
  // set_cb(_kbhook);
}


// pub fn ttt() {
//   #[derive(Debug)] struct Rectangle {width:u32,height:u32}
//   let mut list = [Rectangle{width:10,height:1},Rectangle{width:3,height:5}];

//   let mut sort_operations = vec![];
//   let value = String::from("by key called");

//   list.sort_by_key(|r| { //fn sort_by_key<K, F>(&mut self, f: F) where F: FnMut(&T) -> K, K: Ord,
//     sort_operations.push(value);
//     r.width
//   });
//   println!("{:#?}", list);
// }


/*
https://users.rust-lang.org/t/how-can-threads-calling-ffi-code-in-rust-communicate-with-other-threads-or-modify-shared-state/36029/4
How can threads calling FFI code in Rust communicate with other threads or modify shared state?
use core::mem::MaybeUninit;
use core::ptr;
use std::sync::mpsc;
use std::sync::Once;

static mut CHANNEL: MaybeUninit<(mpsc::Sender<usize>, mpsc::Receiver<usize>)> = MaybeUninit::uninit();
static CHANNEL_INIT: Once = Once::new();

#[inline]
fn get_channel() -> &'static (mpsc::Sender<usize>, mpsc::Receiver<usize>) {
    CHANNEL_INIT.call_once(|| unsafe {ptr::write(CHANNEL.as_mut_ptr(), mpsc::channel());});
    unsafe {&*CHANNEL.as_ptr()}
}
fn main() {
    get_channel(); //safe because call_once will sync
}
 */