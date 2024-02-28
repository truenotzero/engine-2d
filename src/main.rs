use std::path::Path;

use engine_2d::math::Mat3;
use engine_2d::render;
use engine_2d::render::shader::IShaderBuilder;
use engine_2d::render::shader::Shader;
use engine_2d::render::shader::ShaderBuilder;
use engine_2d::render::shader::ShaderPart;
use engine_2d::render::sprite::ISprite;
use engine_2d::render::sprite::Sprite;
use engine_2d::render::texture::ITexture;
use engine_2d::render::texture::Texture;
use engine_2d::render::window::Engine;
use engine_2d::render::window::GameLoop;
use engine_2d::render::window::WindowManager;
use glfw::Context;
use glfw::Key;

// fn wnd_setup() {
//     let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
//     /*
//     glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
//     glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
//     lfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 5);
//     */
//     glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
//     glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
//     glfw.window_hint(glfw::WindowHint::OpenGlProfile(
//         glfw::OpenGlProfileHint::Core,
//     ));
//     let (mut wnd, evt) = glfw
//         .create_window(1200, 1200, "TITLE", glfw::WindowMode::Windowed)
//         .expect("Can't make window!");
//
//     wnd.set_key_polling(true);
//     wnd.make_current();
//
//     // load opengl functions
//     render::init(|s| wnd.get_proc_address(s) as _);
//
//     let mut renderer = Renderer::new();
//
//     while !wnd.should_close() {
//         glfw.poll_events();
//
//         for (_, e) in glfw::flush_messages(&evt) {
//             match e {
//                 glfw::WindowEvent::Key(Key::Escape, _, glfw::Action::Press, _) => {
//                     wnd.set_should_close(true)
//                 }
//                 _ => (),
//             }
//         }
//
//         renderer.render();
//
//         wnd.swap_buffers();
//     }
// }
//
// struct Renderer {
//     shader: Shader,
//     sprite: Sprite,
// }
//
// impl Renderer {
//     fn new() -> Self {
//         let vert_src = r#"
//         #version 450 core
//
//         uniform mat3 uSprite;
//         uniform mat3 uView; // no camera support yet
//
//         layout(location = 0)
//         in vec2 aPos;
//
//         layout(location = 1)
//         in vec2 aUV;
//
//         out vec2 texUV;
//
//         void main() {
//             texUV = aUV;
//
//             gl_Position = vec4(uView * uSprite * vec3(aPos, 1.0), 1.0);
//             gl_Position = vec4(uSprite * vec3(aPos, 1.0), 1.0);
//         }
//         "#;
//
//         let frag_src = r#"
//         #version 450 core
//
//         uniform sampler2D uTexture;
//
//         in vec2 texUV;
//
//         out vec4 FragColor;
//
//         void main() {
//             FragColor = texture(uTexture, texUV);
//         }
//         "#;
//
//         let shader = ShaderBuilder::default()
//             // .add_part(shader::PartType::Vertex, vert_src)
//             // .unwrap()
//             // .add_part(shader::PartType::Fragment, frag_src)
//             // .unwrap()
//             .verify()
//             .unwrap();
//
//         let mut sprite = Sprite::default();
//         // sprite.set_scale((0.1, 0.1).into());
//         // sprite.set_rotation(90.0);
//         // sprite.set_position((0.5, 0.5).into());
//         sprite.set_texture(Texture::from_file(Path::new("deer.png")).unwrap());
//
//         Self { shader, sprite }
//     }
//
//     fn render(&mut self) {
//         render::clear();
//         self.sprite.draw(&self.shader, Mat3::identity());
//     }
// }

struct GameLoopImpl<'a> {
    sprite: Sprite<'a>,
    shader: Shader<'a>,
}

impl<'a> GameLoop<'a> for GameLoopImpl<'a> {
    fn setup(ctx: &'a render::window::DrawContext, _wm: &mut WindowManager) -> Self {
        let vert_src = r#"
         #version 450 core

         uniform mat3 uSprite;
         uniform mat3 uView; // no camera support yet

         layout(location = 0)
         in vec2 aPos;

         layout(location = 1)
         in vec2 aUV;

         out vec2 texUV;

         void main() {
             texUV = aUV;

             gl_Position = vec4(uView * uSprite * vec3(aPos, 1.0), 1.0);
             gl_Position = vec4(uSprite * vec3(aPos, 1.0), 1.0);
         }
         "#;

        let frag_src = r#"
         #version 450 core

         uniform sampler2D uTexture;

         in vec2 texUV;

         out vec4 FragColor;

         void main() {
             FragColor = texture(uTexture, texUV);
         }
         "#;

        let shader = ShaderBuilder::new(ctx)
            .add_part(ShaderPart {
                type_: render::shader::PartType::Vertex,
                source_code: vert_src,
            })
            .unwrap()
            .add_part(ShaderPart {
                type_: render::shader::PartType::Fragment,
                source_code: frag_src,
            })
            .unwrap()
            .verify()
            .unwrap();
        let texture = Texture::from_file(ctx, Path::new("deer.png")).unwrap();
        let sprite = Sprite::new(ctx, texture);

        Self { shader, sprite }
    }

    fn tick(&mut self, dt: std::time::Duration, wm: &mut WindowManager) {}

    fn draw(&mut self, ctx: &render::window::DrawContext, wm: &mut WindowManager) {
        self.sprite.draw(&self.shader, Mat3::identity());
    }
}

fn main() {
    let wm = WindowManager::new(800, 600, "new engine!");
    let mut engine = Engine::new(wm);
    engine.run::<GameLoopImpl>()
}
