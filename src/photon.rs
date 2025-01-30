use crate::masses::StaticMass;
use glam::Vec3;

const G_CONST: f32 = 10.0;

pub struct Photon {
    pub pos: Vec3,
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
    fn step(&mut self, environment: &Vec<StaticMass>, step_size: f32);
}

impl Physics for Photon {
    #[inline(always)]
    fn step(&mut self, environment: &Vec<StaticMass>, step_size: f32) {
        // Newtonian
        for mass in environment {
            let r_sqr = mass.pos.distance_squared(self.pos);
            self.dir = self.dir
                + step_size * step_size * G_CONST * mass.mass * (mass.pos - self.pos)
                    / (r_sqr * r_sqr.sqrt()); // a = GM/r^2
        }

        self.dir = self.dir.normalize();
        self.pos = self.pos + self.dir * step_size;
    }
}
