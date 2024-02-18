use std::path::Path;

use engine_2d::render;
use engine_2d::render::shader;
use engine_2d::render::shader::IShaderBuilder;
use engine_2d::render::shader::Shader;
use engine_2d::render::shader::ShaderBuilder;
use engine_2d::render::sprite::ISprite;
use engine_2d::render::sprite::Sprite;
use engine_2d::render::texture::ITexture;
use engine_2d::render::texture::Texture;
use engine_2d::render::window::Action;
use glfw::Context;
use glfw::Key;

fn wnd_setup() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    /*
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    lfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 5);
    */
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    let (mut wnd, evt) = glfw
        .create_window(1200, 1200, "TITLE", glfw::WindowMode::Windowed)
        .expect("Can't make window!");

    wnd.set_key_polling(true);
    wnd.make_current();

    // load opengl functions
    render::init(|s| wnd.get_proc_address(s) as _);

    let mut renderer = Renderer::new();

    while !wnd.should_close() {
        glfw.poll_events();

        for (_, e) in glfw::flush_messages(&evt) {
            match e {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    wnd.set_should_close(true)
                }
                _ => (),
            }
        }

        renderer.render();

        wnd.swap_buffers();
    }
}

struct Renderer {
    shader: Shader,
    sprite: Sprite,
}

impl Renderer {
    fn new() -> Self {
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

        let shader = ShaderBuilder::default()
            .add_part(shader::Part::Vertex, vert_src)
            .unwrap()
            .add_part(shader::Part::Fragment, frag_src)
            .unwrap()
            .verify()
            .unwrap();

        let mut sprite = Sprite::default();
        sprite.init();
        sprite.set_scale((0.1, 0.1).into());
        sprite.set_rotation(90.0);
        sprite.set_position((0.5, 0.5).into());
        sprite.set_texture(Texture::from_file(Path::new("deer.png")).unwrap());

        Self {
            shader,
            sprite,
        }
    }

    fn render(&mut self) {
        render::clear();
        self.sprite.draw(&self.shader);
    }
}

fn main() {
    wnd_setup();
}
