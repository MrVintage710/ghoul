use std::default;

use bevy::{math::bounding::{Bounded3d, RayCast3d}, prelude::*, window::PrimaryWindow};

use crate::{audio::ambient::{AmbientAudioEvent, AmbientAudioType}, camera::path::CameraPathFollower, game::{ActiveCamera, ToggleGameWorldEvent}, scene::RoomCamera, util::{DelayedEvent, DelayedEventPlugin}};

use super::blackout::BlackoutTransition;

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

#[derive(Debug, Event, Reflect)]
pub enum CameraZoneAction {
    Move(Transform),
    Computer
}

//==============================================================================
//         CameraZone Component
//==============================================================================

#[derive(Component, Reflect)]
pub struct CameraZone {
    pub bounds : Cuboid,
    pub event : CameraZoneAction,
}

impl CameraZone {
    pub fn new(bounds: Cuboid, event: CameraZoneAction) -> Self {
        Self { bounds, event }
    }

    pub fn new_movement_zone(bounds : Cuboid, target_transform : Transform) -> Self {
        Self { bounds, event : CameraZoneAction::Move(target_transform) }
    }
}

//==============================================================================
//         CameraZoneBundle
//==============================================================================

#[derive(Bundle)]
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
    mut camera : Query<(Entity, &Camera, &GlobalTransform, &mut CurrentZone), With<ActiveCamera>>,
    mut fade_event : EventWriter<BlackoutTransition>,
    mut ambient_event : EventWriter<AmbientAudioEvent>,
    mut delayed_world_event : EventWriter<DelayedEvent<ToggleGameWorldEvent>>,
    window : Query<&Window, With<PrimaryWindow>>,
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
                       match camera_zone.event {
                           CameraZoneAction::Move(target_transform) => {
                               commands.entity(cam_entity).insert(CameraPathFollower::to_transform(target_transform, 1.0));
                               *current_zone = CurrentZone(zone_entity);
                           },
                           CameraZoneAction::Computer => {
                               commands.entity(cam_entity).insert(CameraPathFollower::to_transform(
                                   Transform::from_xyz(0.191, 1.078, 1.587)
                                       .with_rotation(Quat::from_euler(EulerRot::XYZ, 3.127, -0.021, 3.141)), 
                                   1.0
                               ));
                               fade_event.send(BlackoutTransition::fade_out(1.0));
                               ambient_event.send(AmbientAudioEvent::fade_out(AmbientAudioType::Storm, 1.0));
                               delayed_world_event.send(DelayedEvent::new(ToggleGameWorldEvent, 1.2));
                           }
                       }
                    }
                }
            }
        }
    }
}

fn detect_back_up(
    mut commands : Commands,
    mut camera : Query<(Entity, &mut CurrentZone), With<ActiveCamera>>,
    camera_zones : Query<(Entity, &CameraZone, Option<&Parent>)>,
    key_input : Res<ButtonInput<KeyCode>>,
) {
    if key_input.just_pressed(KeyCode::Escape) || key_input.just_pressed(KeyCode::Backspace){
        let Ok((cam_entity, mut current_zone)) = camera.get_single_mut() else { return };
        let Ok((zone_entity, _, parent)) = camera_zones.get(current_zone.0) else { return };
        
        if let Some(parent) = parent {
            let Ok((_, camera_zone, _)) = camera_zones.get(**parent) else { return };
            match camera_zone.event {
                CameraZoneAction::Move(target_transform) => {
                    commands.entity(cam_entity).insert(CameraPathFollower::to_transform(target_transform, 1.0));
                    *current_zone = CurrentZone(zone_entity);
                },
                _ => {}
            }
        }
    }
}