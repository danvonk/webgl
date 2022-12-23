use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlBuffer, WebGlVertexArrayObject};
use std::rc::Rc;

use super::{renderer::Renderer};

pub struct VertexAttribute<'a> {
    name: &'a str,
    size: i32,
    type_: u32,
    normalized: bool,
    stride: i32,
    offset: i32
}

pub struct VertexBuffer<'a> {
    obj: WebGlBuffer,
    primitive_mode: u32,
    attrs: Vec::<VertexAttribute<'a>>
}

impl VertexBuffer<'_> {
    pub fn new(rend: &Renderer) -> Result<Self, String> {
        let b = rend.context.create_buffer().ok_or_else(|| String::from("Unable to create new vertex buffer."))?;

        Ok (
            VertexBuffer { 
                obj: b, 
                primitive_mode: WebGl2RenderingContext::TRIANGLES,
                attrs: vec![]
            }
        )
    }

    pub fn bind(&self, rend: &Renderer) {
        rend.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.obj));
    }
     
    pub fn data(&self, rend: &Renderer, data: &Vec<f32>) {
        self.bind(rend);
        unsafe {
                let positions_array_buf_view = js_sys::Float32Array::view(&data);
                rend.context.buffer_data_with_array_buffer_view(
                    WebGl2RenderingContext::ARRAY_BUFFER,
                    &positions_array_buf_view,
                    WebGl2RenderingContext::STATIC_DRAW,
                );
            }
    }

    pub fn set_attribute<T: super::types::GLType>(&mut self) {
        self.attrs = T::vertex_attributes().iter().map(|v| {
            VertexAttribute { name: &v.0, size: v.2, type_: v.1 as u32, normalized: false, stride: 0, offset: 0 }
        }).collect();
    }

}


// wrap an OpenGL VAO:
// keeps track of the vertex attributes
pub struct VertexArray {
    obj: WebGlVertexArrayObject,
//    attached_vbo: Option::<Rc::<VertexBuffer>>
}

impl VertexArray {
    pub fn new(rend: &Renderer) -> Result<VertexArray, String> {
        let vao = rend.context.create_vertex_array()
        .ok_or_else(|| String::from("Could not create vertex array object."))?;

        Ok (
            VertexArray { obj: vao}
        )
    }

    pub fn bind(&self, rend: &Renderer) {
        rend.context.bind_vertex_array(Some(&self.obj));
    }
}


pub struct Mesh {
    vbo: WebGlBuffer,
    vao: WebGlVertexArrayObject
}

impl Mesh {
    pub fn draw() {
    }

}
