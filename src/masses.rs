use glam::Vec3;

#[derive(Copy, Clone)]
pub struct StaticMass {
    pub pos: Vec3,
    pub mass: f32,
}
