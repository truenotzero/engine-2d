use crate::math::Mat3;

use super::shader::IShader;
use super::texture::ITexture;
use crate::window::DrawContext as Context;

// export implementation
pub use imp::*;

pub trait ISprite<'a, Shader: IShader, Texture: ITexture<'a>>: Sized {
    fn new<'c: 'a>(ctx: &'c Context, texture: Texture) -> Self;
    fn draw(&self, shader: &Shader, sprite_matrix: Mat3);
}

#[cfg(feature = "gl45")]
mod imp {
    use std::ffi::c_void;
    use std::mem::size_of;
    use std::mem::size_of_val;

    use super::Context;
    use crate::math::Mat3;
    use crate::render::api as gl;
    use crate::render::api::Buf;
    use crate::render::api::Vao;
    use crate::render::shader::IShader;
    use crate::render::shader::Shader;
    use crate::render::texture::Texture;

    use super::ISprite;

    pub struct Sprite<'a> {
        vao: gl::Vao<'a>,
        /// vbo will never be read from
        /// it is merely used to track the lifetime
        /// of the vertex data
        #[allow(dead_code)]
        vbo: gl::Buf<'a>,
        tex: Texture<'a>,
    }

    impl<'a> ISprite<'a, Shader<'a>, Texture<'a>> for Sprite<'a> {
        fn new<'c: 'a>(ctx: &'c Context, tex: Texture<'a>) -> Self {
            let vao = Vao::new(ctx);
            let vbo = Buf::new(ctx);
            let vertex_data: [f32; 16] = [
                // aPos     aUV
                -1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 0.0, -1.0, -1.0, 0.0, 0.0,
            ];
            gl::verify! {
                gl::BindVertexArray(vao.0);
                gl::BindBuffer(gl::ARRAY_BUFFER, vbo.0);
                gl::BufferData(gl::ARRAY_BUFFER, size_of_val(&vertex_data) as _, vertex_data.as_ptr() as _, gl::STATIC_DRAW);
                gl::EnableVertexAttribArray(0);
                #[allow(clippy::erasing_op)]
                gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 4 * size_of::<f32>() as i32, (0 * size_of::<f32>()) as *const c_void);
                gl::EnableVertexAttribArray(1);
                gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 4 * size_of::<f32>() as i32, (2 * size_of::<f32>()) as *const c_void);
            }

            Self { vao, vbo, tex }
        }

        fn draw(&self, shader: &Shader, sprite_matrix: Mat3) {
            shader.bind();
            let slot = 0;
            self.tex.bind(slot);
            shader.set_parameter("uTexture", &(slot as i32));

            shader.set_parameter("uSprite", &sprite_matrix);

            gl::verify! {
                gl::BindVertexArray(self.vao.0);
                gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
            }
        }
    }
}
