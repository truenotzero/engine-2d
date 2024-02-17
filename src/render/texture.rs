use std::ffi::c_void;
use std::path::Path;

use crate::math::UVec2;

use image::GenericImageView;

// export implementation
pub use imp::*;


#[derive(Clone, Copy)]
pub enum Filtering {
    Linear,
    Nearest,
}

pub trait ITexture{
    fn from_file(path: &Path) -> Result<Self, ()> where Self: Sized {
        let img = image::open(path).map_err(|_| ())?;
        let img = img.flipv();
        let size = img.dimensions().into();
        let pixel_data = img.as_bytes().as_ptr();

        Ok(Self::from_memory(size, pixel_data as _))
    }

    fn from_memory(size: UVec2, pixel_data: *const c_void) -> Self;
    fn set_filtering(&mut self, option: Filtering);
    fn size(&self) -> UVec2;
}

pub trait ITextureBuilder: Default {
    type Out: ITexture;

    fn path(self, path: &Path) -> Result<Self, ()>;
    fn build(self) -> Result<Self::Out, ()>;
}

#[cfg(feature = "gl45")]
mod imp {
    use std::ffi::c_void;

    use crate::math::UVec2;
    use crate::render::api::types::GLenum;
    use crate::render::api as gl;

    use super::Filtering;
    use super::ITexture;

    impl Filtering {
        fn api(self) -> GLenum {
            match self {
                Filtering::Linear => gl::LINEAR,
                Filtering::Nearest => gl::NEAREST,
            }
        }
    }

    pub struct Texture {
        o: gl::Texture,
        size: UVec2,
    }

    impl Texture {
        pub(crate) fn bind(&self, slot: u32) {
            gl::verify! { 
                gl::ActiveTexture(gl::TEXTURE0 + slot);
                gl::BindTexture(gl::TEXTURE_2D, *self.o);
            }
        }
    }

    impl ITexture for Texture {
        fn from_memory(size: UVec2, pixel_data: *const c_void) -> Self {
            let o = gl::Texture::default();
            gl::verify! {
                gl::BindTexture(gl::TEXTURE_2D, *o);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::SRGB_ALPHA as _, size.x as _, size.y as _, 0, gl::RGBA, gl::UNSIGNED_BYTE, pixel_data);
            }

            Self {
                o,
                size,
            }
        }

        fn set_filtering(&mut self, option: super::Filtering) {
            gl::verify! {
                gl::BindTexture(gl::TEXTURE_RECTANGLE, *self.o);
                gl::TexParameteri(gl::TEXTURE_RECTANGLE, gl::TEXTURE_MIN_FILTER, option.api() as _)
            }
        }

        fn size(&self) -> UVec2 {
            self.size
        }
    }
}
