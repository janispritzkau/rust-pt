use std::time::{Duration, Instant};
use std::path::Path;
use std::io;
use image;
use rand::{Rng, XorShiftRng};

use math::*;
use ray::*;
use color::XYZ;
use scene::Scene;

pub struct Renderer {
    scene: Scene,
    width: usize,
    height: usize,
    samples: usize,
    min_depth: usize,
    max_depth: usize
}

impl Renderer {
    pub fn new(scene: Scene, width: usize, height: usize,
        samples: usize, min_depth: usize, max_depth: usize) -> Self {
        Self { scene, width, height, samples, min_depth, max_depth }
    }

    fn radiance(&self, ray: &Ray, rng: &mut XorShiftRng, depth: usize) -> XYZ {
        if depth > self.max_depth || depth > self.min_depth && rng.gen::<f32>() < 0.4 { return XYZ::black() }

        let (hit, material) = if let Some(x) = self.scene.intersect(ray) { x } else {
            return XYZ::new(0.018, 0.021, 0.05) + XYZ::white() * 0.03 * (1.0 - ray.direction.dot(Vector3::unit_z()).max(0.0)).powi(2)
        };

        let mut color: XYZ = if let Some(color) = material.emission(&hit) { color } else { XYZ::black() };

        if color.luminance() == 0.0 {
            let (ray, refl_color) = material.bsdf(&hit, ray, rng);
            color = color + self.radiance(&ray, rng, depth + 1) * refl_color;
        }
        // let v = if ((hit.position.x.floor() + hit.position.y.floor()) % 2.0).abs() < 1.0 {
        //     0.8
        // } else {
        //     0.4
        // };

        // XYZ::white() * hit.normal.dot(-ray.direction).max(0.0) * v
        color
    }

    pub fn render_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let mut image = image::RgbImage::new(self.width as u32, self.height as u32);
        let mut buffer = vec![XYZ::black(); self.width * self.height];

        let mut rng = XorShiftRng::new_unseeded();
        let start_time = Instant::now();
        let mut last_time = start_time;

        for s in 0..self.samples {
            let elapsed = start_time.elapsed();
            let elapsed_s = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 * 1e-9;

            let should_save_image = s + 1 == self.samples ||
            last_time.elapsed() >= Duration::from_millis(
                ((elapsed_s as f32 / 2.0).powf(0.6) * 1000.0) as u64
            ) && elapsed_s != 0.0;

            for y in 0..self.height { for x in 0..self.width {
                let pixel = &mut buffer[y * self.width + x];
                let ray = self.scene.camera.generate_ray(x as f32 + rng.gen::<f32>(), y as f32 + rng.gen::<f32>(), &mut rng);
                let color = self.radiance(&ray, &mut rng, 0);
                *pixel = (*pixel * s as f32 + color) / (s as f32 + 1.0);
                let c = pixel.to_srgb();
                if should_save_image {
                    image[(x as u32, y as u32)] =
                    image::Rgb([(c.x * 255.0) as u8, (c.y * 255.0) as u8, (c.z * 255.0) as u8]);
                }
            } }

            if should_save_image {
                let elapsed = start_time.elapsed();
                let elapsed_s = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 * 1e-9;
                println!("{:>3}% done | {:>7.1} s elapsed | {:>7.1} remaining", 100 * (s + 1) / self.samples,
                elapsed_s, elapsed_s / (s + 1) as f32 * (self.samples - s) as f32);
                try!(image.save(&path));
                last_time = Instant::now();
            }
        }

        Ok(())
    }
}
