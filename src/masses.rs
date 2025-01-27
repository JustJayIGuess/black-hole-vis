use nalgebra::Vector3;

#[derive(Copy, Clone)]
pub struct StaticMass {
    pub pos: Vector3<f32>,
    pub mass: f32,
}
