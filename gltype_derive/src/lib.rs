use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};
use web_sys::WebGl2RenderingContext;

#[proc_macro_derive(GLType)]
pub fn gltype_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    impl_gltype_derive(&ast)
}

fn impl_gltype_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = &ast.data;

    let vtx_attrs = match fields {
        syn::Data::Struct(ds) => impl_field_derive(&ds.fields),
        _ => Vec::new(),
    };

    let mut tk = Vec::new();
    for v in vtx_attrs.iter() {
        // get rid of this silliness
        let a = v.0.clone();
        let b = v.1;
        let c = v.2;
        let d = quote! {
            temp_vec.push((String::from(stringify!(#a)), #b, #c));
        };
        tk.push(d);
    }

    let gen = quote! {
        impl GLType for #name {
            fn vertex_attributes() -> Vec::<(String,i32,i32)> {
                let mut temp_vec = Vec::<(String, i32, i32)>::new();
                #(#tk)*
                temp_vec
            }
        }
    };
    gen.into()
}

fn impl_field_derive(field: &syn::Fields) -> Vec<(String, i32, i32)> {
    let vv: Vec<(String, i32, i32)> = field
        .iter()
        .map(|f| (f.ident.clone().unwrap().to_string(), impl_gl_type(&f.ty), 0))
        .collect();

    vv
}

fn impl_gl_type(t: &syn::Type) -> i32 {
    match t {
        syn::Type::Verbatim(v) => {
            match v.to_string().as_str() {
                "f32" => WebGl2RenderingContext::FLOAT as i32,
                "i32" => WebGl2RenderingContext::INT as i32,
                "u32" => WebGl2RenderingContext::UNSIGNED_INT as i32,
                _ => 0
            }
        },
        _ => 0
    }
}