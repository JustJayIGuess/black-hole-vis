use std::{fs, path::PathBuf, sync::Arc};

use nalgebra::Vector3;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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
                let discs_1_phase = 2.0 * r.powf(0.5) / self.inner_rad;
                let discs_2_phase = 3.1416 * r.powf(0.7) / self.inner_rad;
                let discs_3_phase = 1.7 * r.powf(0.5) / self.inner_rad;
                let grey = (2.5
                    + (discs_1_phase.sin().abs() + 0.1 * discs_2_phase.sin()
                        - 0.2 * discs_3_phase.sin()))
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

    pub fn add_mass(&mut self, pos: Vector3<f32>, mass: f32) {
        self.masses.push(StaticMass { pos, mass });
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

    pub fn par_render_split_pngs(&self) {
        let _ = fs::create_dir("stitch");
        self.cameras.par_iter().for_each(|camera| {
            if let Some(i) = camera.split_index {
                camera.render_png(&format!("stitch/stitch_{}.png", i), self);
            }
        });
    }

    pub fn render_pngs(&self, filename: &str) {
        for (i, camera) in self.cameras.iter().enumerate() {
            camera.render_png(&format!("{}_{}.png", filename, i), self);
        }
    }

    pub fn stitch_pngs(&self, n: usize, filename: &str) {
        use stitchy_core::{ImageFiles, OrderBy, TakeFrom};

        let image_files = ImageFiles::builder()
            .add_directory(fs::canonicalize(PathBuf::from("stitch")).unwrap())
            .unwrap()
            .build()
            .unwrap()
            .sort_and_truncate_by(n, OrderBy::Alphabetic, TakeFrom::Start, false)
            .unwrap();

        // Stitch images in a horizontal line, restricting the width to 1000 pixels
        use stitchy_core::{AlignmentMode, Stitch};
        let stitch = Stitch::builder()
            .image_files(image_files)
            .unwrap()
            .width_limit(10000)
            .alignment(AlignmentMode::Vertical)
            .stitch();

        stitch.unwrap().save(format!("{}.png", filename)).unwrap();
    }
}
