mod generate_shape;
mod shaders;
mod uniforms;

#[macro_use]
extern crate glium;

fn main() {
    // ------ User Defined Parameters ------
    let number_of_splines = 40; // Changes the number of splines
    let spline_angle = 1.0; // Multiple of 2*pi
    // ex: spline_angle = 3.0; splines will make 3 full rotations
    let rotation_speed = 1.0; // increase for faster rotation, decrease for slower


    use glium::{glutin, Surface};
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Get shape from generate shape
    let shape = generate_shape::gen_shape(number_of_splines,
                                          spline_angle * 2.0 * (std::f64::consts::PI as f32));
    // Vertex buffer to use gpu for rendering:
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    // Dummy index list
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
    // Initialize shaders
    let vertex_shader_src = shaders::vertex_shader();
    let fragment_shader_src = shaders::pixle_shader();

    // Program to send to draw function
    let program = glium::Program::from_source(&display, vertex_shader_src,
                                              fragment_shader_src, None).unwrap();
    let mut t: f32 = 0.0;

    event_loop.run(move |ev, _, control_flow| {
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // Drawing the shape
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program,
                    &uniform!{ matrix : uniforms::gen_rot_matrix(t,(std::f64::consts::PI as f32) / 2.0 ),
                        perspective : uniforms::gen_perspective(target.get_dimensions())},
                    &Default::default()).unwrap();
        target.finish().unwrap();
        // Increment t
        t += rotation_speed * 0.003;
    });
}
