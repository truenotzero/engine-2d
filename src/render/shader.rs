// export implementation
pub use imp::*;

pub trait IParameter<Shader: IShader> {
    fn location(&self, shader: &Shader, name: &str) -> usize;

    fn set(&self, location: usize);
}

pub trait IShader {
    fn set_parameter(&self, param_name: &str, param_val: &dyn IParameter<Self>);
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

    use super::IParameter;
    use super::IShader;
    use super::IShaderBuilder;
    use super::Part;
    use crate::math::Mat3;
    use crate::render::api as gl;
    use crate::render::api::types::GLenum;

    impl IParameter<Shader> for f32 {
        fn location(&self, shader: &Shader, name: &str) -> usize {
            let l = gl::verify! { gl::GetUniformLocation(*shader.0, name.as_bytes().as_ptr() as _) };
            l as _
        }

        fn set(&self, location: usize) {
            gl::verify! { gl::Uniform1f(location as _, *self) };
        }
    }

    impl IParameter<Shader> for Mat3 {
        fn location(&self, shader: &Shader, name: &str) -> usize {
            let l = gl::verify! { gl::GetUniformLocation(*shader.0, name.as_bytes().as_ptr() as _) };
            l as _
        }

        fn set(&self, location: usize) {
            gl::verify! { gl::UniformMatrix3fv(location as _, 1, gl::FALSE, self.as_ptr()) };
        }
    }

    impl IParameter<Shader> for i32 {
        fn location(&self, shader: &Shader, name: &str) -> usize {
            let l = gl::verify! { gl::GetUniformLocation(*shader.0, name.as_bytes().as_ptr() as _) };
            l as _
        }

        fn set(&self, location: usize) {
            gl::verify! { gl::Uniform1i(location as _, *self) };
        }
    }

    pub struct Shader(gl::Program);

    impl Shader {
        pub(crate) fn bind(&self) {
            gl::verify! { gl::UseProgram(*self.0) };
        }
    }

    impl IShader for Shader {
        fn set_parameter(&self, param_name: &str, param_val: &dyn IParameter<Self>) {
            let l = param_val.location(self, param_name);
            self.bind();
            param_val.set(l);
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

            if status != gl::TRUE.into() {
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

            if status != gl::TRUE.into() {
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

