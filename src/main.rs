use image::{ImageBuffer, Rgb};
use nalgebra::{self, Point3, UnitVector3, Vector3};

mod object;
mod ray;

use object;
use ray::Ray;

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let imgx: u32 = 400;
    let imgy: u32 = (imgx as f64 / aspect_ratio) as u32;

    let mut imgbuf = ImageBuffer::new(imgx, imgy);

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * aspect_ratio;
    let focal_length: f64 = 1.0;

    let origin: Point3<f64> = Point3::origin();
    let horizontal: Vector3<f64> = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical: Vector3<f64> = Vector3::new(0.0, viewport_height, 0.0);
    let lower_left_corner: Point3<f64> =
        origin - horizontal.scale(0.5) - vertical.scale(0.5) - Vector3::new(0.0, 0.0, focal_length);

    for x in 0..imgx {
        for y in 0..imgy {
            let u = x as f64 / (imgx - 1) as f64;
            let v = y as f64 / (imgy - 1) as f64;
            let ray_origin = origin.clone();
            let ray_direction: Vector3<f64> =
                lower_left_corner + horizontal.scale(u as f64) + vertical.scale(v as f64) - origin;
            let ray_direction: UnitVector3<f64> = UnitVector3::new_normalize(ray_direction);
            let pixel = imgbuf.get_pixel_mut(x, imgy - y - 1);
            let ray = Ray::new(ray_origin, ray_direction);
            *pixel = ray_color(&ray);
        }
    }

    imgbuf.save("image.png").unwrap();
}

fn ray_color(ray: &Ray) -> Rgb<u8> {
    let direction = ray.direction;
    let t = 0.5 * (direction.y + 1.0);
    let r = (((1.0 - t) + 0.5 * t) * 255.0) as u8;
    let g = (((1.0 - t) + 0.7 * t) * 255.0) as u8;
    let b = 255 as u8;

    Rgb([r, g, b])
}
