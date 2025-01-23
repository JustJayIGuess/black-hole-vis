use nalgebra::Vector3;

use crate::StaticMasses;

pub struct Photon {
    pos: Vector3<f32>,
    dir: Vector3<f32>,
    wavelength: f32,
}

impl Photon {
    pub fn new(pos: Vector3<f32>, dir: Vector3<f32>, wavelength: f32) -> Photon {
        Photon {
            pos,
            dir,
            wavelength,
        }
    }
}

pub trait Physics {
    fn pos(&self) -> &Vector3<f32>;
    fn step(&mut self, environment: &StaticMasses) -> Option<()>;
}

impl Physics for Photon {
    fn pos(&self) -> &Vector3<f32> {
        &self.pos
    }

    /// Will return `Some(...)` if photon collided
    fn step(&mut self, environment: &StaticMasses) -> Option<()> {
        Some(())
    }
}
