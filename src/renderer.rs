// renderer holds the context obj
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

pub struct Renderer {
    pub context: WebGl2RenderingContext,
}

impl Renderer {
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Result<Self, JsValue> {
        let ctx = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;

        Ok(Renderer { context: ctx })
    }

    pub fn set_viewport(&mut self, w: u32, h: u32) {
        self.context.viewport(0, 0, w as i32, h as i32);
    }

}
