#[cfg(feature = "gl45")]
mod gl45;

#[cfg(feature = "gl45")]
pub use gl45::*;

// TODO: windowing API
// for now, due to time constraints
// re-export glfw
#[cfg(feature = "glfw")]
pub mod window {
    pub use crate::glfw::*;
}
