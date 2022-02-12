use crate::{hittable::{Hittable, HitRecord}, ray};

type HittablePtr = Box<dyn Hittable>;

pub struct HittableList {
    pub objects: Vec<HittablePtr>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: vec![]
        }
    }

    pub fn with_object(object: HittablePtr) -> Self {
        HittableList {
            objects: vec![object]
        }
    }

    pub fn add(&mut self, object: HittablePtr) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record_result = None;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(r, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = hit_record.t;
                record_result = Some(hit_record);
            }
        }

        if hit_anything {
            record_result
        } else {
            None
        }
    }
}