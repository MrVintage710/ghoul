use bevy::{ecs::{component::Component, system::Commands}, math::{cubic_splines::CubicCurve, Vec3}, prelude::*};

//==============================================================================
//         CameraPathFollower
//==============================================================================

#[derive(Asset, TypePath, Debug)]
pub struct CameraPath {
    curve : CubicCurve<Vec3>,
}

//==============================================================================
//         CameraPathFollower
//==============================================================================

#[derive(Component)]
pub struct CameraPathFollower {
    curve : CubicCurve<Vec3>,
    pub path : Vec<CameraPathSegment>,
}

struct CameraPathSegment {
    target_location : Vec3,
    
    view_target : Vec3,
    length : f32,
}

impl CameraPathFollower {
    pub fn push_node(&mut self, location : Vec3, view_target : Vec3, duration : f32) {
        self.path.push(CameraPathSegment{
            target_location : location,
            view_target,
            length : duration,
        });
    }
}

fn follow_path (
    mut commands : Commands,
    mut 
    time : Res<Time>,
) {
    
}