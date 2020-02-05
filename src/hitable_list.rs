use crate::{
    Hitable,
    HitRecord,
    Ray,
};

pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = rec.clone();

        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for item in self.list.iter() {
            if item.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
