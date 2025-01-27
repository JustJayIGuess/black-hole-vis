use std::{fs, path::PathBuf, sync::Arc};

use image::{GenericImage, ImageBuffer, ImageReader};
use nalgebra::Vector3;
use num::pow::Pow;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const X_AXIS: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);

use crate::{
    camera::{CameraOrtho, Visible},
    masses::StaticMass,
    photon::{Photon, Physics},
};

pub struct World {
    cameras: Vec<CameraOrtho>,
    objects: Vec<Arc<dyn Visible>>,
    masses: Vec<StaticMass>,
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

pub struct Disk {
    pub pos: Vector3<f32>,
    pub outer_rad: f32,
    pub inner_rad: f32,
    pub height: f32,
    pub col: [f32; 3],
}

impl Visible for Disk {
    fn overlap(&self, point: &Vector3<f32>) -> Option<[f32; 3]> {
        let local = point - self.pos;
        let r = local.xy().magnitude_squared();
        if r >= self.inner_rad * self.inner_rad && r <= self.outer_rad * self.outer_rad {
            if local.z.abs() < self.height / 2.0 {
                let theta = (local.y).atan2(local.x);

                let discs_1_phase = 2.0 * r.powf(0.5) / self.inner_rad;
                let discs_2_phase = 3.1416 * r.powf(0.7) / self.inner_rad + theta;
                let discs_3_phase = 1.7 * r.powf(0.5) / self.inner_rad + 2.0 * theta;
                let discs_4_phase = r / self.inner_rad + theta;
                let discs_5_phase = 0.2 * r / self.inner_rad + 2.0 * theta;
                let grey = (2.5
                    + (discs_1_phase.sin().abs() + 0.1 * discs_2_phase.sin()
                        - 0.2 * discs_3_phase.sin()
                        + 0.2 * discs_4_phase.sin()
                        + 0.5 * discs_5_phase.sin()))
                    * (1.0 - (r / self.outer_rad.powi(2)));
                let new_col = [grey * self.col[0], grey * self.col[1], grey * self.col[2]];
                Some(new_col)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl World {
    pub fn new() -> World {
        World {
            cameras: vec![],
            objects: vec![],
            masses: vec![],
        }
    }

    pub fn add_camera(
        &mut self,
        cam_pos: Vector3<f32>,
        subject: Vector3<f32>,
        width: f32,
        height: f32,
        res_width: u32,
        res_height: u32,
    ) {
        self.cameras.push(CameraOrtho::new(
            cam_pos, subject, width, height, res_width, res_height,
        ));
    }

    pub fn clear_cameras(&mut self) {
        self.cameras.clear();
    }

    pub fn split_camera(&mut self, index: usize, n: usize) {
        if index >= self.cameras.len() {
            panic!("Tried to split non-existant camera!");
        }

        let camera = self.cameras.remove(index);

        let mut last_row_end = 0;
        let step = camera.screen.res_height / (n as u32);
        for i in 0..n - 1 {
            self.cameras
                .push(camera.subdivide_camera(last_row_end, last_row_end + step, i));
            last_row_end += step;
        }
        self.cameras
            .push(camera.subdivide_camera(last_row_end, camera.screen.res_height, n - 1));
    }

    pub fn add_object(&mut self, object: Arc<dyn Visible>) {
        self.objects.push(object);
    }

    pub fn clear_objects(&mut self) {
        self.objects.clear();
    }

    pub fn add_mass(&mut self, pos: Vector3<f32>, mass: f32) {
        self.masses.push(StaticMass { pos, mass });
    }

    pub fn clear_masses(&mut self) {
        self.masses.clear();
    }

    pub fn simulate_photon(
        &self,
        mut photon: Photon,
        max_steps: u32,
        step_size: f32,
    ) -> Option<(u32, [f32; 3])> {
        for step in 0..max_steps {
            photon.step(self.masses.clone(), step_size);
            for object in self.objects.iter() {
                if let Some(diffuse) = object.overlap(photon.pos()) {
                    return Some((step, diffuse));
                }
            }
        }
        None
    }

    pub fn split_and_par_render(&mut self, num_threads: usize, filename: &str) {
        let res_width = self.cameras[0].screen.res_width;
        let res_height = self.cameras[0].screen.res_height;
        self.split_camera(0, num_threads);
        self.par_render_split_pngs();
        self.stitch_pngs(res_width, res_height, filename);
    }

    pub fn par_render_split_pngs(&self) {
        let _ = fs::create_dir("stitch");
        self.cameras.par_iter().for_each(|camera| {
            if let Some(i) = camera.split_index {
                camera.render_png(&format!("stitch/stitch_{}.png", i), self);
            }
        });
    }

    pub fn stitch_pngs(&self, width: u32, height: u32, filename: &str) {
        let mut paths: Vec<PathBuf> = fs::read_dir("stitch")
            .unwrap()
            .into_iter()
            .map(|p| p.unwrap().path())
            .collect();

        paths.sort_by_key(|a| {
            a.file_stem()
                .unwrap()
                .to_string_lossy()
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<u32>()
                .unwrap_or(u32::MAX)
        });

        let mut image = ImageBuffer::new(width, height);
        let mut y_px = 0;

        for path in paths {
            let stitch = ImageReader::open(path).unwrap().decode().unwrap();
            let _ = image.copy_from(&stitch.to_rgba8(), 0, y_px);
            y_px += stitch.height();
        }

        let _ = image.save(filename);
    }
}
