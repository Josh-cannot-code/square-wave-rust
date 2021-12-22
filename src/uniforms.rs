pub fn gen_rot_matrix(t: f32, angle: f32) -> [[f32; 4]; 4] {
    let matrix = [
        [t.cos(), 0.0, t.sin(), 0.0],
        [angle.sin() * t.sin(), angle.cos(), -angle.sin() * t.cos(), 0.0],
        [angle.cos() * -t.sin(), angle.sin(), t.cos() * angle.cos(), 0.0],
        [0.0, 0.0, 1.5, 1.0f32]
    ];
    return matrix;
}

pub fn gen_perspective(dimensions: (u32, u32)) -> [[f32; 4]; 4] {
    let perspective = {
        let (width, height) = dimensions;
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = 3.141592 / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    };
    return perspective;
}