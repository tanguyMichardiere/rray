pub mod ray;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use png::{BitDepth, ColorType, Encoder};
use rand::Rng;
use ray::color::{Color, SuperColor};
use ray::vec3::{Direction, Location, UnitDirection};
use ray::{Background, Ray, Sphere};
use rayon::prelude::*;
use std::f64::consts::PI;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub multisampling: u8,
    pub location: Location,
    pub direction: UnitDirection,
    pub focal_length: f64,
    pub background: Background,
    pub data: Option<Vec<u8>>,
}

struct Viewport {
    origin: Location,
    corner: Location,
    x_step: Direction,
    y_step: Direction,
}

impl Image {
    pub fn new() -> Self {
        Image {
            width: 1920,
            height: 1080,
            multisampling: 100,
            location: Location::new(0.0, 0.0, 1.0),
            direction: UnitDirection::new(0.0, 0.0, -1.0),
            focal_length: 2.0,
            background: Background::BlueGradient,
            data: None,
        }
    }

    fn aspect_ratio(&self) -> f64 {
        (self.width as f64) / (self.height as f64)
    }

    fn viewport(&self) -> Viewport {
        let center = self.location + self.focal_length * self.direction;
        let ar = self.aspect_ratio();
        let mut hor = self
            .direction
            .rot(UnitDirection::new(0.0, 1.0, 0.0), -PI / 2.0);
        hor.set_y(0.0);
        let ver = (self.direction ^ -hor).as_unit_vector();
        let corner = center - ar * hor + ver;
        Viewport {
            origin: self.location,
            corner,
            x_step: (2.0 * ar / self.width as f64) * hor,
            y_step: (-2.0 / self.height as f64) * ver,
        }
    }

    fn color(&self, ray: Ray, spheres: &Vec<Sphere>) -> Color {
        if ray.is_dead() {
            return self.background.color(ray);
        }
        let mut lowest_t = ray.range;
        let mut hit = None;
        for sphere in spheres {
            match sphere.hit(&ray) {
                Some(t) => {
                    if t < lowest_t {
                        lowest_t = t;
                        hit = Some(sphere);
                    }
                }
                None => (),
            }
        }
        match hit {
            Some(sphere) => {
                let mut res = SuperColor::new();
                let normal = (ray.at(lowest_t) - sphere.center).as_unit_vector();
                // res.add(normal.as_color());
                res.add(sphere.color.clone());
                res.add(ray::color::BLACK);
                res.add(self.color(ray.diffuse(lowest_t, normal), spheres));
                res.as_color()
            }
            None => self.background.color(ray),
        }
    }

    fn compute_pixel(
        &self,
        viewport: Viewport,
        x: usize,
        y: usize,
        spheres: &Vec<Sphere>,
    ) -> Color {
        let mut res = SuperColor::new();
        let mut rng = rand::thread_rng();
        for _ in 0..self.multisampling {
            let ray = Ray::new(
                viewport.origin,
                (viewport.corner
                    + (x as f64 + rng.gen::<f64>()) * viewport.x_step
                    + (y as f64 + rng.gen::<f64>()) * viewport.y_step
                    - viewport.origin)
                    .as_unit_vector(),
            );
            res.add(self.color(ray, spheres));
        }
        res.as_color()
    }

    pub fn compute(&mut self, spheres: Vec<Sphere>) {
        let mut data = vec![0; 3 * self.width * self.height];
        data.par_chunks_mut(3 * self.width)
            .enumerate()
            .progress_with(
                ProgressBar::new(self.height as u64)
                    .with_style(ProgressStyle::default_bar().template("{wide_bar} ETA: {eta}")),
            )
            .for_each(|(y, row)| {
                row.chunks_mut(3).enumerate().for_each(|(x, pixel)| {
                    let color = self.compute_pixel(self.viewport(), x, y, &spheres);
                    pixel[0] = color.get_red();
                    pixel[1] = color.get_green();
                    pixel[2] = color.get_blue();
                })
            });
        self.data = Some(data);
    }

    pub fn write(self, file_name: &str) {
        match self.data {
            Some(data) => {
                let file = File::create(Path::new(file_name)).expect("Error creating output file");
                let buffer = BufWriter::new(file);
                let mut encoder = Encoder::new(buffer, self.width as u32, self.height as u32);
                encoder.set_color(ColorType::RGB);
                encoder.set_depth(BitDepth::Eight);
                let mut writer = encoder.write_header().expect("Error creating png header");
                writer
                    .write_image_data(&data)
                    .expect("Error writing png data");
            }
            None => eprintln!("Error: call Image::compute before Image::write"),
        }
    }
}
