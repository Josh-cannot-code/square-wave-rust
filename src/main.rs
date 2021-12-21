mod generate_shape;
mod shaders;

#[macro_use]
extern crate glium;

fn main() {
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Get shape from generate shape
    let shape = generate_shape::gen_shape(20);
    //let shape = generate_shape::test_shape();

    // Vertex buffer to use gpu for rendering:
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    // Dummy index list
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
    // Initialize shaders
    let vertex_shader_src = shaders::vertex_shader();
    let fragment_shader_src = shaders::pixle_shader();
    // Program to send to draw function
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    event_loop.run(move |ev, _, control_flow| {

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        // Draw triangle
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}
