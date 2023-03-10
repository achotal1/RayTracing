use super::vec::{Vec3, Point3};
use super::ray::Ray;
use super::hit::{Hit, HitRecord};
use super::material::Scatter;
use std::rc::Rc;

pub struct Sphere{
	center: Point3,
	radius: f64,
	mat: Rc<dyn Scatter>
}

impl Sphere{
	pub fn new(cen: Point3, r: f64, m: Rc<dyn Scatter>) -> Sphere{
		Sphere{
			center: cen,
			radius: r,
			mat: m
		}
	}

}

impl Hit for Sphere{
	/*
		finding roots for this equation:
		t^2*b⋅b+2tb⋅(A−C)+(A−C)⋅(A−C)−r2=0
		b^2 - 4ac
		(-b - discriminant.squareroot)/2a
    */
	fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
		let oc = r.origin() - self.center;
    	let a = r.direction().length().powi(2);
    	let b = oc.dot(r.direction());
	    let c = oc.length().powi(2) - self.radius.powi(2);
 		let discriminant = b * b - a * c;
	    
	    if discriminant < 0.0{
	    	return None;
	    }

	    let sqrt = discriminant.sqrt();
	    let mut root = (-b - sqrt)/a;

	    if root < t_min || t_max < root{
	    	root = (-b + sqrt)/a;
	    	if root < t_min || t_max < root{
	    		return None;
	    	}
	    }


	    let mut rec = HitRecord{
	    	p: r.at(root),
	    	normal: Vec3::new(0.0, 0.0, 0.0),
	    	mat: self.mat.clone(),
	    	t: root,
	    	front_face: false
	    };

	    let outward_normal = (rec.p - self.center)/self.radius;
	    rec.set_face_normal(r, outward_normal);

	    Some(rec)
	}

}