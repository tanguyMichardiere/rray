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
    /// width of the image to generate (default: 1920)
    #[argh(option, short = 'w')]
    width: Option<usize>,
    /// height of the image to generate (default: 1080)
    #[argh(option, short = 'h')]
    height: Option<usize>,
    /// multisampling (default: 100)
    #[argh(option, short = 'm')]
    multisampling: Option<u8>,
    /// location of the camera (default: (0,0,0))
    #[argh(option, short = 'l')]
    camera_location: Option<Location>,
    /// direction of the camera (default: (0,0,-1))
    #[argh(option, short = 'd')]
    camera_direction: Option<UnitDirection>,
    /// field of view of the camera in degrees (default: 80)
    #[argh(option, short = 'f')]
    fov: Option<f64>,
    /// background of the image (default: blue gradient)
    #[argh(option, short = 'b')]
    background: Option<Background>,
}

fn main() {
    let args: Args = argh::from_env();
    if !args.spheres.ends_with(".json") {
        panic!("Error: spheres file must be a json file");
    }
    let objects: Vec<Sphere> = serde_json::from_str(
        &fs::read_to_string(&args.spheres).expect(&format!("Error opening {}", &args.spheres)),
    )
    .expect(&format!("Error parsing {}", &args.spheres));
    let output = match args.output {
        Some(file_path) => {
            if !file_path.ends_with(".png") {
                panic!("Error: output file must be a png file");
            }
            file_path
        }
        None => args.spheres.replace(".json", ".png"),
    };
    let mut image = image::Image::new(
        args.width.unwrap_or(1920),
        args.height.unwrap_or(1080),
        args.multisampling.unwrap_or(100),
        args.camera_location.unwrap_or(Location::new(0.0, 0.0, 0.0)),
        args.camera_direction
            .unwrap_or(UnitDirection::new(0.0, 0.0, -1.0)),
        args.fov.unwrap_or(80.0),
        args.background.unwrap_or(Background::BlueGradient),
    );
    image.compute(objects);
    image.write(&output);
}
