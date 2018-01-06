extern crate gl;
extern crate glutin;

use glutin::GlContext;
use glutin::WindowEvent;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("")
        .with_transparency(true);
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    let _ = unsafe { gl_window.make_current() };

    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    events_loop.run_forever(|event| {
        println!("{:?}", event);
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                WindowEvent::Closed |
                WindowEvent::KeyboardInput { input: glutin::KeyboardInput {
                    state: glutin::ElementState::Pressed,
                    virtual_keycode: Some(glutin::VirtualKeyCode::Q),
                    modifiers: glutin::ModifiersState { logo: true, .. },
                    ..
                }, ..} |
                WindowEvent::KeyboardInput { input: glutin::KeyboardInput {
                    state: glutin::ElementState::Pressed,
                    virtual_keycode: Some(glutin::VirtualKeyCode::W),
                    modifiers: glutin::ModifiersState { logo: true, .. },
                    ..
                }, ..} => {
                    return glutin::ControlFlow::Break
                },
                WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                _ => (),
            },
            _ => ()
        }

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        let _ = gl_window.swap_buffers();
        glutin::ControlFlow::Continue
    });
}
