use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    // change range
    pub fn output(self) -> Self {
        self * 255.99
    }
}
