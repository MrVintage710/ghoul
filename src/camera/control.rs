use bevy::{ecs::component::Component, math::Vec3};



//==============================================================================
//         CameraPathFollower
//==============================================================================

#[derive(Component)]
pub struct CameraPathFollower {
    pub path : Vec<Vec3>,
    pub speed : f32,
    pub current : usize,
    pub target : Vec3,
}

struct CameraPathSegment {
    target_location : Option<Vec3>,
    length : f32,
}