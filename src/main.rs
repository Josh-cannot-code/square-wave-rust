mod generate_shape;
mod shaders;
mod uniforms;

#[macro_use]
extern crate glium;

fn main() {
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Get shape from generate shape
    let shape = generate_shape::gen_shape(50, 2.0 * (std::f64::consts::PI as f32));
    // Vertex buffer to use gpu for rendering:
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    // Dummy index list
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
    // Initialize shaders
    let vertex_shader_src = shaders::vertex_shader();
    let fragment_shader_src = shaders::pixle_shader();
    // Program to send to draw function
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let mut t: f32 = 0.0;
    event_loop.run(move |ev, _, control_flow| {
        // Drawing the shape
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program,
                    &uniform!{ matrix : uniforms::gen_rot_matrix(t,(std::f64::consts::PI as f32) / 2.0 ),
                        perspective : uniforms::gen_perspective(target.get_dimensions())},
                    &Default::default()).unwrap();
        target.finish().unwrap();
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        // Increment t
        t += 0.0004;
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
