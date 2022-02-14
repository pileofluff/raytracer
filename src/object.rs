use crate::Point3;
use crate::Ray;
use crate::UnitVector3;
use crate::Vector3;

#[derive(Debug)]
pub struct CollisionRecord {
    pub coordinates: Point3<f64>,
    pub normal: UnitVector3<f64>,
    pub t: f64,
    pub front_face: bool,
}

impl CollisionRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &UnitVector3<f64>) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {*outward_normal} else {-*outward_normal};
    }
}

pub trait Collide {
    fn collide(&self, ray: &Ray, t_interval: (f64, f64), rec: &mut CollisionRecord) -> bool;
}

#[derive(Debug)]
pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Collide for Sphere {
    fn collide(&self, ray: &Ray, t_interval: (f64, f64), rec: &mut CollisionRecord) -> bool {
        let oc: Vector3<f64> = ray.origin - self.center;
        let a: f64 = ray.direction.norm_squared();
        let half_b: f64 = oc.dot(&ray.direction);
        let c: f64 = oc.norm_squared() - self.radius * self.radius;
        let disc: f64 = half_b * half_b - a * c;

        if disc < 0.0 {
            false
        } else {
            let disc_sqrt: f64 = disc.sqrt();
            let mut root: f64 = (-half_b - disc_sqrt) / a;

            if root < t_interval.0 || t_interval.1 < root {
                root = (-half_b + disc_sqrt) / a;
                if root < t_interval.0 || t_interval.1 < root {
                    return false;
                }
            }

            rec.t = root;
            rec.coordinates = ray.coordinates_at(rec.t);
            let outward_normal: UnitVector3<f64> = UnitVector3::new_normalize(rec.coordinates - self.center);
            rec.set_face_normal(ray, &outward_normal);

            true
        }
    }
}
