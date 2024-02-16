use std::ffi::c_void;
use std::ops::Deref;

use self::gl::types::GLenum;
use self::gl::types::GLuint;

// contains the raw OpenGL 4.5 bindings as well as RAII containers for OpenGL objects

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// re-export publically under namespace gl45
pub use gl::*;

// automatically verifies every OpenGL call (by calling glGetError())
// retrieves all errors, and if at least one error is found panics
// can be used to encapsulate a single or multiple OpenGL calls
// ex 1:
// gl::verify! { gl::CreateProgram() };
// ex 2:
// gl::verify! {
//      gl::CreateVertexArrays(1, &mut vao);
//      gl::BindVertexArray(vao);
// }
macro_rules! verify {
    () => {};
    ( $( $call:expr $(;)* )+ ) => {
        $({
            let e = unsafe { $call };
            $crate::render::api::call_impl(file!(), line!()).unwrap();
            e
        });+
    };
}

pub(crate) use verify;

pub fn call_impl(file: &'static str, line: u32) -> Result<(), &'static str> {
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

//
pub fn init(f: impl FnMut(&'static str) -> *const c_void) {
    gl::load_with(f)
}

pub struct Vao(GLuint);

impl Default for Vao {
    fn default() -> Self {
        let mut n = 0;
        verify! { gl::CreateVertexArrays(1, &mut n) };
        Self(n)
    }
}

impl Deref for Vao {
    type Target = GLuint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        verify! { gl::DeleteVertexArrays(1, &self.0) };
    }
}

pub struct Buf(GLuint);

impl Default for Buf {
    fn default() -> Self {
        let mut n = 0;
        verify! { gl::CreateBuffers(1, &mut n) };
        Self(n)
    }
}

impl Deref for Buf {
    type Target = GLuint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for Buf {
    fn drop(&mut self) {
        verify! { gl::DeleteBuffers(1, &self.0) };
    }
}

pub struct Shader(GLuint);

impl Shader {
    pub fn new(type_: GLenum) -> Self {
        let n = verify! { gl::CreateShader(type_)};
        Self(n)
    }
}

impl Deref for Shader {
    type Target = GLuint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        verify! { gl::DeleteShader(self.0) };
    }
}

pub struct Program(GLuint);

impl Default for Program {
    fn default() -> Self {
        let n = verify! { gl::CreateProgram()};
        Self(n)
    }
}

impl Deref for Program {
    type Target = GLuint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        verify! { gl::DeleteProgram(self.0) };
    }
}
