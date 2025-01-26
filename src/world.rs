use std::{io::Write, rc::Rc};

use image::{Rgb, RgbImage};
use nalgebra::Vector3;

const MAX_STEPS: u32 = 256;
const STEP_SIZE: f32 = 0.05;

use crate::{
    camera::{CameraOrtho, Visible},
    masses::StaticMass,
    photon::Physics,
};

pub struct World {
    camera: CameraOrtho,
    objects: Vec<Rc<dyn Visible>>,
    masses: Rc<Vec<StaticMass>>,
}

pub struct Sphere {
    pub pos: Vector3<f32>,
    pub rad: f32,
    pub col: [f32; 3],
}

impl Visible for Sphere {
    #[inline]
    fn overlap(&self, point: &Vector3<f32>) -> Option<[f32; 3]> {
        if point.metric_distance(&self.pos) <= self.rad {
            Some(self.col)
        } else {
            None
        }
    }
}

pub struct TestBlobs {
    pub pos: Vector3<f32>,
    pub scale: f32,
    pub size: f32,
    pub col: [f32; 3],
}

impl Visible for TestBlobs {
    #[inline]
    fn overlap(&self, point: &Vector3<f32>) -> Option<[f32; 3]> {
        let local = (point - self.pos) / self.scale;
        if local.magnitude_squared()
            + (4.0 * local.x).sin()
            + (4.0 * local.y).sin()
            + (4.0 * local.z).sin()
            <= self.size
        {
            Some(self.col)
        } else {
            None
        }
    }
}

impl World {
    pub fn new(
        cam_pos: Vector3<f32>,
        subject: Vector3<f32>,
        width: f32,
        height: f32,
        res_width: u32,
        res_height: u32,
        masses: Rc<Vec<StaticMass>>,
    ) -> World {
        World {
            camera: CameraOrtho::new(cam_pos, subject, width, height, res_width, res_height),
            objects: vec![],
            masses: masses,
        }
    }

    pub fn add_object(&mut self, object: Rc<dyn Visible>) {
        self.objects.push(object);
    }

    pub fn render(&self, filename: &str) {
        let mut image = RgbImage::new(self.camera.screen.res_width, self.camera.screen.res_height);
        let mut prog: u64 = 0;
        let base = 0.5f32.powf(2.0 / (MAX_STEPS as f32));

        for (x_px, y_px) in self.camera.screen {
            if x_px == 0 {
                print!(
                    "\r{:.1}%   ",
                    100.0 * (prog as f64)
                        / (self.camera.screen.res_width * self.camera.screen.res_height) as f64
                );
                std::io::stdout().flush().unwrap();
            }
            prog += 1;
            let mut photon = self.camera.pixel_to_photon(x_px, y_px);

            let mut intersect: Option<(u32, [f32; 3])> = None;
            for step in 0..MAX_STEPS {
                photon.step(self.masses.clone(), STEP_SIZE);
                for object in self.objects.iter() {
                    if let Some(diffuse) = object.overlap(photon.pos()) {
                        intersect = Some((step, diffuse));
                        break;
                    }
                }
                if intersect.is_some() {
                    break;
                }
            }

            let col = if let Some((steps, diffuse)) = intersect {
                let grey = base.powf(steps as f32).min(1.0);
                Rgb([
                    (grey * diffuse[0] * 255.0) as u8,
                    (grey * diffuse[1] * 255.0) as u8,
                    (grey * diffuse[2] * 255.0) as u8,
                ])
            } else {
                Rgb([0, 0, 0])
            };
            image.put_pixel(x_px, y_px, col);
        }

        image.save(filename).unwrap();
    }
}
