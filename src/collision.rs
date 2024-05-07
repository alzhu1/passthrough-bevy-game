use bevy::{math::bounding::Aabb2d, prelude::*};

#[derive(Component, Debug, Default)]
pub struct Collider {
    pub width: f32,
    pub height: f32,
    // Bitmask?
    pub layer_mask: u8,
}

impl Collider {
    pub fn get_aabb2d(&self, center: Vec2) -> Aabb2d {
        Aabb2d::new(center, Vec2::new(self.width / 2.0, self.height / 2.0))
    }
}
