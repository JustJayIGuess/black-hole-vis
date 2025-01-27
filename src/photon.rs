use crate::masses::StaticMass;
use nalgebra::Vector3;

const G_CONST: f32 = 10.0;

pub struct Photon {
    pos: Vector3<f32>,
    dir: Vector3<f32>,
}

impl Photon {
    pub fn new(pos: Vector3<f32>, dir: Vector3<f32>) -> Photon {
        Photon {
            pos,
            dir: dir.normalize(),
        }
    }
}

pub trait Physics {
    fn pos(&self) -> &Vector3<f32>;
    fn step(&mut self, environment: Vec<StaticMass>, step_size: f32);
}

impl Physics for Photon {
    fn pos(&self) -> &Vector3<f32> {
        &self.pos
    }

    fn step(&mut self, environment: Vec<StaticMass>, step_size: f32) {
        // Newtonian
        for mass in environment.iter() {
            let acc: f32 = G_CONST * mass.mass / (mass.pos.metric_distance(&self.pos)).powi(2);
            let dir: Vector3<f32> = (mass.pos - self.pos).normalize();
            self.dir = self.dir + acc * dir * step_size * step_size; // a = GM/r^2
        }

        self.dir.normalize_mut();
        self.pos = self.pos + self.dir * step_size;
    }
}
