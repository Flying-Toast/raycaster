use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use common::input::{InputState, BistateInput, Input, ForeignInput};
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
    next_input_id: u32,
    keydown_cb: Closure<dyn FnMut(KeyboardEvent)>,
    keyup_cb: Closure<dyn FnMut(KeyboardEvent)>,
}

impl Controls {
    pub fn get_input(&mut self, dt: u8) -> Input {
        let foreign = ForeignInput::new(self.get_state(), dt);

        Input::new(foreign, self.next_id())
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
        window.add_event_listener_with_callback("keydown", keydown_cb.as_ref().unchecked_ref()).unwrap();

        let state_clone2 = state.clone();
        let bindings_clone2 = bindings.clone();
        let keyup_cb = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            if let Some(input) = bindings_clone2.key_to_bistate_input(&e.key()) {
                state_clone2.borrow_mut().bistates().set(input, false);
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);
        window.add_event_listener_with_callback("keyup", keyup_cb.as_ref().unchecked_ref()).unwrap();

        Self {
            state,
            bindings,
            next_input_id: 1,
            keydown_cb,
            keyup_cb,
        }
    }

    fn get_state(&self) -> InputState {
        self.state.borrow().clone()
    }

    fn next_id(&mut self) -> u32 {
        self.next_input_id = self.next_input_id.wrapping_add(1);

        self.next_input_id.wrapping_sub(1)
    }
}

impl Drop for Controls {
    fn drop(&mut self) {
        let window = window().unwrap();
        window.remove_event_listener_with_callback("keydown", self.keydown_cb.as_ref().unchecked_ref()).unwrap();
        window.remove_event_listener_with_callback("keyup", self.keyup_cb.as_ref().unchecked_ref()).unwrap();
    }
}
