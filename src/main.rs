use engine_2d::math::Vec2;
use engine_2d::render;
use engine_2d::render::shader;
use engine_2d::render::shader::IShader;
use engine_2d::render::shader::IShaderBuilder;
use engine_2d::render::shader::ShaderBuilder;
use engine_2d::vec;
use glfw::Action;
use glfw::Context;
use glfw::Key;

extern crate glfw;

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

        render();

        wnd.swap_buffers();
    }
}

fn render() {
    let vec2 = vec![1.0, 2.0];
    let vec22 = vec![2.0, 3.0];

    let mut vec222: Vec2 = vec2 + vec22;
    println!("{}", vec222);
    vec222 += vec2;
    println!("{}", vec222);
    let vert_src = r#"
    #version 450 core

    layout(location = 0)
    in vec4 aPos;

    void main() {
        gl_Position = aPos;
    }
    "#;

    let frag_src = r#"
    #version 450 core

    out vec4 FragColor;

    void main() {
        FragColor = vec4(1.0f);
    }
    "#;

    let shader = ShaderBuilder::default()
        .add_part(shader::Part::Vertex, vert_src)
        .unwrap()
        .add_part(shader::Part::Fragment, frag_src)
        .unwrap()
        .verify()
        .unwrap();

    shader.draw();
}

fn main() {
    println!("Hello, world!");

    wnd_setup();
}
