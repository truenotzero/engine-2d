use std::ffi::CString;
use std::mem::size_of_val;
use std::ptr::null;

use engine_2d::gl;
use engine_2d::gl_call;
use engine_2d::shader::opengl_46::Shader as gl46Shader;
use engine_2d::shader::Shader;
use engine_2d::shader::Type;
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
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    let (mut wnd, evt) = glfw.create_window(1200, 1200, "TITLE", glfw::WindowMode::Windowed).expect("Can't make window!");

    wnd.set_key_polling(true);
    wnd.make_current();

    // load opengl functions
    gl::load_with(|s| wnd.get_proc_address(s) as *const _);

    let mut time = 0.0;

    while !wnd.should_close() {
        glfw.poll_events();

        for (_, e) in glfw::flush_messages(&evt) {
            match e {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => wnd.set_should_close(true),
                _ => (),
            }
        }


        let wh = wnd.get_framebuffer_size();
        gl_call! { gl::Viewport(0, 0, wh.0, wh.1) };
        render(time);
        time += 0.01;
        wnd.swap_buffers();
    }

}

fn render(t: f32) {
    gl_call! { gl::ClearColor(0.23, 0.64, 0.11, 1.0) };
    gl_call! { gl::Clear(gl::COLOR_BUFFER_BIT) };

    let mut vao = 0;
    gl_call! { gl::GenVertexArrays(1, &mut vao) };
    gl_call! { gl::BindVertexArray(vao) };

    let triangle = [
        0.0f32, 0.5,
        0.5, -0.5,
        -0.5, -0.5,
    ];

    let mut vbo = 0;
    gl_call! { gl::GenBuffers(1, &mut vbo) };
    gl_call! { gl::BindBuffer(gl::ARRAY_BUFFER, vbo) };
    gl_call! { gl::BufferData(gl::ARRAY_BUFFER, size_of_val(&triangle) as _, triangle.as_ptr() as _, gl::STATIC_DRAW) };

    gl_call! { gl::EnableVertexAttribArray(0) };
    gl_call! { gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, null()) };

    // let vert = gl_call! { gl::CreateShader(gl::VERTEX_SHADER) };
    let vert_src = CString::new("#version 450 core
    layout (location = 0)
    in vec4 aPos;
    uniform float time;
    void main() {
        float r = 0.2f;
        vec2 pos = aPos.xy + r * vec2(cos(time), sin(time));
        gl_Position = vec4(pos, aPos.zw);
    }
    ".to_owned()).unwrap();
    // let p_vert_src = vert_src.as_ptr();
    // gl_call! { gl::ShaderSource(vert, 1, &p_vert_src, null()) };
    // gl_call! { gl::CompileShader(vert) };
    // let mut status = 0;
    // gl_call! { gl::GetShaderiv(vert, gl::COMPILE_STATUS, &mut status) };
    // if status != gl::TRUE.into() { panic!("Faield to compile vertex shader!") }

    // let frag  = gl_call! { gl::CreateShader(gl::FRAGMENT_SHADER) };
    let frag_src = CString::new("#version 450 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0f);
    }
    ".to_owned()).unwrap();
    // let p_frag_src = frag_src.as_ptr();
    // gl_call! { gl::ShaderSource(frag, 1, &p_frag_src, null()) };
    // gl_call! { gl::CompileShader(frag) };
    // let mut status = 0;
    // gl_call! { gl::GetShaderiv(frag, gl::COMPILE_STATUS, &mut status) };
    // if status != gl::TRUE.into() { panic!("Faield to compile fragment shader!") }

    // let prog = gl_call! { gl::CreateProgram() };
    // gl_call! { gl::AttachShader(prog, vert) };
    // gl_call! { gl::AttachShader(prog, frag) };
    // gl_call! { gl::LinkProgram(prog) };
    // let mut status = 0;
    // gl_call! { gl::GetProgramiv(prog, gl::LINK_STATUS, &mut status) };
    // if status != gl::TRUE.into() { panic!("Faield to link program!") }
    // gl_call! { gl::DeleteShader(frag) };
    // gl_call! { gl::DeleteShader(vert) };

    let mut prog = gl46Shader::new();
    prog.add(Type::Vertex, vert_src.as_bytes()).unwrap();
    prog.add(Type::Fragment, frag_src.as_bytes()).unwrap();
    prog.verify().unwrap();

    // gl_call! { gl::UseProgram(prog) };
    prog.bind();
    // let time = CString::new("time".to_owned()).unwrap();
    // let time = gl_call! { gl::GetUniformLocation(prog, time.as_ptr()) };
    // gl_call! { gl::Uniform1f(time, t) };
    gl_call! { gl::DrawArrays(gl::TRIANGLES, 0, 3) };

    // gl_call! { gl::DeleteProgram(prog) };
    // gl_call! { gl::DeleteBuffers(1, &mut vbo) };
    // gl_call! { gl::DeleteVertexArrays(1, &mut vao) };

    gl_call! {
        gl::DeleteBuffers(1, &mut vbo);
        gl::DeleteVertexArrays(1, &mut vao);
    };
}

fn main() {
    println!("Hello, world!");

    wnd_setup();
}
