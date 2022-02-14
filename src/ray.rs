use crate::Point3;
use crate::UnitVector3;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: UnitVector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: UnitVector3<f64>) -> Self {
        Ray { origin, direction }
    }

    pub fn coordinates_at(&self, t: f64) -> Point3<f64> {
        self.origin + self.direction.scale(t)
    }
}
