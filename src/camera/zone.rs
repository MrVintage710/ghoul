use bevy::{math::bounding::{Bounded3d, RayCast3d}, prelude::*, window::PrimaryWindow};

use crate::{camera::path::CameraPathFollower, game::ActiveCamera, scene::RoomCamera};

//==============================================================================
//         CameraZone Plugin
//==============================================================================

pub struct CameraZonePlugin;

impl Plugin for CameraZonePlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_systems(Update, (detect_camera_zone_click, detect_back_up))
            
            .register_type::<CameraZone>()
        ;
    }
}

//==============================================================================
//         CameraZone Type
//==============================================================================

pub enum CameraZoneType {
    Move(Transform),
    Event()
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
//         Available Zones
//==============================================================================

#[derive(Debug, Component)]
pub struct CurrentZone(pub Entity);

//==============================================================================
//         CameraZone Systems
//==============================================================================

fn detect_camera_zone_click(
    mut commands : Commands,
    window : Query<&Window, With<PrimaryWindow>>,
    mut camera : Query<(Entity, &Camera, &GlobalTransform, &mut CurrentZone), With<ActiveCamera>>,
    camera_zones : Query<(Entity, &CameraZone, &Transform, Option<&Children>)>,
    mouse : Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let Ok(window) = window.get_single() else { return };
        let Ok((cam_entity, camera, cam_transform, mut current_zone)) = camera.get_single_mut() else { return };
        if let Some(pos) = window.cursor_position() {
            if let Some(ray) = camera.viewport_to_world(cam_transform, pos) {
                let Ok((_, _, _, children)) = camera_zones.get(current_zone.0) else { return };
                let Some(children) = children else { return };
                
                for child in children {
                    let Ok((zone_entity, camera_zone, transform, _)) = camera_zones.get(*child) else { return };
                    
                    // let (_, rotation, translation) = transform.to_scale_rotation_translation();
                    let ray_cast = RayCast3d::from_ray(ray, 10.0);
                    let aabb = camera_zone.bounds.aabb_3d(transform.translation, transform.rotation);
                    
                    if let Some(_) = ray_cast.aabb_intersection_at(&aabb) {        
                        commands.entity(cam_entity).insert(CameraPathFollower::to_transform(camera_zone.target_transform, 1.0));
                        *current_zone = CurrentZone(zone_entity);
                    }
                }
            }
        }
    }
}

fn detect_back_up(
    mut commands : Commands,
    mut camera : Query<(Entity, &Camera, &GlobalTransform, &mut CurrentZone), With<ActiveCamera>>,
    camera_zones : Query<(Entity, &CameraZone, &Transform, Option<&Parent>)>,
    key_input : Res<ButtonInput<KeyCode>>,
) {
    if key_input.just_pressed(KeyCode::Escape) || key_input.just_pressed(KeyCode::Backspace){
        let Ok((cam_entity, camera, cam_transform, mut current_zone)) = camera.get_single_mut() else { return };
        let Ok((zone_entity, camera_zone, transform, parent)) = camera_zones.get(current_zone.0) else { return };
        
        if let Some(parent) = parent {
            let Ok((_, camera_zone, _, parent_transform)) = camera_zones.get(**parent) else { return };
            commands.entity(cam_entity).insert(CameraPathFollower::to_transform(camera_zone.target_transform, 1.0));
            *current_zone = CurrentZone(**parent);
        }
    }
}