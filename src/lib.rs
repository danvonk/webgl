
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};
use std::rc::Rc;

use gltype_derive::GLType;

mod gfx;
use gfx::mesh::{VertexArray, VertexBuffer};
use gfx::shader::{Shader, ShaderProgram};
use gfx::renderer::Renderer;
use gfx::types::GLType;


#[derive(GLType)]
struct Vertex {
    pos: [f32;2],
    col: f32
}


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let w = window
        .inner_width()
        .expect("no inner width")
        .as_f64()
        .expect("no inner width") as u32 - 20;
    let h = window
        .inner_height()
        .expect("no inner height")
        .as_f64()
        .expect("no inner height") as u32 - 20;

    canvas.set_width(w);
    canvas.set_height(h);

    let rend = Renderer::new(&canvas)?;
    rend.set_viewport(w,h);

    let vshader = Rc::new(Shader::new(&rend, include_str!("shaders/simple.vert"), WebGl2RenderingContext::VERTEX_SHADER)?);
    let fshader = Rc::new(Shader::new(&rend, include_str!("shaders/simple.frag"), WebGl2RenderingContext::FRAGMENT_SHADER)?);

    let prog = ShaderProgram::new(&rend, &vec![vshader, fshader])?;
    rend.use_program(&prog);

    let mut vbuf = VertexBuffer::new(&rend)?;
    let vertices = vec![
        -0.5, 0.5, // Top-let
        0.5, 0.5, // Top-right
        0.5, -0.5, // Bottom-right
        0.5, -0.5, // Bottom-right
        -0.5, -0.5, // Bottom-let
        -0.5, 0.5, // Top-let
    ];

    vbuf.data(&rend, &vertices);
    vbuf.set_attribute::<Vertex>();

    let _varray = VertexArray::new(&rend)?;

    //let position_attribute_location = context.get_attrib_location(&program, "position");

    //context.vertex_attrib_pointer_with_i32(
    //    position_attribute_location as u32,
    //    2,
    //    WebGl2RenderingContext::FLOAT,
    //    false,
    //    0,
    //    0,
    //);
    //context.enable_vertex_attrib_array(position_attribute_location as u32);

    //context.bind_vertex_array(Some(&vao));

    //let vert_count = (vertices.len() / 2) as i32;
    //draw(&context, vert_count);

    Ok(())
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
}