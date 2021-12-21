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
fn matrix_to_vertices(mat : &na::DMatrix<f32>) -> Vec<Vertex> {
    let mut vertices = [Vertex{ position: [0.0, 0.0, 0.0] }; 8];
    for i in 0..7 {
        vertices[i] = Vertex{position:[mat[(0,i)], mat[(1,i)], mat[(2,i)]]};
    }
    return vertices.to_vec();
}

fn generate_splines(shape: &mut Vec<Vertex>, prev_spline : &DMatrix<f32>, number_of_splines : u32) -> Vec<Vertex> {
    // if number of splines reached, return shape
    if number_of_splines * 8 >= shape.len() as u32 {
        let mut mat = na::dmatrix![0.0, 0.5, 0.5,1.0,1.0,0.5,0.5,0.0;
                          0.0,-0.5,-0.5,0.0,0.0,0.5,0.5,0.0;
                          0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];
        mat = mat * ( (shape.len() as f32) / ((8 * number_of_splines) as f32));
        let mut spline_vec = matrix_to_vertices(&mat);
        shape.append(&mut spline_vec);
        generate_splines(shape, &mat, number_of_splines);
    }
    return shape.to_vec();
}

pub fn gen_shape(number_of_splines : u32) -> Vec<Vertex> {
    let mut mat = na::dmatrix![0.0, 0.5, 0.5,1.0,1.0,0.5,0.5,0.0;
                          0.0,-0.5,-0.5,0.0,0.0,0.5,0.5,0.0;
                          0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];
    let mut shape = Vec::new();
    return generate_splines(&mut shape, &mat, number_of_splines);
}

pub fn test_shape() -> Vec<Vertex> {
    let mat = na::dmatrix![0.0, 1.0, 1.0,2.0,2.0,1.0,1.0,0.0;
                          0.0,-1.0,-1.0,0.0,0.0,1.0,1.0,0.0;
                          0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0];
    let shape = matrix_to_vertices(&(mat * 0.5));
    return shape;
}