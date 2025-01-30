use std::io::Write;

use glam::Vec3;
use image::{Rgb, RgbImage};

use crate::{photon::Photon, world::World};

const MAX_STEPS: u32 = 350;
const STEP_SIZE: f32 = 0.05;

pub trait Visible: Send + Sync {
    fn overlap(&self, point: &Vec3) -> Option<[f32; 3]>;
}

pub struct Ortho {
    pub pos: Vec3,
    pub subject: Vec3,
    pub screen: Screen,
    pub split_index: Option<usize>,
    pub exposure: f32,
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

#[allow(clippy::cast_precision_loss, clippy::cast_sign_loss, clippy::cast_possible_truncation)]
impl Ortho {
    pub fn new(
        pos: Vec3,
        subject: Vec3,
        width: f32,
        height: f32,
        res_width: u32,
        res_height: u32,
        exposure: f32,
    ) -> Ortho {
        Ortho {
            pos,
            subject,
            screen: Screen {
                width,
                height,
                res_width,
                res_height,
            },
            split_index: None,
            exposure,
        }
    }

    pub fn subdivide_camera(&self, row_start: u32, row_end: u32, index: usize) -> Ortho {
        let pos = self.pixel_to_clip_pos(
            self.screen.res_width as f32 / 2.0,
            (row_start as f32 + row_end as f32) / 2.0,
        );
        let subject = pos + self.subject - self.pos;
        Ortho {
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
            exposure: self.exposure,
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

        for (prog, (x_px, y_px)) in (0u64..).zip(self.screen.into_iter()) {
            if x_px == 0 && y_px % 10 == 0 {
                print!(
                    "\r\t{}: {:.1}%   ",
                    filename,
                    100.0 * (prog as f64)
                        / f64::from(self.screen.res_width * self.screen.res_height)
                );
                std::io::stdout().flush().unwrap();
            }
            let photon = self.pixel_to_photon(x_px, y_px);

            let col = if let Some(diffuse) = world.simulate_photon(photon, MAX_STEPS, STEP_SIZE) {
                [
                    self.exposure * diffuse[0],
                    self.exposure * diffuse[1],
                    self.exposure * diffuse[2],
                ]
            } else {
                [0.0, 0.0, 0.0]
            };
            image.put_pixel(
                x_px,
                y_px,
                Rgb(col
                    .iter()
                    .map(|x| (x.clamp(0.0, 1.0) * 255.0) as u8)
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap()),
            );
        }

        println!("\r\t{filename}: 100.0%   ");
        println!("Done Rendering {filename}. Saving...");
        image.save(filename).unwrap();
        println!("Saved.");
    }
}
