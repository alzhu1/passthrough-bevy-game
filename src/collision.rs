use bevy::{math::bounding::Aabb2d, prelude::*};

#[derive(Component)]
pub struct Collider {
    pub width: f32,
    pub height: f32,
}

impl Collider {
    pub fn get_aabb2d(&self, center: Vec2) -> Aabb2d {
        Aabb2d::new(center, Vec2::new(self.width / 2.0, self.height / 2.0))
    }
}
