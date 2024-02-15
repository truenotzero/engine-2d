use std::ffi::CStr;

use crate::gl;

// automatically calls glGetError() after every OpenGL call
// retrieves all errors, and if at least one error is found panics
// can be used to encapsulate a single or multiple OpenGL calls
// ex 1:
// gl_call! { gl::CreateProgram() };
// ex 2:
// gl_call! {
//      gl::CreateVertexArrays(1, &mut vao);
//      gl::BindVertexArray(vao);
// }
#[macro_export]
macro_rules! gl_call {
    () => {};
    ( $( $call:expr $(;)* )+ ) => {
        $({
            let e = unsafe { $call };
            $crate::shader::gl_call_impl(file!(), line!()).unwrap();
            e
        });+
    };
}
pub fn gl_call_impl(file: &'static str, line: u32) -> Result<(), &'static str> {
    let mut has_err = false;
    let mut err_str = "No error!";
    loop {
        let e = unsafe { gl::GetError() };
        err_str = match e {
            gl::NO_ERROR => break,
            gl::INVALID_ENUM => "GL_INVALID_ENUM",
            gl::INVALID_VALUE => "GL_INVALID_VALUE",
            gl::INVALID_OPERATION => "GL_INVALID_OPERATION",
            gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
            gl::OUT_OF_MEMORY => "GL_OUT_OF_MEMORY",
            gl::STACK_UNDERFLOW => "GL_STACK_UNDERFLOW",
            gl::STACK_OVERFLOW => "GL_STACK_OVERFLOW",
            _ => "Bad value from glGetError()",
        };
        has_err = true;
        println!("[{file}:{line}] {err_str}");
    }

    if !has_err {
        Ok(())
    } else {
        Err(err_str)
    }
}

pub enum Type {
    Vertex,
    Fragment,
}

pub trait Shader {
    fn new() -> Self where Self: Sized;
    fn load() -> Self where Self: Sized;
    fn add(&mut self, shader_type: Type, shader_code: &[u8]) -> Result<(), String>;
    fn verify(&self) -> Result<(), String>;

    // temporary
    fn bind(&self);
}

// openGL implementation
pub mod opengl_46 {
    use std::ptr::null_mut;

    use crate::gl;
    use crate::gl::types::GLuint;

    use super::Type;

    struct ShaderPart(GLuint);

    impl ShaderPart {
        pub fn new(shader_type: Type) -> Self {
            let gl_type = match shader_type {
                super::Type::Vertex => gl::VERTEX_SHADER,
                super::Type::Fragment => gl::FRAGMENT_SHADER,
            };

            Self(
                gl_call! { gl::CreateShader(gl_type) }
            )
        }
    }

    impl Drop for ShaderPart {
        fn drop(&mut self) {
            gl_call! { gl::DeleteShader(self.0) };
        }
    }

    pub struct Shader(GLuint);

    impl super::Shader for Shader {
        fn new() -> Self where Self: Sized {
            Self(
                gl_call! { gl::CreateProgram() },
            )
        }

        fn load() -> Self where Self: Sized {
            Self::new()
        }

        fn add(&mut self, shader_type: super::Type, shader_code: &[u8]) -> Result<(), String> {
            let shader = ShaderPart::new(shader_type);
            let mut status = 1;
            gl_call! { 
                gl::ShaderSource(shader.0, 1, &(shader_code.as_ptr() as _), &(shader_code.len() as _));
                gl::CompileShader(shader.0);
                gl::GetShaderiv(shader.0, gl::COMPILE_STATUS, &mut status);
             };

            if status == gl::FALSE.into() {
                let mut buf_len = 0;
                gl_call! { gl::GetShaderiv(shader.0, gl::INFO_LOG_LENGTH, &mut buf_len) };
                let mut buf = vec![0u8; buf_len as _];
                gl_call! { gl::GetShaderInfoLog(shader.0, buf_len, null_mut(), buf.as_mut_ptr() as _)}
                return Err(String::from_utf8(buf).unwrap());
            }

            gl_call! { gl::AttachShader(self.0, shader.0) };
            Ok(())
        }

        fn verify(&self) -> Result<(), String> {
            let mut status = 0;
            gl_call! {
                gl::LinkProgram(self.0);
                gl::GetProgramiv(self.0, gl::LINK_STATUS, &mut status);
            }

            if status == gl::FALSE.into() {
                let mut buf_len = 0;
                gl_call! { gl::GetProgramiv(self.0, gl::INFO_LOG_LENGTH, &mut buf_len) };
                let mut buf = vec![0u8; buf_len as _];
                gl_call! { gl::GetProgramInfoLog(self.0, buf_len, null_mut(), buf.as_mut_ptr() as _)}
                return Err(String::from_utf8(buf).unwrap());
            }

            Ok(())
        }

        fn bind(&self) {
            gl_call! { gl::UseProgram(self.0) };
        }

        
    }

    impl Drop for Shader {
        fn drop(&mut self) {
            gl_call! { gl::DeleteProgram(self.0) };
        }
    }
}


