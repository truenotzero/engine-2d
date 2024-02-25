use std::ffi::c_void;
use std::marker::PhantomData;

use crate::render::Context;

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
            let _e = unsafe { $call };
            $crate::render::api::verify_impl(file!(), line!(), stringify!($call)).unwrap();
            _e
        });+
    };
}

pub(crate) use verify;

pub fn verify_impl(file: &str, line: u32, call: &str) -> Result<(), &'static str> {
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
        println!("[{file}:{line}] {err_str} | {call}");
    }

    if !has_err {
        Ok(())
    } else {
        Err(err_str)
    }
}

//
pub fn init<'w>(f: impl FnMut(&'static str) -> *const c_void) -> Context<'w> {
    gl::load_with(f);
    verify! {
        gl::Enable(gl::FRAMEBUFFER_SRGB);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    Context(PhantomData)
}

pub fn clear() {
    verify! {
        // set by default to this value anyway
        // gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub struct Vao<'a>(pub GLuint, PhantomData<&'a ()>);

impl<'a> Vao<'a> {
    pub fn new<'c: 'a>(_: Context<'c>) -> Self {
        let mut n = 0;
        verify! { gl::CreateVertexArrays(1, &mut n) };
        Self(n, PhantomData)
    }
}

impl<'a> Drop for Vao<'a> {
    fn drop(&mut self) {
        verify! { gl::DeleteVertexArrays(1, &self.0) };
    }
}

pub struct Buf<'a>(pub GLuint, PhantomData<&'a ()>);

impl<'a> Buf<'a> {
    pub fn new<'c: 'a>(_: Context<'c>) -> Self {
        let mut n = 0;
        verify! { gl::CreateBuffers(1, &mut n) };
        Self(n, PhantomData)
    }
}

impl<'a> Drop for Buf<'a> {
    fn drop(&mut self) {
        verify! { gl::DeleteBuffers(1, &self.0) };
    }
}

pub struct Shader<'a>(pub GLuint, PhantomData<&'a ()>);

impl<'a> Shader<'a> {
    pub fn new<'c :'a>(type_: GLenum, _: Context<'c>) -> Self {
        let n = verify! { gl::CreateShader(type_)};
        Self(n, PhantomData)
    }
}

impl<'a> Drop for Shader<'a> {
    fn drop(&mut self) {
        verify! { gl::DeleteShader(self.0) };
    }
}

pub struct Program<'a>(pub GLuint, PhantomData<&'a ()>);

impl<'a> Program<'a> {
    pub fn new<'c: 'a>(_: Context<'c>) -> Self {
        let n = verify! { gl::CreateProgram() };
        Self(n, PhantomData)
    }
}

impl<'a> Drop for Program<'a> {
    fn drop(&mut self) {
        verify! { gl::DeleteProgram(self.0) };
    }
}

pub struct Texture<'a>(pub GLuint, PhantomData<&'a ()>);

impl<'a> Texture<'a> {
    pub fn new<'c>(_: Context<'c>) -> Self {
        let mut n = 0;
        verify! { gl::GenTextures(1, &mut n) };
        Self(n, PhantomData)
    }
}

impl<'a> Drop for Texture<'a> {
    fn drop(&mut self) {
        verify! { gl::DeleteTextures(1, &self.0) }
    }
}
