use std::ops::Index;

use crate::math::IVec2;

#[derive(Clone, Copy)]
pub enum Filtering {
    Linear,
    Nearest,
}

pub trait ITexture {
    fn width(&self) -> i32;
    fn height(&self) -> i32;
}

pub trait IAtlasColumn: Index<usize> {
    type Texture: ITexture;
}

pub trait IAtlas: Index<usize> {
    type Texture: ITexture;
}

pub trait IAtlasBuilder: Default {
    type Out: IAtlas;

    fn texture_dimensions(self, dim: IVec2) -> Result<Self, ()>;
    fn build(self) -> Result<Self::Out, ()>;
}

#[cfg(feature = "gl45")]
mod imp {
    use std::ops::Index;

    use crate::math::IVec2;
    use crate::render::api as gl;

    use super::IAtlas;
    use super::IAtlasBuilder;
    use super::ITexture;

    pub struct Texture;

    impl ITexture for Texture {
        fn width(&self) -> i32 {
            todo!()
        }

        fn height(&self) -> i32 {
            todo!()
        }
    }

    pub struct AtlasRow;

    pub struct Atlas;

    impl Index<usize> for Atlas {
        type Output=AtlasRow;

        fn index(&self, index: usize) -> &Self::Output {
            todo!()
        }
    }

    impl IAtlas for Atlas {
        type Texture=Texture;
    }

    #[derive(Default)]
    pub struct AtlasBuilder {
        o: gl::Texture,
        dim: Option<IVec2>,
    }

    impl IAtlasBuilder for AtlasBuilder {
        type Out = Atlas;

        fn texture_dimensions(self, dim: IVec2) -> Result<Self, ()> {
            Ok(Self {
                dim: Some(dim),
                ..self
            })
        }

        fn build(self) -> Result<Self::Out, ()> {
            gl::verify! {
                gl::TexImage2D(gl::TEXTURE_RECTANGLE, 0, gl::RGBA as _, todo!(), todo!(), 0, gl::RGBA, gl::UNSIGNED_BYTE, todo!());
            }

            todo!()
        }
    }
}

pub use imp::*;
