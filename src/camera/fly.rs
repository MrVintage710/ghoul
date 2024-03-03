
use bevy::{input::mouse::MouseMotion, prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use bevy_debug_text_overlay::screen_print;

use crate::game::ActiveCamera;

//==============================================================================
//         FlyCam Plugin
//==============================================================================

pub struct FlyCamPlugin;

impl Plugin for FlyCamPlugin {
    fn build(&self, app: &mut App) {
        app
        
            .add_systems(Update, flycam_control)
            .add_systems(PostUpdate, toggle_fly_cam)
            
            .add_event::<ToggleFlyCam>()
        ;
    }
}

//==============================================================================
//         Toggle FlyCam Event
//==============================================================================

#[derive(Debug, Clone, Copy, Event)]
pub struct ToggleFlyCam;

//==============================================================================
//         FlyCam Component
//==============================================================================

#[derive(Component)]
pub struct FlyCam {
    start_transform : Transform,
    pitch : f32,
    yaw : f32,
}

//==============================================================================
//         FlyCam Systems
//==============================================================================

fn toggle_fly_cam(
    mut commands : Commands,
    mut current_cam : Query<(Entity, &mut Transform, Option<&mut FlyCam>), With<ActiveCamera>>,
    mut event_reader : EventReader<ToggleFlyCam>,
) {
    if !event_reader.is_empty() {
        let Ok((entity, mut transform,  flycam)) = current_cam.get_single_mut() else { return };
        
        if let Some(flycam) = flycam {
            *transform = flycam.start_transform;
            commands.entity(entity).remove::<FlyCam>();
        } else {
            let rot = transform.rotation.to_euler(EulerRot::XYZ);
            
            commands.entity(entity).insert(FlyCam {
                start_transform : transform.clone(),
                pitch : rot.0,
                yaw : rot.2,
            });
        }
    }
    
    event_reader.read();
}

fn flycam_control(
    mut flycams : Query<(&mut FlyCam, &mut Transform)>,
    mut window : Query<&mut Window, With<PrimaryWindow>>,
    mut mouse_motion : EventReader<MouseMotion>,
    mouse_input : Res<ButtonInput<MouseButton>>,
    key_input : Res<ButtonInput<KeyCode>>,
    time : Res<Time>,
) {
    for (mut flycam, mut transform) in flycams.iter_mut() {
        screen_print!("FlyCam Active");
    
        if mouse_input.pressed(MouseButton::Left) {
            let mut window = window.single_mut();
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
            
            for event in mouse_motion.read() {
                flycam.yaw -= event.delta.x * 0.005;
                flycam.pitch -= event.delta.y * 0.005;
                flycam.pitch = flycam.pitch.clamp(-1.5, 1.5);
            }
        } else {
            let mut window = window.single_mut();
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
        
        let mut new_pos = transform.translation;
        
        if key_input.pressed(KeyCode::KeyW) {
            new_pos += transform.forward() * 0.5 * time.delta_seconds();
        }
        
        if key_input.pressed(KeyCode::KeyS) {
            new_pos -= transform.forward() * 0.5 * time.delta_seconds();
        }
        
        if key_input.pressed(KeyCode::KeyA) {
            new_pos -= transform.right() * 0.5 * time.delta_seconds();
        }
        
        if key_input.pressed(KeyCode::KeyD) {
            new_pos += transform.right() * 0.5 * time.delta_seconds();
        }
        
        if key_input.pressed(KeyCode::Space) {
            new_pos += Direction3d::Y * 0.5 * time.delta_seconds();
        }
        
        if key_input.pressed(KeyCode::ShiftLeft) {
            new_pos -= Direction3d::Y * 0.5 * time.delta_seconds();
        }
        
        let pitch = Quat::from_rotation_x(flycam.pitch);
        let yaw = Quat::from_rotation_y(flycam.yaw);
        
        transform.translation = new_pos;
        transform.rotation = yaw * pitch;
    }
}
