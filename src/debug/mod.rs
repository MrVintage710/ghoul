use bevy::{diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, prelude::*, window::close_on_esc};
use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{camera::{fly::ToggleFlyCam, path::CameraPathFollower, zone::CameraZone}, game::GameState};

//==============================================================================
//         Debug Plugin
//==============================================================================

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                WorldInspectorPlugin::default(),
                OverlayPlugin { font_size: 18.0, ..default() },
                FrameTimeDiagnosticsPlugin,
            ))
            
            .add_systems(Update, (close_on_esc, display_debug_info, toggle_debug_mode, debug_camera_paths, debug_camera_zones))
        
            .init_resource::<DebugMode>()
        ;
    }
}

//==============================================================================
//         Debug Mode
//==============================================================================

#[derive(Resource)]
pub struct DebugMode(bool);

impl Default for DebugMode {
    fn default() -> Self {
        Self(true)
    }
}

impl DebugMode {
    pub fn toggle(&mut self) {
        self.0 = !self.0;
    }
    
    pub fn is_enabled(&self) -> bool {
        self.0
    }
}

//==============================================================================
//         Debug Systems
//==============================================================================

fn display_debug_info(
    diagnostics: Res<DiagnosticsStore>,
    debug_mode: Res<DebugMode>,
    game_state: Res<State<GameState>>,
) {
    if debug_mode.is_enabled() {
        if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps) = fps_diagnostic.value() {
                screen_print!(sec: 0.1, r"FPS: {:.2}", fps);
            }
        }
        
        screen_print!(sec: 0.1, "Game State: {:?}", game_state);
    }
}

fn toggle_debug_mode(
    mut debug_mode: ResMut<DebugMode>,
    mut fly_cam_event: EventWriter<ToggleFlyCam>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F3) {
        debug_mode.toggle();
    }
    
    if input.just_pressed(KeyCode::F4) {
        fly_cam_event.send(ToggleFlyCam);
    }
}

fn debug_camera_paths(
    mut gizmos : Gizmos,
    cameras : Query<(&Transform, &CameraPathFollower)>,
) {
    for (transform, path_follower) in cameras.iter() {
        let positions = path_follower.iter_transforms().map(|t| t.target_transform.translation).collect::<Vec<_>>();
        gizmos.linestrip(positions, Color::WHITE);
        gizmos.sphere(transform.translation, transform.rotation, 0.05, Color::AZURE);
    }
}

fn debug_camera_zones (
    mut gizmos: Gizmos,
    camera_zones: Query<(&Transform, &CameraZone)>,
) {
    for (transform, zone) in camera_zones.iter() {
        gizmos.primitive_3d(zone.bounds, transform.translation, transform.rotation, Color::WHITE);
    }
}