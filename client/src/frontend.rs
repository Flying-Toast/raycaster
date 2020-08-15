use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use std::cell::RefCell;
use crate::network::{Network, NetworkStatus};
use web_sys::window;


enum RunAgain { Yes, No }

pub struct Frontend {
    last_time: f64,
    network: Network,
}

impl Frontend {
    pub fn new() -> Self {
        Self {
            last_time: 0.0,
            network: Network::new(),
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

        let status = self.network.status();
        // don't run mainloop until network is connected
        if let NetworkStatus::Connecting = status {
            return RunAgain::Yes;
        }

        let messages = self.network.drain_messages();
        console_log!("MESSAGES: {:#?}", messages);

        match status {
            NetworkStatus::Connected => RunAgain::Yes,
            _ => RunAgain::No,
        }
    }

    pub fn start(mut self) {
        let hostname = window().unwrap().location().hostname().unwrap();
        self.network.connect(&hostname, 8000, false);

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
    window().unwrap()
        .request_animation_frame(cb.as_ref().unchecked_ref())
        .unwrap();
}
