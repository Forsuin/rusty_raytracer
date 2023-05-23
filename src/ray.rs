use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(self, t: f64) -> Vec3 {
        self.origin + t * self.dir
    }

    pub fn origin(self) -> Vec3 {
        self.origin
    }

    pub fn dir(self) -> Vec3 {
        self.dir
    }
}

impl Default for Ray {
    fn default() -> Self {
        Ray {
            origin: Vec3::default(),
            dir: Vec3::default(),
        }
    }
}