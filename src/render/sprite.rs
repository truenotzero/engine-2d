use crate::math::Mat3;
use crate::math::Vec2;

use super::shader::IShader;
use super::texture::ITexture;

// export implementation
pub use imp::*;

pub trait ISprite<Shader: IShader, Texture: ITexture> {
    fn draw(&self, shader: &Shader);

    fn set_scale(&mut self, scale: Vec2);
    fn scale(&self) -> Vec2;

    fn set_rotation(&mut self, rotation: f32);
    fn rotation(&self) -> f32;

    fn set_position(&mut self, pos: Vec2);
    fn position(&self) -> Vec2;


    fn set_texture(&mut self, texture: Texture);

    fn matrix(&self) -> Mat3 {
        Mat3::translate(self.position()) * Mat3::scale(self.scale())
    }
}

#[cfg(feature = "gl45")]
mod imp {
    use std::ffi::c_void;
    use std::mem::size_of;
    use std::mem::size_of_val;

    use crate::math::Mat3;
    use crate::math::Vec2;
    use crate::render::api as gl;
    use crate::render::shader::IShader;
    use crate::render::shader::Shader;
    use crate::render::texture::Texture;

    use super::ISprite;

    #[derive(Default)]
    pub struct Sprite {
        vao: gl::Vao,
        vbo: gl::Buf,
        tex: Option<Texture>,
        pos: Vec2,
        scale: Vec2,
        rotation: f32,
    }

    impl Sprite {
        pub fn init(&mut self) {
            self.scale = (1.0, 1.0).into();
            let vertex_data: [f32; 16] = [
                // aPos     aUV
                -1.0, 1.0,  0.0, 1.0,
                1.0, 1.0,   1.0, 1.0,
                1.0, -1.0,  1.0, 0.0,
                -1.0, -1.0, 0.0, 0.0,
            ];
            gl::verify! {
                gl::BindVertexArray(*self.vao);
                gl::BindBuffer(gl::ARRAY_BUFFER, *self.vbo);
                gl::BufferData(gl::ARRAY_BUFFER, size_of_val(&vertex_data) as _, vertex_data.as_ptr() as _, gl::STATIC_DRAW);
                gl::EnableVertexAttribArray(0);
                gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 4 * size_of::<f32>() as i32, (0 * size_of::<f32>()) as *const c_void);
                gl::EnableVertexAttribArray(1);
                gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 4 * size_of::<f32>() as i32, (2 * size_of::<f32>()) as *const c_void);
            }
        }
    }

    impl ISprite<Shader, Texture> for Sprite {
        fn draw(&self, shader: &Shader) {
            shader.bind();
            if let Some(tex) = &self.tex {
                let slot = 0;
                tex.bind(slot);
                shader.set_parameter("uTexture", &(slot as i32));
            }

            let sprite_mat = Mat3::translate(self.pos) * Mat3::rotate(self.rotation) * Mat3::scale(self.scale);
            shader.set_parameter("uSprite", &sprite_mat);

            gl::verify! { 
                gl::BindVertexArray(*self.vao);
                gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
            }
        }

        fn set_texture(&mut self, texture: Texture) {
            self.tex = Some(texture);
        }

        fn set_position(&mut self, pos: Vec2) {
            self.pos = pos;
        }

        fn position(&self) -> Vec2 {
            self.pos
        }

        fn set_scale(&mut self, scale: Vec2) {
            self.scale = scale;
        }

        fn scale(&self) -> Vec2 {
            self.scale
        }

        fn set_rotation(&mut self, rotation: f32) {
            self.rotation = rotation;
        }

        fn rotation(&self) -> f32 {
            self.rotation
        }
    }

}

