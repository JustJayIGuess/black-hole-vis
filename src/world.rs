use std::rc::Rc;

use image::{Rgb, RgbImage};
use nalgebra::Vector3;

const MAX_STEPS: u32 = 200;
const BASE: f32 = 0.99;
const STEP_SIZE: f32 = 0.03;

use crate::{
    camera::{CameraOrtho, Visible},
    masses::StaticMasses,
    photon::{Photon, Physics},
};

pub struct World {
    camera: CameraOrtho,
    objects: Vec<Rc<dyn Visible>>,
    masses: Rc<StaticMasses>,
}

pub struct Sphere {
    pub pos: Vector3<f32>,
    pub rad: f32,
}

impl Visible for Sphere {
    fn overlap(&self, point: &Vector3<f32>) -> bool {
        point.metric_distance(&self.pos) <= self.rad
    }
}

pub struct TestBlobs {
    pub pos: Vector3<f32>,
    pub scale: f32,
    pub size: f32,
}

impl Visible for TestBlobs {
    fn overlap(&self, point: &Vector3<f32>) -> bool {
        let local = (point - self.pos) / self.scale;
        local.magnitude_squared() + (4.0 * local.x).sin() + (4.0 * local.y).sin() + (4.0 * local.z).sin() <= self.size
    }
}

impl World {
    pub fn new(
        cam_pos: Vector3<f32>,
        subject: Vector3<f32>,
        near_clip: f32,
        width: f32,
        height: f32,
        res_width: u32,
        res_height: u32,
        masses: Rc<StaticMasses>,
    ) -> World {
        World {
            camera: CameraOrtho::new(
                cam_pos, subject, near_clip, width, height, res_width, res_height,
            ),
            objects: vec![],
            masses: masses,
        }
    }

    pub fn add_object(&mut self, object: Rc<dyn Visible>) {
        self.objects.push(object);
    }

    pub fn render(&self) {
        let mut image = RgbImage::new(
            self.camera.screen.res_width,
            self.camera.screen.res_height
        );
        for (x_px, y_px) in self.camera.screen {
            let mut photon = self.camera.pixel_to_photon(x_px, y_px);

            let mut intersect: Option<u32> = None;
            for step in 0..MAX_STEPS {
                photon.step(self.masses.clone(), STEP_SIZE);
                for object in self.objects.iter() {
                    if object.overlap(photon.pos()) {
                        intersect = Some(step);
                        break;
                    }
                }
                if intersect.is_some() {
                    break;
                }
            }

            let col = if let Some(steps) = intersect {
                let grey_float = (BASE.powf(steps as f32) * 255.0).min(255.0);
                let grey = grey_float as u8;
                Rgb([grey, grey, grey])
            } else {
                Rgb([0, 0, 0])
            };
            image.put_pixel(x_px, y_px, col);
        }

        image.save("out.png").unwrap();
    }
}
