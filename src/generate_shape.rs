extern crate glium;
extern crate nalgebra as na;

use nalgebra::DMatrix;

// Standard glium vertex
#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

// Take a matrix that represents a spline and convert into a vector of verticies
fn matrix_to_vertices(mat : na::DMatrix<f32>) -> Vec<Vertex> {
    let mut vertices = [Vertex{ position: [0.0, 0.0, 0.0] }; 8];
    for i in 0..7 {
        vertices[i] = Vertex{position:[mat[(0,i)], mat[(1,i)], mat[(2,i)]]};
    }
    return vertices.to_vec();
}

fn generate_splines(mut shape: &mut Vec<Vertex>, prev_spline : &DMatrix<f32>, number_of_splines : u32) -> Vec<Vertex> {
    // if number of splines reached, return shape
    if number_of_splines > shape.len() as u32 {
        let scale: f32 = shape.len() as f32 / (8*number_of_splines) as f32;
        let delta: f32 = 1.0 / number_of_splines as f32;
        let spline = prev_spline * scale;
        // translate
        let x_0 = &prev_spline[(0,2)];
        let translation_x = f32::sqrt((*x_0 * *x_0) + (delta * delta));
        spline.add_scalar(translation_x);
        let mut spline_vec = matrix_to_vertices(spline);
        shape.append(&mut spline_vec);
        generate_splines(shape, &spline, number_of_splines);
    }
    return shape.to_vec();
}

pub fn gen_shape(number_of_splines : u32) -> Vec<Vertex> {
    let mut shape = Vec::new();
    let mat = na::dmatrix![0.0, 1.0, 1.0,2.0,2.0,1.0,1.0,0.0;
                          0.0,-1.0,-1.0,0.0,0.0,1.0,1.0,0.0;
                          0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];
    mat = mat * 0.5;
    return generate_splines(&mut shape, &mat, number_of_splines);
}

pub fn test_shape() -> Vec<Vertex> {
    let mat = na::dmatrix![0.0, 1.0, 1.0,2.0,2.0,1.0,1.0,0.0;
                          0.0,-1.0,-1.0,0.0,0.0,1.0,1.0,0.0;
                          0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];
    let shape = matrix_to_vertices(mat * 0.5);
    return shape;
}