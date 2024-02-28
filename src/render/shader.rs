// export implementation
pub use imp::*;

use crate::render::window::DrawContext as Context;

#[macro_export]
macro_rules! shader {
    (#type $shader_type:ident $code:literal) => {
        ShaderPart {
            type_: PartType::$shader_type,
            source_code: $code,
        }
    };
}

pub trait IParameter<Shader: IShader> {
    fn location(&self, shader: &Shader, name: &str) -> usize;

    fn set(&self, location: usize);
}

pub trait IShader {
    fn set_parameter(&self, param_name: &str, param_val: &dyn IParameter<Self>);
}

#[derive(Clone, Copy)]
pub enum PartType {
    Vertex,
    Fragment,
}

pub struct ShaderPart<'s> {
    pub type_: PartType,
    pub source_code: &'s str,
}

pub trait IShaderBuilder<'c>: Sized {
    type Out: IShader;
    fn new(ctx: &'c Context) -> Self;
    fn add_part(self, part: ShaderPart<'_>) -> Result<Self, String>;
    fn verify(self) -> Result<Self::Out, String>;
}

#[cfg(feature = "gl45")]
mod imp {
    use std::ptr::null_mut;

    use super::Context;
    use super::IParameter;
    use super::IShader;
    use super::IShaderBuilder;
    use super::PartType;
    use super::ShaderPart;
    use crate::math::Mat3;
    use crate::render::api as gl;
    use crate::render::api::types::GLenum;

    impl<'a> IParameter<Shader<'a>> for f32 {
        fn location(&self, shader: &Shader, name: &str) -> usize {
            let l =
                gl::verify! { gl::GetUniformLocation(shader.0.0, name.as_bytes().as_ptr() as _) };
            l as _
        }

        fn set(&self, location: usize) {
            gl::verify! { gl::Uniform1f(location as _, *self) };
        }
    }

    impl<'a> IParameter<Shader<'a>> for Mat3 {
        fn location(&self, shader: &Shader, name: &str) -> usize {
            let l =
                gl::verify! { gl::GetUniformLocation(shader.0.0, name.as_bytes().as_ptr() as _) };
            l as _
        }

        fn set(&self, location: usize) {
            gl::verify! { gl::UniformMatrix3fv(location as _, 1, gl::FALSE, self.as_ptr()) };
        }
    }

    impl<'a> IParameter<Shader<'a>> for i32 {
        fn location(&self, shader: &Shader, name: &str) -> usize {
            let l =
                gl::verify! { gl::GetUniformLocation(shader.0.0, name.as_bytes().as_ptr() as _) };
            l as _
        }

        fn set(&self, location: usize) {
            gl::verify! { gl::Uniform1i(location as _, *self) };
        }
    }

    pub struct Shader<'a>(gl::Program<'a>);

    impl<'a> Shader<'a> {
        pub(crate) fn bind(&self) {
            gl::verify! { gl::UseProgram(self.0.0) };
        }
    }

    impl<'a> IShader for Shader<'a> {
        fn set_parameter(&self, param_name: &str, param_val: &dyn IParameter<Self>) {
            let l = param_val.location(self, param_name);
            self.bind();
            param_val.set(l);
        }
    }

    impl PartType {
        fn api(self) -> GLenum {
            match self {
                PartType::Vertex => gl::VERTEX_SHADER,
                PartType::Fragment => gl::FRAGMENT_SHADER,
            }
        }
    }

    pub struct ShaderBuilder<'c> {
        p: gl::Program<'c>,
        ctx: &'c Context,
    }

    impl<'c> IShaderBuilder<'c> for ShaderBuilder<'c> {
        type Out = Shader<'c>;
        fn new(ctx: &'c Context) -> Self {
            Self {
                p: gl::Program::new(ctx),
                ctx,
            }
        }

        fn add_part(self, shader_part: ShaderPart<'_>) -> Result<Self, String> {
            let type_ = shader_part.type_;
            let source_code = shader_part.source_code;

            let shader = gl::Shader::new(type_.api(), self.ctx);
            let src = source_code.as_bytes().as_ptr() as _;
            let len = source_code.len() as _;
            let mut status = 0;
            gl::verify! {
                gl::ShaderSource(shader.0, 1, &src, &len);
                gl::CompileShader(shader.0);
                gl::GetShaderiv(shader.0, gl::COMPILE_STATUS, &mut status);
            }

            if status != gl::TRUE.into() {
                let mut buf_len = 0;
                gl::verify! { gl::GetShaderiv(shader.0, gl::INFO_LOG_LENGTH, &mut buf_len) };

                let mut buf = vec![0u8; buf_len as _];
                gl::verify! { gl::GetShaderInfoLog(shader.0, buf_len, null_mut(), buf.as_mut_ptr() as _) };
                // CString -> String is safe
                Err(String::from_utf8(buf).unwrap())
            } else {
                gl::verify! { gl::AttachShader(self.p.0, shader.0) };
                Ok(self)
            }
        }

        fn verify(self) -> Result<Self::Out, String> {
            let mut status = 0;
            gl::verify! {
                gl::LinkProgram(self.p.0);
                gl::GetProgramiv(self.p.0, gl::LINK_STATUS, &mut status);
            }

            if status != gl::TRUE.into() {
                let mut buf_len = 0;
                gl::verify! { gl::GetProgramiv(self.p.0, gl::INFO_LOG_LENGTH, &mut buf_len) };

                let mut buf = vec![0u8; buf_len as _];
                gl::verify! { gl::GetProgramInfoLog(self.p.0, buf_len, null_mut(), buf.as_mut_ptr() as _) };
                // CString -> String is safe
                Err(String::from_utf8(buf).unwrap())
            } else {
                Ok(Shader(self.p))
            }
        }
    }
}
