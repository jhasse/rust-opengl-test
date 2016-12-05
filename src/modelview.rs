use nalgebra::*;

#[derive(Clone)]
pub struct Modelview {
    pub matrix: Matrix4<f32>,
}

impl Modelview {
    pub fn new() -> Modelview {
        Modelview{ matrix: new_identity(4) }
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.matrix = self.matrix * Matrix4::new(1f32, 0f32, 0f32, x,
                                                 0f32, 1f32, 0f32, y,
                                                 0f32, 0f32, 1f32, 0f32,
                                                 0f32, 0f32, 0f32, 1f32);
    }

    pub fn reset(&mut self) {
        self.matrix = new_identity(4);
    }
}
