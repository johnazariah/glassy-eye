
use super::Vec3D;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point3D {
    x : f64,
    y : f64,
    z : f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }
}

impl std::ops::Add<Vec3D> for Point3D {
    type Output = Point3D;
    fn add(self, rhs: Vec3D) -> Self::Output {
        Point3D::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

pub struct Ray { 
    pub origin: Point3D,
    pub direction: Vec3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Vec3D) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3D {
        self.origin + (self.direction * t)
    }
}