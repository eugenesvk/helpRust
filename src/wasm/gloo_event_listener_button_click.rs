#![allow(non_snake_case,unused_imports,unused_mut,unused_variables,dead_code, unused_assignments)]
use wasm_bindgen::prelude::*;

// #[wasm_bindgen(start)] fn run() {
#[wasm_bindgen(start)] fn run() -> Result<(),JsValue> {
  let _ = mouse_listener_example_gloo();
  Ok(())
}

pub struct DelayedHelloButton {
  button  	: web::Element,
  on_click	: EventListener,
}
impl DelayedHelloButton {
  pub fn new(doc:&web::Document) -> Result<DelayedHelloButton, JsValue> {
    let button = doc.create_element("button")?; // Create a `<button>` element
    button.set_text_content(Some("DelayedHelloButton"));
    // button.set_inner_html("DelayedHelloButton");
    log("@DelayedHelloButton: button created");
    let button2 = button.clone(); // Listen to "click" events on the button
    log("@DelayedHelloButton: button cloned2 to listen to clicks");
    let on_click = EventListener::new(&button, "click", move |_event| {
      web::console::log_1(&"DelayedHelloButton clone3".into());
      let button3 = button2.clone(); // After 1sec timeout, update the button's text content
      log("@DelayedHelloButton: button cloned3 to update button content");
      Timeout::new(1_000, move || {
        log("@DelayedHelloButton: 1 sec passed");
        button3.set_text_content(Some("Hello from one second ago!"));
      }).forget();
    });
    Ok(DelayedHelloButton{button,on_click})
  }
}

fn mouse_listener_example_gloo() -> Result<(),JsValue> {
  // let game = Rc::new(RefCell::new(game));
  let win   	= web::window().expect("should have a window in this context");
  let doc   	= win.document()   .expect("window should have a document");
  let body  	= doc.body()       .expect("document should have a body");
  // let btn	= doc.create_element("button")?; // Create a `<button>` element
  // body.append_child(&btn)?;
  log("@key_listener_example_gloo: new delayed buton");
  let delayed_button	= DelayedHelloButton::new(&doc);
  log("@key_listener_example_gloo: new delayed buton unwrap");
  let delayed_button_res	= delayed_button.unwrap();
  log("@key_listener_example_gloo:   buton unwrap .button");
  let el_btn_delayed	= delayed_button_res.button;
  log("@key_listener_example_gloo:   buton unwrap .on_click");
  let evt_btn_delayed	= delayed_button_res.on_click;
  log("@key_listener_example_gloo:   buton unwrap .button append to body");
  evt_btn_delayed.keep_alive(); // forget keeps the EventListener alive forever, so it will never be dropped, button will not work without it
  body.append_child(&el_btn_delayed).unwrap(); //?;


  // https://github.com/Misfits-Rebels-Outcasts/WebAssemblyMan-rustwasm/blob/master/rustwasm/gloo_events_eventlistener/src/lib.rs
  let button = doc.create_element("button").unwrap();
  button.set_inner_html("Button");
  let on_click = EventListener::new(&button, "click", move |_event| {
    web::console::log_1(&"Hello World".into());
  });
  on_click.keep_alive();
  body.append_child(&button).unwrap();

  // let listener = EventListener::new(&doc, "keydown", move |event| {
  //   let event = event.dyn_into::<KeyboardEvent>().unwrap_throw();
  //   if event.key() == "Enter" {
  //     log("@key_listener_example_gloo: Enter pressed");
  //     // { let mut game = game.borrow_mut();
  //       // game.handle_input();}
  //     // let game = game.clone();
  //     // Timeout::new(0, move || {let mut game = game.borrow_mut();game.run();}).forget();
  //   }
  // });
  // listener.keep_alive();
  Ok(())
}



// If you do need to remove the DOM node, you can create a struct which contains the node and the EventListener:
struct MyButton {
  button: web_sys::HtmlButtonElement,
  _on_click: EventListener<'static>,

impl MyButton {
  fn remove(self) {
    // Remove node from DOM
    if let Some(parent) = self.button.parent_node() {
      parent.remove_child(&self.button).unwrap_throw();
    }
  }
}
// Now you just need to keep MyButton alive as long as you need it. You'll probably store it in some app-wide state:
struct State {
  my_button: MyButton,
}

// And this works because it'll consume self, thereby also consuming _on_click and dropping it all when the function finishes?
// Yes, exactly. So you don't need to touch _on_click at all (which is why EventListener is so great).
// That's also why it has the leading _, because otherwise Rust would complain that the on_click property is never used.
// P.S. You don't need to use Box::new(evt_listener).forget(), you can just use evt_listener.forget()
