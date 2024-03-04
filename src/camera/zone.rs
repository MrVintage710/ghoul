use bevy::{math::bounding::{Bounded3d, RayCast3d}, prelude::*, window::PrimaryWindow};

use crate::{camera::path::CameraPathFollower, game::ActiveCamera, scene::RoomCamera};

//==============================================================================
//         CameraZone Plugin
//==============================================================================

pub struct CameraZonePlugin;

impl Plugin for CameraZonePlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_systems(Update, detect_camera_zone_click)
            
            .register_type::<CameraZone>()
        ;
    }
}

//==============================================================================
//         CameraZone Component
//==============================================================================

#[derive(Component, Reflect, Default)]
pub struct CameraZone {
    pub bounds : Cuboid,
    pub target_transform : Transform,
}

impl CameraZone {
    pub fn new(bounds : Cuboid, target_transform : Transform) -> Self {
        Self { bounds, target_transform }
    }
}

//==============================================================================
//         CameraZoneBundle
//==============================================================================

#[derive(Default, Bundle)]
pub struct CameraZoneBundle {
    pub camera_zone : CameraZone,
    pub spatial_bundle : SpatialBundle,
}

impl CameraZoneBundle {
    pub fn new(camera_zone: CameraZone, spatial_bundle: SpatialBundle) -> Self {
        Self { camera_zone, spatial_bundle }
    }
}

//==============================================================================
//         CameraZone Systems
//==============================================================================

fn detect_camera_zone_click(
    mut commands : Commands,
    window : Query<&Window, With<PrimaryWindow>>,
    camera : Query<(Entity, &Camera, &GlobalTransform), (With<ActiveCamera>, With<RoomCamera>)>,
    camera_zone : Query<(&CameraZone, &GlobalTransform)>,
    mouse : Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let Ok(window) = window.get_single() else { return };
        let Ok((cam_entity, camera, cam_transform)) = camera.get_single() else { return };
        if let Some(pos) = window.cursor_position() {
            if let Some(ray) = camera.viewport_to_world(cam_transform, pos) {
                for (camera_zone, transform) in camera_zone.iter() {
                    let (_, rotation, translation) = transform.to_scale_rotation_translation();
                    let ray_cast = RayCast3d::from_ray(ray, 10.0);
                    let aabb = camera_zone.bounds.aabb_3d(translation, rotation);
                    
                    if let Some(hit) = ray_cast.aabb_intersection_at(&aabb) {
                        println!("Hit camera zone: {:?}", hit);
                        
                        commands.entity(cam_entity).insert(CameraPathFollower::to_transform(camera_zone.target_transform, 1.0));
                    }
                }
            }
        }
    }
}