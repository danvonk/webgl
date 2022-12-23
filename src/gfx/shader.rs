use web_sys::{WebGlShader, WebGlProgram, WebGl2RenderingContext};
use std::rc::Rc;

use super::renderer::Renderer;

pub struct Shader<'a> {
    pub obj: WebGlShader,
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
}


pub struct ShaderProgram<'a> {
    pub obj: WebGlProgram,
    attached_shaders: Vec::<Rc::<Shader<'a>>>
}

impl ShaderProgram<'_> {
    pub fn new<'a>(rend: &Renderer, shaders: &Vec::<Rc::<Shader>>) -> Result<ShaderProgram<'a>, String> {
        let prog = rend.context.create_program().ok_or_else(|| String::from("Renderer unable to create a new shader program"))?;

        for s in shaders {
            rend.context.attach_shader(&prog, &s.obj);
        }

        rend.context.link_program(&prog);

        if rend.context.get_program_parameter(&prog, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
            {
                Ok(ShaderProgram {
                    obj: prog,
                    attached_shaders: shaders.to_vec()
                })
            } else {
                Err(String::from("Unable to link shader program"))
            }
    }
}
