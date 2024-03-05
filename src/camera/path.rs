use std::{collections::VecDeque, time::Duration};

use bevy::{ecs::{component::Component, system::Commands}, math::{cubic_splines::CubicCurve, Vec3}, prelude::*, reflect};

use crate::util::EasingFunction;
//==============================================================================
//         CameraPathFollower
//==============================================================================

pub struct CameraPathPlugin;

impl Plugin for CameraPathPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, follow_path)
        
            .register_type::<CameraPathFollower>()
        ;
    }
}

//==============================================================================
//         CameraPathFollower
//==============================================================================

#[derive(Component, Reflect)]
pub struct CameraPathFollower {
    path : VecDeque<CameraPathSegment>,
    timer : Timer,
    pub should_run : bool,
}

impl Default for CameraPathFollower {
    fn default() -> Self {
        Self {
            path : VecDeque::new(),
            timer : Timer::new(Duration::from_secs(0), TimerMode::Once),
            should_run : true,
        }
    }
}

#[derive(Component, Default, Reflect)]
pub struct CameraPathSegment {
    pub target_transform : Transform,
    #[reflect(ignore)]
    curve : CubicSegment<Vec2>,
    length : f32,
}

impl CameraPathFollower {
    pub fn to_transform(transform : Transform, duration : f32) -> Self {
        let mut cam_path = Self::default();
        cam_path.push_node(transform, duration, EasingFunction::EaseInOut);
        cam_path
    }
    
    pub fn push_node(&mut self, target : Transform, duration : f32, curve : impl Into<CubicSegment<Vec2>>) {
        if self.path.is_empty() {
            self.timer = Timer::new(Duration::from_secs_f32(duration), TimerMode::Once);
        }
        
        self.path.push_back(CameraPathSegment{
            target_transform : target,
            length : duration,
            curve : curve.into(),
        });
    }
    
    pub fn get_current_target(&self) -> &Transform {
        &self.path.front().unwrap().target_transform
    }
    
    pub fn get_current_curve(&self) -> &CubicSegment<Vec2> {
        &self.path.front().unwrap().curve
    }
    
    pub fn get_current_length(&self) -> f32 {
        self.path.front().unwrap().length
    }

    pub fn iter_transforms(&self) -> std::collections::vec_deque::Iter<'_, CameraPathSegment> {
        self.path.iter()
    }
}

fn follow_path (
    mut commands : Commands,
    mut cameras : Query<(Entity, &mut Transform, &mut CameraPathFollower)>,
    time : Res<Time>,
) {
    for (entity, mut camera_transform, mut path_follower) in cameras.iter_mut() {
        if !path_follower.should_run {
            continue;
        }
        
        let current_target = path_follower.get_current_target();
        let current_curve = path_follower.get_current_curve();
        let t = path_follower.timer.fraction();
        
        camera_transform.translation =  camera_transform.translation.lerp(current_target.translation, current_curve.position(t).y);
        camera_transform.rotation = camera_transform.rotation.slerp(current_target.rotation, current_curve.position(t).y);
        
        path_follower.timer.tick(time.delta().mul_f32(0.1));
        if path_follower.timer.finished() {
            path_follower.path.pop_front();
            
            if path_follower.path.is_empty() {
                commands.entity(entity).remove::<CameraPathFollower>();
            } else {
                path_follower.timer = Timer::new(Duration::from_secs_f32(path_follower.get_current_length()), TimerMode::Once);
            }
        }
    }
}