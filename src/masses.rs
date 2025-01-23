use nalgebra::Vector3;

pub struct StaticMass {
    pub pos: Vector3<f32>,
    pub mass: f32,
}

pub struct StaticMasses {
    masses: Vec<StaticMass>,
}

impl StaticMasses {
    pub fn new() -> StaticMasses {
        StaticMasses { masses: vec![] }
    }

    pub fn add(&mut self, pos: Vector3<f32>, mass: f32) {
        self.masses.push(StaticMass { pos, mass });
    }
}
