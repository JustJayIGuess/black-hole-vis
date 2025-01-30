use crate::masses::StaticMass;
use glam::Vec3;

const G_CONST: f32 = 10.0;

pub struct Photon {
    pos: Vec3,
    dir: Vec3,
}

impl Photon {
    pub fn new(pos: Vec3, dir: Vec3) -> Photon {
        Photon {
            pos,
            dir: dir.normalize(),
        }
    }
}

pub trait Physics {
    fn pos(&self) -> &Vec3;
    fn step(&mut self, environment: Vec<StaticMass>, step_size: f32);
}

impl Physics for Photon {
    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn step(&mut self, environment: Vec<StaticMass>, step_size: f32) {
        // Newtonian
        for mass in environment.iter() {
            let acc: f32 = G_CONST * mass.mass / mass.pos.distance_squared(self.pos);
            let dir: Vec3 = (mass.pos - self.pos).normalize();
            self.dir = self.dir + acc * dir * step_size * step_size; // a = GM/r^2
        }

        self.dir = self.dir.normalize_or_zero();
        self.pos = self.pos + self.dir * step_size;
    }
}
