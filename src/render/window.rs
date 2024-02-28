use std::time::Duration;

use glfw::Context;

use crate::time::Ticker;

use super::init;

pub struct DrawContext(());

pub struct WindowManager {
    window: glfw::PWindow,
    event_pump: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    glfw: glfw::Glfw,
}

impl WindowManager {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        // window hints go here
        let (mut window, event_pump) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .unwrap();
        window.set_key_polling(true);

        Self {
            window,
            event_pump,
            glfw,
        }
    }

    pub fn make_draw_context(&mut self) -> DrawContext {
        self.window.make_current();
        init(|procstr| self.window.get_proc_address(procstr));

        DrawContext(())
    }

    pub fn show(&mut self) {
        self.window.show();
    }
}

pub trait GameLoop<'c> {
    fn setup(ctx: &'c DrawContext, wm: &mut WindowManager) -> Self;
    fn tick(&mut self, dt: Duration, wm: &mut WindowManager);
    fn draw(&mut self, ctx: &'c DrawContext, wm: &mut WindowManager);
}

pub struct Engine {
    ctx: DrawContext,
    window_manager: WindowManager,
}

impl Engine {
    pub fn new(mut window_manager: WindowManager) -> Self {
        let ctx = window_manager.make_draw_context();

        Self {
            ctx,
            window_manager,
        }
    }

    pub fn run<'g, G: GameLoop<'g>>(&'g mut self) {
        let mut game_loop = G::setup(&self.ctx, &mut self.window_manager);
        self.window_manager.show();

        let mut delta_time = Ticker::new();
        while !self.window_manager.window.should_close() {
            self.window_manager.glfw.poll_events();
            for (_, e) in glfw::flush_messages(&self.window_manager.event_pump) {
                if let glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) = e {
                    self.window_manager.window.set_should_close(true);
                }
            }

            let dt = delta_time.tick();
            game_loop.tick(dt, &mut self.window_manager);
            game_loop.draw(&self.ctx, &mut self.window_manager);

            self.window_manager.window.swap_buffers();
        }
    }
}
