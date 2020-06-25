mod image;

use argh::FromArgs;
use image::ray::vec3::{Location, UnitDirection};
use image::ray::{Background, Sphere};
use std::fs;

#[derive(FromArgs)]
/// Generate PNG images given a json file containing the coordinates of spheres and their color
struct Args {
    /// spheres file name (must end by ".json")
    #[argh(positional)]
    spheres: String,
    /// output file name (must end by ".png")
    #[argh(option, short = 'o')]
    output: Option<String>,
    /// width of the image to generate ()
    #[argh(option, short = 'w')]
    width: Option<usize>,
    /// todo
    #[argh(option, short = 'h')]
    height: Option<usize>,
    /// todo
    #[argh(option, short = 'l')]
    camera_location: Option<Location>,
    /// todo
    #[argh(option, short = 'd')]
    camera_direction: Option<UnitDirection>,
    /// todo
    #[argh(option, short = 'b')]
    background: Option<Background>,
    /// todo
    #[argh(option, short = 'f')]
    focal_length: Option<f64>,
}

fn main() {
    let args: Args = argh::from_env();
    if !args.spheres.ends_with(".json") {
        panic!("todo");
    }
    let objects: Vec<Sphere> =
        serde_json::from_str(&fs::read_to_string(&args.spheres).expect("todo")).expect("todo");
    let output = match args.output {
        Some(file_path) => {
            if !file_path.ends_with(".png") {
                panic!("todo");
            }
            file_path
        }
        None => args.spheres.replace(".json", ".png"),
    };
    let mut image = image::Image::new();
    match args.width {
        Some(width) => image.width = width,
        None => (),
    }
    match args.height {
        Some(height) => image.height = height,
        None => (),
    }
    match args.camera_location {
        Some(location) => image.location = location,
        None => (),
    }
    match args.camera_direction {
        Some(direction) => image.direction = direction,
        None => (),
    }
    match args.focal_length {
        Some(focal_length) => image.focal_length = focal_length,
        None => (),
    }
    match args.background {
        Some(background) => image.background = background,
        None => (),
    }
    image.compute(objects);
    image.write(&output);
}
