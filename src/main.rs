#[macro_use]
extern crate glium;

fn main() {
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Vertex, to be moved
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
    }

    // Look into this a little more
    implement_vertex!(Vertex, position);

    // Basic Triangle: TODO: Move triangle
    let vertex1 = Vertex{ position: [-0.5, -0.5, 0.0] };
    let vertex2 = Vertex{ position: [0.0, 0.5, 0.0] };
    let vertex3 = Vertex{ position: [0.5, -0.25, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    // Vertex buffer to use gpu for rendering:
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    // Dummy index list
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    //GLSL shader TODO: Move shaders
    //Vertex Shader
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;

        void main() {
            gl_Position = vec4(position, 1.0);
        }
    "#;
    // Pixle shader
    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
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
