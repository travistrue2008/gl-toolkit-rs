mod state;
mod sprite_state;

use crate::state::{State, FiniteStateMachine};
use crate::sprite_state::SpriteState;

use gl_toolkit::{Feature, BlendComponent, ClearFlag};
use lazy_static::lazy_static;
use std::cell::Cell;
use std::sync::mpsc::Receiver;
use std::time::Instant;
use vex::Matrix4;

use glfw::{
	Action,
	Context,
	Key,
	Glfw,
	Window,
	WindowEvent,
	WindowHint,
	WindowMode,
};

lazy_static! {
    static ref proj_mat: Matrix4 = Matrix4::ortho(0.0, 480.0, 0.0, 272.0, 0.0, 1000.0);
}

fn init_glfw() -> Glfw {
    let mut glfw = glfw::init(Some(glfw::Callback {
		f: error_callback,
		data: Cell::new(0),
    })).unwrap();

    glfw.window_hint(WindowHint::ContextVersion(4, 1));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    glfw
}

fn init_window(glfw: &Glfw) -> (Window, Receiver<(f64, WindowEvent)>) {
    let (mut window, events) = glfw
        .create_window(640, 480, "Test", WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    (window, events)
}

fn init_gl(window: &mut Window) {
    let loader = |symbol| window.get_proc_address(symbol) as *const _;

    gl_toolkit::init(loader).unwrap();
    gl_toolkit::enable(Feature::Blend);
    gl_toolkit::clear_color(0.2, 0.3, 0.3, 1.0);
    gl_toolkit::blend_func(BlendComponent::OneMinusSrcAlpha, BlendComponent::SrcAlpha);
}

fn error_callback(_: glfw::Error, description: String, error_count: &Cell<usize>) {
	println!("GLFW error ({}): {}", error_count.get(), description);
	error_count.set(error_count.get() + 1);
}

fn process_events(window: &mut Window, events: &Receiver<(f64, WindowEvent)>) {
    for (_, event) in glfw::flush_messages(&events) {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            WindowEvent::FramebufferSize(width, height) =>
                resize_frame(width as u32, height as u32),
            _ => {}
        }
    }
}

fn resize_frame(width: u32, height: u32) {
    gl_toolkit::set_viewport(0, 0, width, height);
}

fn main() {
    let mut glfw = init_glfw();
    let (mut window, events) = init_window(&glfw);

    init_gl(&mut window);

    let start_time = Instant::now();
    let mut fsm = FiniteStateMachine::new();
    fsm.push(SpriteState::new());

    while !window.should_close() {
        let elapsed_time = start_time.elapsed().as_secs_f32();

        process_events(&mut window, &events);
        fsm.update(elapsed_time);
        fsm.render();

        window.swap_buffers();
        glfw.poll_events();
    }
}
