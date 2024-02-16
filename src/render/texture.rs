use crate::math::IVec2;

#[derive(Clone, Copy)]
pub enum Filtering {
    Linear,
    Nearest,
}

pub trait ITexture {}

pub trait IAtlas {}

pub trait IAtlasBuilder: Default {
    type Out: IAtlas;

    fn texture_dimensions(dim: IVec2) -> Result<Self, ()>;
    fn build(self) -> Result<Self::Out, ()>;
}

#[cfg(feature = "gl45")]
mod imp {
    use crate::math::IVec2;
    use crate::render::api as gl;

    use super::IAtlas;
    use super::IAtlasBuilder;

    pub struct Atlas;

    impl IAtlas for Atlas {}

    #[derive(Default)]
    pub struct AtlasBuilder(gl::Texture);

    impl IAtlasBuilder for AtlasBuilder {
        type Out = Atlas;

        fn texture_dimensions(dim: IVec2) -> Result<Self, ()> {
            todo!()
        }

        fn build(self) -> Result<Self::Out, ()> {
            todo!()
        }
    }
}

pub use imp::*;
