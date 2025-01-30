use crate::camera::Visible;
use glam::{Vec3, Vec3Swizzles};

pub struct Sphere {
    pub pos: Vec3,
    pub rad: f32,
    pub col: [f32; 3],
}

impl Visible for Sphere {
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
