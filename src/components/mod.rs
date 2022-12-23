use ecs_macro::Component;
use glam::Vec2;
use sdl2::rect::Rect;
use time::Duration;

use crate::asset_store::AssetId;

#[derive(Debug, Clone, Copy, Component, Builder)]
pub struct TransformComponent {
    pub position: Vec2,
    #[builder(default = "Vec2::ONE")]
    pub scale: Vec2,
    #[builder(default = "0.0")]
    pub rotation: f32,
}


#[derive(Debug, Clone, Component)]
pub struct RigidBodyComponent {
    pub velocity: Vec2
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SpriteLayer {
    Tiles(u32),
    Enemies(u32),
    Ui(u32),
}

impl PartialOrd for SpriteLayer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SpriteLayer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (SpriteLayer::Tiles(a), SpriteLayer::Tiles(b)) => a.cmp(&b),
            (SpriteLayer::Tiles(_), SpriteLayer::Enemies(_)) => std::cmp::Ordering::Less,
            (SpriteLayer::Tiles(_), SpriteLayer::Ui(_)) => std::cmp::Ordering::Less,
            (SpriteLayer::Enemies(_), SpriteLayer::Tiles(_)) => std::cmp::Ordering::Greater,
            (SpriteLayer::Enemies(a), SpriteLayer::Enemies(b)) => a.cmp(&b),
            (SpriteLayer::Enemies(_), SpriteLayer::Ui(_)) => std::cmp::Ordering::Less,
            (SpriteLayer::Ui(_), SpriteLayer::Tiles(_)) => std::cmp::Ordering::Greater,
            (SpriteLayer::Ui(_), SpriteLayer::Enemies(_)) => std::cmp::Ordering::Greater,
            (SpriteLayer::Ui(a), SpriteLayer::Ui(b)) => a.cmp(&b)
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct SpriteComponent {
    pub width: u32,
    pub height: u32,
    pub asset_id: AssetId,
    pub src: Rect,
    pub layer: SpriteLayer
}

impl SpriteComponent {
    pub fn tile(width: u32, height: u32, asset_id: &str) -> Self {
        Self { width, height, asset_id: asset_id.to_owned(), src: Rect::new(0, 0, width, height), layer: SpriteLayer::Tiles(1) }
    }
    
    pub fn enemy(width: u32, height: u32, asset_id: &str) -> Self {
        Self { width, height, asset_id: asset_id.to_owned(), src: Rect::new(0, 0, width, height), layer: SpriteLayer::Enemies(1) }
    }

    pub fn ui(width: u32, height: u32, asset_id: &str) -> Self {
        Self { width, height, asset_id: asset_id.to_owned(), src: Rect::new(0, 0, width, height), layer: SpriteLayer::Ui(1) }
    }
}


#[derive(Debug, Clone, Component, Builder)]
pub struct AnimationComponent {
    pub num_of_frames: u32,
    #[builder(default = "1")]
    pub current_frame: u32,
    #[builder(default = "1")]
    pub frame_rate_speed: u32,
    #[builder(default = "true")]
    pub should_loop: bool,
    pub start_time: Duration,
}

#[derive(Debug, Clone, Component, Builder)]
pub struct BoxColliderComponent {
    pub width: u32,
    pub height: u32,
    #[builder(default = "Vec2::ZERO")]
    pub offset: Vec2
}

#[derive(Debug, Clone, Component, Builder)]
pub struct KeyboardControlledComponent {
    #[builder(default = "Vec2::ZERO")]
    pub up_velocity: Vec2,
    #[builder(default = "Vec2::ZERO")]
    pub right_velocity: Vec2,
    #[builder(default = "Vec2::ZERO")]
    pub down_velocity: Vec2,
    #[builder(default = "Vec2::ZERO")]
    pub left_velocity: Vec2
}

#[derive(Debug, Clone, Component)]
pub struct CameraFollowComponent;
