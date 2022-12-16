use web_sys::{WebGlShader, WebGlProgram, WebGl2RenderingContext};
use std::rc::Rc;

use crate::renderer::Renderer;


pub struct Shader<'a> {
    obj: WebGlShader,
    shader_type: u32,
    is_compiled: bool,
    has_errors: bool,
    source: &'a str
}

impl Shader<'_> {
    pub fn new<'a>(rend: &Renderer, source: &'a str, shader_type: u32) -> Result<Shader<'a>, String> {
        let s = rend.context
            .create_shader(shader_type)
                    .ok_or_else(|| String::from("Renderer unable to create a new shader"))?;

        rend.context.shader_source(&s, source);
        rend.context.compile_shader(&s);

        let is_compiled = rend.context.get_shader_parameter(&s, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false);

        Ok(Shader {obj: s,
                   shader_type,
                   is_compiled,
                   has_errors: !is_compiled,
                   source
        })
    }

    pub fn compile(rend: &Renderer, source: &'a str) ->

}

struct ShaderProgram<'a> {
    obj: WebGlProgram,
    attached_shaders: Vec::<Rc::<Shader<'a>>>
}
