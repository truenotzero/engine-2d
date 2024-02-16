pub trait IShader {
    fn draw(&self);
}

#[derive(Clone, Copy)]
pub enum Part {
    Vertex,
    Fragment,
}

pub trait IShaderBuilder: Default {
    type Out: IShader;
    fn add_part(self, part: Part, source_code: &str) -> Result<Self, String>;
    fn verify(self) -> Result<Self::Out, String>;
}

#[cfg(feature = "gl45")]
mod imp {
    use std::ptr::null_mut;

    use super::IShader;
    use super::IShaderBuilder;
    use super::Part;
    use crate::render::api as gl;
    use crate::render::api::types::GLenum;

    pub struct Shader(gl::Program);

    impl IShader for Shader {
        fn draw(&self) {
            todo!()
        }
    }

    impl Part {
        fn api(self) -> GLenum {
            match self {
                Part::Vertex => gl::VERTEX_SHADER,
                Part::Fragment => gl::FRAGMENT_SHADER,
            }
        }
    }

    #[derive(Default)]
    pub struct ShaderBuilder(gl::Program);

    impl IShaderBuilder for ShaderBuilder {
        type Out = Shader;

        fn add_part(self, type_: super::Part, source_code: &str) -> Result<Self, String> {
            let shader = gl::Shader::new(type_.api());
            let src = source_code.as_bytes().as_ptr() as _;
            let len = source_code.len() as _;
            let mut status = 0;
            gl::verify! {
                gl::ShaderSource(*shader, 1, &src, &len);
                gl::CompileShader(*shader);
                gl::GetShaderiv(*shader, gl::COMPILE_STATUS, &mut status);
            }

            if status == gl::FALSE.into() {
                let mut buf_len = 0;
                gl::verify! { gl::GetShaderiv(*shader, gl::INFO_LOG_LENGTH, &mut buf_len) };

                let mut buf = vec![0u8; buf_len as _];
                gl::verify! { gl::GetShaderInfoLog(*shader, buf_len, null_mut(), buf.as_mut_ptr() as _) };
                // CString -> String is safe
                Err(String::from_utf8(buf).unwrap())
            } else {
                gl::verify! { gl::AttachShader(*self.0, *shader) };
                Ok(self)
            }
        }

        fn verify(self) -> Result<Self::Out, String> {
            let mut status = 0;
            gl::verify! {
                gl::LinkProgram(*self.0);
                gl::GetProgramiv(*self.0, gl::LINK_STATUS, &mut status);
            }

            if status == gl::FALSE.into() {
                let mut buf_len = 0;
                gl::verify! { gl::GetProgramiv(*self.0, gl::INFO_LOG_LENGTH, &mut buf_len) };

                let mut buf = vec![0u8; buf_len as _];
                gl::verify! { gl::GetProgramInfoLog(*self.0, buf_len, null_mut(), buf.as_mut_ptr() as _) };
                // CString -> String is safe
                Err(String::from_utf8(buf).unwrap())
            } else {
                Ok(Shader(self.0))
            }
        }
    }
}

pub use imp::*;
