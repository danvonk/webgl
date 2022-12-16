use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlBuffer, WebGlVertexArrayObject};
use std::rc::Rc;

pub struct VertexBuffer {
    obj: WebGlBuffer,
    primitive_mode: u32
}

impl VertexBuffer {
}


// wrap an OpenGL VAO:
// keeps track of the vertex attributes
struct VertexArray {
    obj: WebGlVertexArrayObject,
    attached_vbo: Option::<Rc::<VertexBuffer>>
}


struct Mesh {
    vbo: WebGlBuffer,
    vao: WebGlVertexArrayObject
}
