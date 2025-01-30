use std::{fs, path::PathBuf, sync::Arc};

use glam::{Vec3, Vec3Swizzles};
use image::{GenericImage, ImageBuffer, ImageReader};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    camera::{Ortho, Visible},
    masses::StaticMass,
    photon::{Photon, Physics},
};

pub struct World {
    cameras: Vec<Ortho>,
    objects: Vec<Arc<dyn Visible>>,
    masses: Vec<StaticMass>,
}

pub struct Sphere {
    pub pos: Vec3,
    pub rad: f32,
    pub col: [f32; 3],
}

impl Visible for Sphere {
    #[inline]
    fn overlap(&self, point: &Vec3) -> Option<[f32; 3]> {
        if point.distance_squared(self.pos) <= self.rad.powi(2) {
            Some(self.col)
        } else {
            None
        }
    }
}

pub struct TestBlobs {
    pub pos: Vec3,
    pub scale: f32,
    pub size: f32,
    pub col: [f32; 3],
}

impl Visible for TestBlobs {
    #[inline]
    fn overlap(&self, point: &Vec3) -> Option<[f32; 3]> {
        let local = (point - self.pos) / self.scale;
        if local.length_squared()
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
    pub pos: Vec3,
    pub outer_rad: f32,
    pub inner_rad: f32,
    pub height: f32,
    pub col: [f32; 3],
}

impl Visible for Disk {
    fn overlap(&self, point: &Vec3) -> Option<[f32; 3]> {
        let local = point - self.pos;
        if local.z.abs() < self.height / 2.0 {
            let r_sqr = local.xy().length_squared();
            if r_sqr >= self.inner_rad * self.inner_rad && r_sqr <= self.outer_rad * self.outer_rad
            {
                let theta = (local.y).atan2(local.x);
                let discs_1_phase = 2.0 * r_sqr.powf(0.5) / self.inner_rad;
                let discs_2_phase = 3.24 * r_sqr.powf(0.7) / self.inner_rad + theta;
                let discs_3_phase = 1.7 * r_sqr.powf(0.5) / self.inner_rad + 2.0 * theta;
                let discs_4_phase = r_sqr / self.inner_rad + theta;
                let discs_5_phase = 0.2 * r_sqr / self.inner_rad + 2.0 * theta;
                let grey = (2.5
                    + (discs_1_phase.sin().abs() + 0.1 * discs_2_phase.sin()
                        - 0.2 * discs_3_phase.sin()
                        + 0.2 * discs_4_phase.sin()
                        + 0.5 * discs_5_phase.sin()))
                    * (1.0 - (r_sqr / self.outer_rad.powi(2)))
                    * (5.0 * (r_sqr - self.inner_rad) / self.inner_rad).clamp(0.0, 1.0);
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

    pub fn add_camera(&mut self, camera: Ortho) {
        self.cameras.push(camera);
    }

    pub fn clear_cameras(&mut self) {
        self.cameras.clear();
    }

    pub fn split_camera(&mut self, index: usize, n: usize) {
        assert!(
            index < self.cameras.len(),
            "Tried to split non-existant camera!"
        );

        let camera = self.cameras.remove(index);

        let mut last_row_end = 0;
        let step = camera.screen.res_height / u32::try_from(n).unwrap();
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

    pub fn add_mass(&mut self, pos: Vec3, mass: f32) {
        self.masses.push(StaticMass { pos, mass });
    }

    pub fn clear_masses(&mut self) {
        self.masses.clear();
    }

    pub(crate) fn simulate_photon(
        &self,
        mut photon: Photon,
        max_steps: u32,
        step_size: f32,
    ) -> Option<[f32; 3]> {
        for _ in 0..max_steps {
            photon.step(&self.masses, step_size);
            for object in &self.objects {
                if let Some(diffuse) = object.overlap(&photon.pos) {
                    return Some(diffuse);
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
        World::stitch_pngs(res_width, res_height, filename);
    }

    fn par_render_split_pngs(&self) {
        let _ = fs::create_dir("stitch");
        self.cameras.par_iter().for_each(|camera| {
            if let Some(i) = camera.split_index {
                camera.render_png(&format!("stitch/stitch_{i}.png"), self);
            }
        });
    }

    fn stitch_pngs(width: u32, height: u32, filename: &str) {
        let mut paths: Vec<PathBuf> = fs::read_dir("stitch")
            .unwrap()
            .map(|p| p.unwrap().path())
            .collect();

        paths.sort_by_key(|a| {
            a.file_stem()
                .unwrap()
                .to_string_lossy()
                .chars()
                .filter(char::is_ascii_digit)
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
