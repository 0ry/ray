#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate image;

use image::{DynamicImage, GenericImage, Rgba, Pixel};

pub mod vector;
pub mod point;

use vector::Vector3;
use point::Point;

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

const GAMMA: f32 = 2.2;

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / GAMMA)
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

pub fn to_rgba(c: &Color) -> Rgba<u8> {
    Rgba::from_channels((gamma_encode(c.red) * 255.0) as u8,
                        (gamma_encode(c.green) * 255.0) as u8,
                        (gamma_encode(c.blue) * 255.0) as u8,
                        255)
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
}

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        assert!(scene.width > scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x = (((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        Ray {
            origin: Point::zero(),
            direction: Vector3 {
                    x: sensor_x,
                    y: sensor_y,
                    z: -1.0,
                }
                .normalize(),
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

// TODO understand this
impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        //Create a line segment between the ray origin and the center of the sphere
        let l: Vector3 = self.center - ray.origin;
        //Use l as a hypotenuse and find the length of the adjacent side
        let adj2 = l.dot(&ray.direction);
        //Find the length-squared of the opposite side
        //This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
        let d2 = l.dot(&l) - (adj2 * adj2);
        //If that length-squared is less than radius squared, the ray intersects the sphere
        d2 < (self.radius * self.radius)
    }
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            if scene.sphere.intersect(&ray) {
                image.put_pixel(x, y, to_rgba(&scene.sphere.color));
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }
    image
}

#[test]
fn can_render_scene_test() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1_f64,
            color: Color {
                red: 0.4,
                green: 1.0,
                blue: 0.4,
            },
        },
    };

    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}

fn main() {
    // let v = Vector3::zero();
    // let p = Point::zero();
    // let p1 = Point::from_one(1_f64);
    // let p2 = Point { x: 2_f64, y: 3_f64, z: 2_f64 };
    // println!("This vector {:?}, looks a lot like this point {:?}", v, p);
    // println!("p1 - p2 = {:?}", (p1 - p2));

}
