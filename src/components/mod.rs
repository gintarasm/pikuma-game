use ecs_macro::Component;
use glam::Vec2;

#[derive(Debug, Clone, Copy, Component)]
pub struct TransformComponent {
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
}


#[derive(Debug, Clone, Copy, Component)]
pub struct RigidBodyComponent {
    pub velocity: Vec2
}

#[derive(Debug, Clone, Copy, Component)]
pub struct SpriteComponent {}
