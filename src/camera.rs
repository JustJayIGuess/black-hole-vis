use nalgebra::Vector3;

use crate::photon::Photon;

pub trait Visible {
    fn overlap(&self, point: &Vector3<f32>) -> Option<[f32; 3]>;
}

pub struct CameraOrtho {
    pub pos: Vector3<f32>,
    pub subject: Vector3<f32>,
    pub screen: Screen,
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
        pos: Vector3<f32>,
        subject: Vector3<f32>,
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
        }
    }

    pub fn pixel_to_clip_pos(&self, x_px: u32, y_px: u32) -> Vector3<f32> {
        let dir = self.subject - self.pos;
        let mut x_basis = Vector3::new(-dir.y, dir.x, 0.0);
        x_basis.normalize_mut();
        let mut y_basis = x_basis.cross(&dir);
        y_basis.normalize_mut();

        let x_px_trans = x_px as i32 - (self.screen.res_width / 2) as i32;
        let y_px_trans = -((self.screen.res_height / 2) as i32 - y_px as i32);

        let x = self.screen.width / (self.screen.res_width as f32) * (x_px_trans as f32);
        let y = self.screen.height / (self.screen.res_height as f32) * (y_px_trans as f32);

        self.pos + x * x_basis + y * y_basis
    }

    pub fn pixel_to_photon(&self, x_px: u32, y_px: u32) -> Photon {
        Photon::new(self.pixel_to_clip_pos(x_px, y_px), self.subject - self.pos)
    }
}
