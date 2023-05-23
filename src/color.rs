use std::fmt::{Display, Formatter};
use std::io;
use std::io::Write;
use crate::vec3::Vec3;

pub type Color = Vec3;

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", (255.999 * self.x()) as i32, (255.999 * self.y()) as i32, (255.999 * self.z()) as i32)
    }
}