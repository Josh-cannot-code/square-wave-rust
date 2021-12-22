extern crate glium;
extern crate nalgebra as na;
// TODO: Anti alias
// TODO: Mouse event camera rotation

// Standard glium vertex
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

// Take a matrix that represents a spline and convert into a vector of verticies
fn matrix_to_vertices(mat : &na::DMatrix<f32>) -> Vec<Vertex> {
    let mut vertices = vec![Vertex{ position: [0.0, 0.0, 0.0] }; 8];
    for i in 0..8 {
        vertices[i] = Vertex{position:[mat[(0,i)], mat[(1,i)], mat[(2,i)]]};
    }
    return vertices;
}

pub fn gen_shape(number_of_splines : u32, max_angle : f32) -> Vec<Vertex> {
    // Template Matrix
    let mat = na::dmatrix![0.0, 0.5, 0.5,1.0,1.0,0.5,0.5,0.0;
                          0.0,-0.5,-0.5,0.0,0.0,0.5,0.5,0.0;
                          0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];

    // Inital previous spline
    let mut prev_spline = na::dmatrix![0.0, 0.0, 0.0,0.0,0.0,0.0,0.0,0.0;
                          0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0;
                          0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];
    // Shape vector
    let mut shape = Vec::new();
    // Maximum length of the shape
    let max_len = (8 * number_of_splines) as f32;
    // Distance between splines
    let delta = ((8.0 / max_len) * mat[(0, 4)]) / 2.0;
    // Main generation loop
    while (shape.len() as f32) < max_len {
        let cur_len = shape.len() as f32;
        let cur_iter = cur_len / 8.0;

        // Scale the spline
        let mut cur_spline = &mat * ((cur_len + 8.0) / max_len);
        let angle = max_angle * (-cur_len / max_len);

        // Rotate the spline
        let rot_mat = na::dmatrix![angle.cos(), 0.0, angle.sin();
                                    0.0, 1.0, 0.0;
                                    -angle.sin(), 0.0, -angle.cos()];
        cur_spline = rot_mat * cur_spline;

        // Translate the spline
        let x_0 = - cur_spline[(0,2)];
        let z_0 = - cur_spline[(2,2)];
        let dx = if cur_iter > 0.0 { delta * angle.cos() } else { 0.0 };
        let dz = if cur_iter > 0.0 { delta * angle.sin() } else { 0.0 };
        for mut col in cur_spline.column_iter_mut() {
            col[0] = col[0] + prev_spline[(0,2)] + x_0 + dx;
            col[2] = col[2] + prev_spline[(2,2)] + z_0 - dz;
        }
        if cur_iter > 0.0 {
            cur_spline.column_mut(0).copy_from(&prev_spline.column(1));
            cur_spline.column_mut(7).copy_from(&prev_spline.column(6));
        }
        // Convert to vertices and add to shape
        shape.append(&mut matrix_to_vertices(&cur_spline));
        // Update prev_spline
        prev_spline = cur_spline;
    }
    return shape;
}