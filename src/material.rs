use crate::{HitRecord, Ray, Vec3};
use rand::prelude::*;

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v.clone() - n.clone() * Vec3::dot(v, n) * 2.0
}

fn refract(v: Vec3, n: &Vec3, ni_over_nt: f64, refracted: &mut Vec3) -> bool {
    let uv = Vec3::unit_vector(&v);
    let dt = Vec3::dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        *refracted = (uv - n.clone() * dt) * ni_over_nt - n.clone() * discriminant.sqrt();
        true
    } else {
        false
    }
}

pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    let mut p = Vec3::new(rng.gen(), rng.gen(), rng.gen()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);

    while p.squared_length() >= 1.0 {
        p = Vec3::new(rng.gen(), rng.gen(), rng.gen()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

pub trait Material: MaterialClone {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub trait MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        mut rng: &mut ThreadRng,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = rec.p.clone() + rec.normal.clone() + random_in_unit_sphere(&mut rng);
        *scattered = Ray::new(rec.p.clone(), target - rec.p.clone());
        *attenuation = self.albedo.clone();
        return true;
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, f: f64) -> Metal {
        Metal {
            albedo,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&Vec3::unit_vector(r_in.direction()), &rec.normal);
        *scattered = Ray::new(
            rec.p.clone(),
            reflected + random_in_unit_sphere(rng) * self.fuzz,
        );
        *attenuation = self.albedo.clone();
        return (Vec3::dot(&scattered.direction(), &rec.normal)) > 0.0;
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        rng: &mut ThreadRng,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let outward_normal;
        let reflected = reflect(r_in.direction(), &rec.normal);
        let ni_over_nt;
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut refracted = Vec3::zero();
        let reflect_prob;
        let cosine;

        if Vec3::dot(r_in.direction(), &rec.normal) > 0.0 {
            outward_normal = -rec.normal.clone();
            ni_over_nt = self.ref_idx;
            cosine =
                Vec3::dot(r_in.direction(), &rec.normal) * self.ref_idx / r_in.direction().length();
        } else {
            outward_normal = rec.normal.clone();
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -Vec3::dot(r_in.direction(), &rec.normal) / r_in.direction().length();
        }

        if refract(
            r_in.direction().clone(),
            &outward_normal,
            ni_over_nt,
            &mut refracted,
        ) {
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            reflect_prob = 1.0;
        }

        if rng.gen::<f64>() < reflect_prob {
            *scattered = Ray::new(rec.p.clone(), reflected);
        } else {
            *scattered = Ray::new(rec.p.clone(), refracted);
        }

        true
    }
}
