use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;


enum RunAgain { Yes, No }

pub struct Frontend {
    last_time: f64,
}

impl Frontend {
    pub fn new() -> Self {
        Self {
            last_time: 0.0,
        }
    }

    fn main_loop(&mut self, current_time: f64) -> RunAgain {
        // skip the first iteration because we don't have a time delta yet
        if self.last_time == 0.0 {
            self.last_time = current_time;
            return RunAgain::Yes;
        }

        let dt = current_time - self.last_time;
        self.last_time = current_time;

        // do stuff....

        RunAgain::Yes
    }

    pub fn start(mut self) {
        let closure = Rc::new(RefCell::new(None));
        let starter = closure.clone();

        *starter.borrow_mut() = Some(Closure::wrap(Box::new(move |current_time: f64| {
            if let RunAgain::Yes = self.main_loop(current_time) {
                request_frame(closure.borrow().as_ref().unwrap());
            } else {
                let _ = closure.borrow_mut().take();
            }
        }) as Box<dyn FnMut(f64)>));

        request_frame(starter.borrow().as_ref().unwrap());
    }
}

fn request_frame(cb: &Closure<dyn FnMut(f64)>) {
    web_sys::window().unwrap()
        .request_animation_frame(cb.as_ref().unchecked_ref())
        .unwrap();
}
