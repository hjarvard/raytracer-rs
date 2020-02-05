use rand::prelude::*;
use raytrace_rs::{
    Camera, Dielectric, HitRecord, Hitable, HitableList, Lambertian, Material, Metal, Ray, Sphere,
    Vec3,
};
//use std::f64::consts::PI;

fn random_scene(rng: &mut ThreadRng) -> HitableList {
    let mut list: Vec<Box<dyn Hitable>> = vec![Box::new(Sphere::new(
        Vec3::new(0.0, -1000., 0.0),
        1000.0,
        Box::new(Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        }),
    ))];

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;

            let choose_mat: f64 = rng.gen();
            let center = Vec3::new(a + 0.9 * rng.gen::<f64>(), 0.2, b + 0.9 * rng.gen::<f64>());

            if (center.clone() - Vec3::new(4.0, 2.0, 0.0)).length() > 0.9 {
                let material: Box<dyn Material> = if choose_mat < 0.8 {
                    Box::new(Lambertian {
                        albedo: Vec3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        ),
                    })
                } else if choose_mat < 0.95 {
                    Box::new(Metal::new(
                        Vec3::new(
                            0.5 * (1.0 + rng.gen::<f64>()),
                            0.5 * (1.0 + rng.gen::<f64>()),
                            0.5 * (1.0 + rng.gen::<f64>()),
                        ),
                        0.5 * rng.gen::<f64>(),
                    ))
                } else {
                    Box::new(Dielectric::new(1.5))
                };

                list.push(Box::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        }),
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    HitableList { list }
}

fn color(rng: &mut ThreadRng, r: &Ray, world: &dyn Hitable, depth: i32) -> Vec3 {
    let mut rec = HitRecord {
        t: 0.0,
        p: Vec3::zero(),
        normal: Vec3::zero(),
        material: Box::new(Lambertian {
            albedo: Vec3::zero(),
        }),
    };

    if world.hit(&r, 0.001, std::f64::MAX, &mut rec) {
        let mut scattered = Ray::new(Vec3::zero(), Vec3::zero());
        let mut attenuation = Vec3::zero();
        if depth < 50
            && rec
                .material
                .scatter(rng, &r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * color(rng, &scattered, world, depth + 1);
        } else {
            return Vec3::zero();
        }
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let nx = 384;
    let ny = 192;
    let ns = 100;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let mut rng = rand::thread_rng();

    let lookfrom = Vec3::new(12.0, 2.0, 3.5);
    let lookat = Vec3::new(0.0, 0.5, 0.0);
    let dist_to_focus = (lookfrom.clone() - lookat.clone()).length();
    let aperture = 0.1;

    let world = random_scene(&mut rng);
    let mut cam = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
    );

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zero();

            for _s in 0..ns {
                let u: f64 = ((i as f64) + rng.gen::<f64>()) / nx as f64;
                let v: f64 = ((j as f64) + rng.gen::<f64>()) / ny as f64;
                let r = cam.get_ray(u, v);
                col += color(&mut rng, &r, &world, 0);
            }

            col /= ns as f64;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

            let ir = (col.r() * 255.99).trunc();
            let ig = (col.g() * 255.99).trunc();
            let ib = (col.b() * 255.99).trunc();

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
