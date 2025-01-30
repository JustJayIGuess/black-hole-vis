use std::{f32::consts::PI, sync::Arc, time::SystemTime};

use black_hole_vis::camera::Ortho;
use black_hole_vis::objects::Disk;
use black_hole_vis::world::World;
use black_hole_vis::masses::StaticMass;
use glam::Vec3;

const NUM_THREADS: usize = 10;
const RES_WIDTH: u32 = 1920;
const RES_HEIGHT: u32 = 1080;
const FRAMES: usize = 210;

/* This might take a while.
When done, use something like ffmpeg to join stills into a video.
E.g.,
ffmpeg -framerate 30 -pattern_type glob -i 'out/ *.png' \
-c:v libx264 -pix_fmt yuv420p out.mp4
*/
#[allow(clippy::cast_precision_loss)]
fn main() {
    let mut world = World::new();
    let now = SystemTime::now();

    for i in 0..FRAMES {

        let t = i as f32 / FRAMES as f32;

        world.add_object(Arc::new(Disk {
            pos: Vec3::new(0.0, 0.0, 0.0),
            inner_rad: 1.0,
            outer_rad: 8.0,
            height: 0.1,
            col: [1.0, 0.5, 0.3],
        }));

        world.add_mass(StaticMass {
            pos: Vec3::new(0.0, 0.0, 0.0),
            mass: 10.0,
        });

        world.add_camera(Ortho::new(
            Vec3::new(
                8.0 * (4.0 * PI * t).cos(),
                8.0 * (4.0 * PI * t).sin(),
                3.0 * (1.0 / (1.0 + (20.0 * (t - 0.5)).exp()) - 0.5),
            ),
            Vec3::new(0.0, 0.0, 0.0),
            15.0 * RES_WIDTH as f32 / RES_HEIGHT as f32,
            15.0,
            RES_WIDTH,
            RES_HEIGHT,
            0.55,
        ));

        world.split_and_par_render(NUM_THREADS, &format!("out/frame_{i:0>6}.png"));
        world.clear_objects();
        world.clear_cameras();
        world.clear_masses();

        println!("Saved frame {i} after {:?}.", now.elapsed().unwrap());
    }
}
