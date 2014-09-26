use cgmath::Matrix4;

pub struct Modelview {
    matrix: Matrix4<f64>
}

impl Modelview {
    pub fn new() -> Modelview {
        Modelview{ matrix: Matrix4::identity() }
    }
}
