use rand::{Rng};

use super::vec::{Vec3, Color};
use super::ray::Ray;
use super::hit::HitRecord;

pub trait Scatter{
	fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian{
	albedo: Color
}
pub struct Metal {
    albedo: Color
}

impl Lambertian{
	pub fn new(a: Color) -> Lambertian{
		Lambertian{
			albedo: a
		}
	}
}

impl Metal {
    pub fn new(a: Color) -> Metal {
        Metal {
            albedo: a
        }
    }
}

impl Scatter for Lambertian{
	fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
		let mut scatter_dir = rec.normal + Vec3::random_in_unit_sphere().normalized();
		if scatter_dir.near_zero(){
			scatter_dir = rec.normal;
		}
		let scattered = Ray::new(rec.p, scatter_dir);

		Some((self.albedo, scattered))
	}
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(rec.normal).normalized();
        let scattered = Ray::new(rec.p, reflected);

        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}