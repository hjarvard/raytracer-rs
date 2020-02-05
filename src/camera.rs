use crate::{Ray, Vec3};
use rand::prelude::*;
use std::f64::consts::PI;

fn random_in_unit_disk(rng: &mut ThreadRng) -> Vec3 {
    let mut p = Vec3::new(2.0, 2.0, 2.0);

    while Vec3::dot(&p, &p) >= 1.0 {
        p = Vec3::new(rng.gen(), rng.gen(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
    }
    p
}

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f64,
    pub rng: ThreadRng,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = Vec3::unit_vector(&(lookfrom.clone() - lookat.clone()));
        let u = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        Camera {
            lower_left_corner: lookfrom.clone()
                - u.clone() * focus_dist * half_width
                - v.clone() * focus_dist * half_height
                - w.clone() * focus_dist,
            horizontal: u.clone() * half_width * 2.0 * focus_dist,
            vertical: v.clone() * half_height * 2.0 * focus_dist,
            origin: lookfrom.clone(),
            lens_radius: aperture / 2.0,
            rng: rand::thread_rng(),
            w: w.clone(),
            u: u.clone(),
            v: v.clone(),
        }
    }

    pub fn get_ray(&mut self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk(&mut self.rng) * self.lens_radius;
        let offset = self.u.clone() * rd.x() + self.v.clone() * rd.y();

        Ray::new(
            self.origin.clone() + offset.clone(),
            self.lower_left_corner.clone()
                + (self.horizontal.clone() * s)
                + (self.vertical.clone() * t)
                - self.origin.clone()
                - offset.clone(),
        )
    }
}
