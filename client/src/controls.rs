use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use common::input::{InputState, BistateInput};
use web_sys::{KeyboardEvent, window};


#[derive(Debug)]
pub struct Controls {
    state: Rc<RefCell<InputState>>,
    #[allow(dead_code)]
    keydown_cb: Closure<dyn FnMut(KeyboardEvent)>,
    #[allow(dead_code)]
    keyup_cb: Closure<dyn FnMut(KeyboardEvent)>,
}

impl Controls {
    pub fn get_state(&mut self) -> InputState {
        (*self.state.borrow()).clone()
    }

    pub fn new() -> Self {
        let state = Rc::new(RefCell::new(InputState::new()));
        let window = window().unwrap();

        let state_clone1 = state.clone();
        let keydown_cb = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            if e.repeat() { return; }

            let input = match e.key().as_str() { //TODO: abstract this
                "w" => BistateInput::MoveForwards,
                "a" => BistateInput::MoveLeft,
                "s" => BistateInput::MoveBackwards,
                "d" => BistateInput::MoveRight,
                _ => return,
            };
            state_clone1.borrow_mut().bistates().set(input, true);
        }) as Box<dyn FnMut(KeyboardEvent)>);
        window.set_onkeydown(Some(keydown_cb.as_ref().unchecked_ref()));

        let state_clone2 = state.clone();
        let keyup_cb = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            let input = match e.key().as_str() { //TODO: abstract this
                "w" => BistateInput::MoveForwards,
                "a" => BistateInput::MoveLeft,
                "s" => BistateInput::MoveBackwards,
                "d" => BistateInput::MoveRight,
                _ => return,
            };
            state_clone2.borrow_mut().bistates().set(input, false);
        }) as Box<dyn FnMut(KeyboardEvent)>);
        window.set_onkeyup(Some(keyup_cb.as_ref().unchecked_ref()));

        Self {
            state,
            keydown_cb,
            keyup_cb,
        }
    }
}

impl Drop for Controls {
    fn drop(&mut self) {
        let window = window().unwrap();
        window.set_onkeydown(None);
        window.set_onkeyup(None);
    }
}
