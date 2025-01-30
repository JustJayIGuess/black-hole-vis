use std::io::Write;

use glam::Vec3;
use image::{Rgb, RgbImage};

use crate::{photon::Photon, world::World};

const MAX_STEPS: u32 = 400;
const STEP_SIZE: f32 = 0.05;

pub trait Visible: Send + Sync {
    fn overlap(&self, point: &Vec3) -> Option<[f32; 3]>;
}

pub struct CameraOrtho {
    pub pos: Vec3,
    pub subject: Vec3,
    pub screen: Screen,
    pub split_index: Option<usize>,
}

#[derive(Clone, Copy)]
pub struct Screen {
    pub width: f32,
    pub height: f32,
    pub res_width: u32,
    pub res_height: u32,
}

pub struct ScreenIterator {
    width: u32,
    height: u32,
    x: u32,
    y: u32,
}

impl IntoIterator for Screen {
    type Item = (u32, u32);

    type IntoIter = ScreenIterator;

    fn into_iter(self) -> Self::IntoIter {
        ScreenIterator {
            width: self.res_width,
            height: self.res_height,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for ScreenIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.x += 1;
        if self.x == self.width {
            self.x = 0;
            self.y += 1;
        }
        if self.y == self.height {
            return None;
        }
        Some((self.x, self.y))
    }
}

impl CameraOrtho {
    pub fn new(
        pos: Vec3,
        subject: Vec3,
        width: f32,
        height: f32,
        res_width: u32,
        res_height: u32,
    ) -> CameraOrtho {
        CameraOrtho {
            pos,
            subject,
            screen: Screen {
                width,
                height,
                res_width,
                res_height,
            },
            split_index: None,
        }
    }

    pub fn subdivide_camera(&self, row_start: u32, row_end: u32, index: usize) -> CameraOrtho {
        let pos = self.pixel_to_clip_pos(
            self.screen.res_width as f32 / 2.0,
            (row_start as f32 + row_end as f32) / 2.0,
        );
        let subject = pos + self.subject - self.pos;
        CameraOrtho {
            pos,
            subject,
            screen: Screen {
                width: self.screen.width,
                height: self.screen.height * (row_start.abs_diff(row_end) as f32)
                    / (self.screen.res_height as f32),
                res_width: self.screen.res_width,
                res_height: row_start.abs_diff(row_end),
            },
            split_index: Some(index),
        }
    }

    pub fn pixel_to_clip_pos(&self, x_px: f32, y_px: f32) -> Vec3 {
        let dir = self.subject - self.pos;
        let x_basis = Vec3::new(-dir.y, dir.x, 0.0).normalize_or_zero();
        let y_basis = x_basis.cross(dir).normalize_or_zero();

        let x_px_trans = x_px - (self.screen.res_width / 2) as f32;
        let y_px_trans = -((self.screen.res_height / 2) as f32 - y_px);

        let x = self.screen.width / (self.screen.res_width as f32) * x_px_trans;
        let y = self.screen.height / (self.screen.res_height as f32) * y_px_trans;

        self.pos + x * x_basis + y * y_basis
    }

    pub fn pixel_to_photon(&self, x_px: u32, y_px: u32) -> Photon {
        Photon::new(
            self.pixel_to_clip_pos(x_px as f32, y_px as f32),
            self.subject - self.pos,
        )
    }

    pub fn render_png(&self, filename: &str, world: &World) {
        let mut image = RgbImage::new(self.screen.res_width, self.screen.res_height);
        let mut prog: u64 = 0;
        let base = 0.5f32.powf(2.0 / (MAX_STEPS as f32));

        for (x_px, y_px) in self.screen {
            if x_px == 0 && y_px % 10 == 0 {
                print!(
                    "\r\t{}: {:.1}%   ",
                    filename,
                    100.0 * (prog as f64) / (self.screen.res_width * self.screen.res_height) as f64
                );
                std::io::stdout().flush().unwrap();
            }
            prog += 1;
            let photon = self.pixel_to_photon(x_px, y_px);

            let col = if let Some((steps, diffuse)) =
                world.simulate_photon(photon, MAX_STEPS, STEP_SIZE)
            {
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

        println!("\r\t{}: 100.0%   ", filename);
        println!("Done Rendering {}. Saving...", filename);
        image.save(filename).unwrap();
        println!("Saved.")
    }
}
