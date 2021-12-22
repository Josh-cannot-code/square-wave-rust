//Vertex Shader

pub fn vertex_shader() -> &'static str {
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;
    return vertex_shader_src;
}

// Pixle shader
pub fn pixle_shader() -> &'static str {
    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.0, 0.0, 0.0, 1.0);
        }
    "#;
    return fragment_shader_src;
}