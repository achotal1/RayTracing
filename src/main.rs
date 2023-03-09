mod vec;
mod ray;
mod hit;
mod sphere;
mod camera;

use std::io::{stderr, Write};
use vec::{Vec3, Point3, Color};
use ray::Ray;
use hit::{Hit, World};
use sphere::Sphere;
use camera::Camera;
use rand::prelude::*;



fn ray_color(r: &Ray, world: &World, depth: u64) -> Color{
	// if ray hits sphere
	if depth <= 0{
		return Color::new(0.0, 0.0, 0.0);
	}

	if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
		let target = rec.p + rec.normal + Vec3::random_in_unit_sphere().normalized();
		let r = Ray::new(rec.p, target - rec.p);
        0.5 * ray_color(&r, world, depth - 1)
    } else {
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}


fn main() {
	const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMG_HEIGHT: u64 = ((256 as f64)/ASPECT_RATIO) as u64;
    const IMG_WIDTH : u64 = 256;
    const  SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 5;
    // world
    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    //making the camera
    let cam = Camera::new();


    println!("P3");
    println!("{} {}", IMG_WIDTH, IMG_HEIGHT);
    println!("255");
    let mut rng = rand::thread_rng();

    for j in (0..IMG_HEIGHT).rev(){
    	eprintln!("Scanlines remaining: {:3}", IMG_HEIGHT - j - 1);
    	stderr().flush().unwrap();

    	for i in 0..IMG_WIDTH{
    		let mut pixel_color = Color::new(0.0,0.0,0.0);
    		for _ in 0..SAMPLES_PER_PIXEL{
    			let random_u: f64 = rng.gen();
    			let random_v: f64 = rng.gen();
    			let u = ((i as f64) + random_u)/((IMG_WIDTH - 1) as f64);
    			let v = ((j as f64) + random_v)/((IMG_HEIGHT - 1) as f64);
    			let r = cam.get_ray(u,v);
    			pixel_color += ray_color(&r, &world, MAX_DEPTH);
    		}

            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));

    	}
    }
    eprintln!("Done");


}
