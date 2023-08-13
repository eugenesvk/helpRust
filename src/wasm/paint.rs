

use std::cell::Cell;
use std::rc::Rc;
fn wasm_bindgen_paint() -> Result<(), JsValue> { //https://github.com/rustwasm/wasm-bindgen/tree/main/examples/paint
  let doc   	= web_sys::window().unwrap().document().unwrap();
  let canvas	= doc.create_element("canvas")?.dyn_into::<web_sys::HtmlCanvasElement>()?;
  doc.body().unwrap().append_child(&canvas)?;
  canvas.set_width(640);
  canvas.set_height(480);
  canvas.style().set_property("border", "solid")?;
  let ctx = canvas.get_context("2d")?.unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
  let ctx = Rc::new(ctx);
  let pressed = Rc::new(Cell::new(false));
  { let ctx = ctx.clone();
    let pressed = pressed.clone();
    let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
      ctx.begin_path();
      ctx.move_to(event.offset_x() as f64, event.offset_y() as f64);
      pressed.set(true);
    });
    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  { let ctx = ctx.clone();
    let pressed = pressed.clone();
    let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
      if pressed.get() {
        ctx.line_to(event.offset_x() as f64, event.offset_y() as f64);
        ctx.stroke();
        ctx.begin_path();
        ctx.move_to(event.offset_x() as f64, event.offset_y() as f64);
      }
    });
    canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  { let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
      pressed.set(false);
      ctx.line_to(event.offset_x() as f64, event.offset_y() as f64);
      ctx.stroke();
    });
    canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
    closure.forget();
  }
  Ok(())
}