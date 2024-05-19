use std::cell::Cell;
use std::rc::Rc;

use leptos::create_node_ref;
use leptos::html::canvas;
use leptos::html::Canvas;
use leptos::logging::*;
use leptos::IntoView;
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
#[component]
pub fn Signature() -> impl IntoView {
    let canvas_ref = create_node_ref::<Canvas>();
    create_effect(move |_| {
        log!("Create Effect");
        let canvas = canvas_ref.get().unwrap();
        let context = Rc::new(canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap());
        let pressed = Rc::new(Cell::new(false));
        {
            let context = context.clone();
            let canvas = canvas.clone();
            let pressed = pressed.clone();
            let on_resize = Closure::<dyn FnMut()>::new(move || {
                let context = context.clone();
                let canvas = canvas.clone();
                let pressed = pressed.clone();
                canvas.set_width(canvas.client_width() as u32);
                canvas.set_height(canvas.client_height() as u32);
                // if let Err(err) = start(context, canvas, pressed) {
                //     error!("Error: {:?}", err);
                // }
            });
            window().set_onresize(Some(on_resize.as_ref().unchecked_ref()));
            on_resize.forget();
        }
        let canvas = canvas.clone();
        start(context, canvas, pressed).unwrap();
    });

    view! {
        <canvas width="100%" height="100%" id="signature" ref=canvas_ref>
            "Unsupported browser"
        </canvas>
    }
}

fn start(
    context: Rc<CanvasRenderingContext2d>,
    canvas: HtmlElement<Canvas>,
    pressed: Rc<Cell<bool>>,
) -> Result<(), JsValue> {
    log!("Canvas Start");
    canvas.set_width(canvas.client_width() as u32);
    canvas.set_height(canvas.client_height() as u32);
    {
        let context = context.clone();
        let pressed = pressed.clone();
        register_closure(canvas.clone(), "mousedown", move |event| {
            context.set_stroke_style(&JsValue::from_str("white"));
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        });
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        register_closure(canvas.clone(), "mousemove", move |event| {
            if pressed.get() {
                context.set_stroke_style(&JsValue::from_str("white"));
                context.set_line_width(5.0);
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            }
        });
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        register_closure(canvas.clone(), "mouseup", move |event| {
            pressed.set(false);
            context.set_stroke_style(&JsValue::from_str("white"));
            context.set_line_width(5.0);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        });
    }
    Ok(())
}

fn register_closure<F: FnMut(web_sys::MouseEvent) + 'static>(
    canvas: HtmlElement<Canvas>,
    event: &str,
    callback: F,
) {
    let callback = Closure::<dyn FnMut(_)>::new(callback);
    canvas
        .add_event_listener_with_callback(event, callback.as_ref().unchecked_ref())
        .unwrap();
    callback.forget();
}
