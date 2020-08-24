use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use common::input::{InputState, BistateInput};
use web_sys::{KeyboardEvent, window};


#[derive(Debug)]
struct Keybindings {
    forwards: String,
    backwards: String,
    left: String,
    right: String,
}

impl Keybindings {
    pub fn default() -> Self {
        Self {
            forwards: "w".to_string(),
            backwards: "s".to_string(),
            left: "a".to_string(),
            right: "d".to_string(),
        }
    }

    pub fn key_to_bistate_input(&self, key: &str) -> Option<BistateInput> {
        let key = key.to_ascii_lowercase();

        Some(match key {
            _ if key == self.forwards => BistateInput::MoveForwards,
            _ if key == self.backwards => BistateInput::MoveBackwards,
            _ if key == self.left => BistateInput::MoveLeft,
            _ if key == self.right => BistateInput::MoveRight,

            _ => return None,
        })
    }
}

#[derive(Debug)]
pub struct Controls {
    state: Rc<RefCell<InputState>>,
    bindings: Rc<Keybindings>,
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
        let bindings = Rc::new(Keybindings::default());
        let window = window().unwrap();

        let state_clone1 = state.clone();
        let bindings_clone1 = bindings.clone();
        let keydown_cb = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            if e.repeat() { return; }

            if let Some(input) = bindings_clone1.key_to_bistate_input(&e.key()) {
                state_clone1.borrow_mut().bistates().set(input, true);
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);
        window.set_onkeydown(Some(keydown_cb.as_ref().unchecked_ref()));

        let state_clone2 = state.clone();
        let bindings_clone2 = bindings.clone();
        let keyup_cb = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            if let Some(input) = bindings_clone2.key_to_bistate_input(&e.key()) {
                state_clone2.borrow_mut().bistates().set(input, false);
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);
        window.set_onkeyup(Some(keyup_cb.as_ref().unchecked_ref()));

        Self {
            state,
            bindings,
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
