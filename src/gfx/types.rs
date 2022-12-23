use web_sys::WebGl2RenderingContext;
//pub trait GLType {
//    fn vertex_attributes() -> Vec::<(String,i32,i32)>;
//}
pub trait GlType {
    fn gl_type(&self) -> u32;
    fn gl_size(&self) -> i32
    where
        Self: Sized;
}

impl GlType for f32 {
    fn gl_type(&self) -> u32 {
        WebGl2RenderingContext::FLOAT
    }
    fn gl_size(&self) -> i32 {
        1
    }
}

impl GlType for u32 {
    fn gl_type(&self) -> u32 {
        WebGl2RenderingContext::UNSIGNED_INT
    }
    fn gl_size(&self) -> i32 {
        1
    }
}

impl GlType for [f32;2] {
    fn gl_type(&self) -> u32 {
        WebGl2RenderingContext::UNSIGNED_INT
    }
    fn gl_size(&self) -> i32 {
        2
    }
}

pub fn gl_type<U>(x: U) -> u32
where
    U: GlType,
{
    x.gl_type()
}

pub fn gl_size<U>(x: U) -> i32
where
    U: GlType,
{
    x.gl_size()
}

