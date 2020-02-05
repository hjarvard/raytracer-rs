use crate::{HitRecord, Hitable, Material, Ray, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(cen: Vec3, r: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin().clone() - self.center.clone();
        let a = Vec3::dot(r.direction(), r.direction());
        let b = Vec3::dot(&oc, r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p.clone() - self.center.clone()) / self.radius;
                rec.material = self.material.clone();
                return true;
            }

            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p.clone() - self.center.clone()) / self.radius;
                rec.material = self.material.clone();
                return true;
            }
        }

        false
    }
}
