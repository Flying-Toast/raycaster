use common::entity::EntityID;
use common::gamestate::GameState;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::{JsCast, JsValue};

pub struct Renderer {
    ctx: CanvasRenderingContext2d,
    canvas: HtmlCanvasElement,
}

impl Renderer {
    pub fn new() -> Self {
        let canvas = window().unwrap()
            .document().unwrap()
            .query_selector("#canvas").unwrap().unwrap()
            .dyn_into::<HtmlCanvasElement>().unwrap();

        let ctx = canvas.get_context("2d").unwrap().unwrap()
            .dyn_into::<CanvasRenderingContext2d>().unwrap();

        let this = Self {
            ctx,
            canvas,
        };
        this.resize();

        this
    }

    /// Renders `state` as seen by the `perspective` entity
    pub fn render(&mut self, state: &GameState, perspective: EntityID) {
        let ctx = &self.ctx;
        let canvas = &self.canvas;
        let canvas_width = canvas.width();
        let canvas_height = canvas.height();

        ctx.clear_rect(0.0, 0.0, canvas_width as f64, canvas_height as f64);

        ctx.set_fill_style(&JsValue::from_str("purple"));//REMOVE
        ctx.fill_rect(0.0, 0.0, canvas_width as f64, canvas_height as f64);//REMOVE
    }

    fn resize(&self) {
        const MAX_WIDTH: u32 = 1600;
        const MAX_HEIGHT: u32 = 1600;
        let inner_width = window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32;
        let inner_height = window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32;
        self.canvas.set_width(core::cmp::min(MAX_WIDTH, inner_width));
        self.canvas.set_height(core::cmp::min(MAX_HEIGHT, inner_height));
    }
}
