use crate::{Material, Ray, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
