use super::vec::{Vec3, Point3};

pub struct Ray{
	origin: Point3,
	dir: Vec3
}

impl Ray{
	pub fn new(originPoint: Point3, direction: Vec3) -> Ray{
		Ray{
			origin: originPoint,
			dir: direction
		}
	}

	pub fn origin(&self) -> Point3{
		self.origin
	}

	pub fn direction(&self) -> Vec3{
		self.dir
	}

	pub fn at(&self, t: f64) -> Point3{
		self.origin + t*self.dir
	}
}
