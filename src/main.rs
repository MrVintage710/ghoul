use bevy::{prelude::*, render::view::RenderLayers};
use camera::{path::CameraPathFollower, zone::{CameraZone, CameraZoneBundle, CurrentZone}, CameraPlugin};
use game::{ActiveCamera, GamePlugin};
use loading::LoadingPlugin;
use scene::{RoomCamera, ScenePlugin};
use util::EasingFunction;

#[cfg(debug_assertions)]
mod debug;

mod scene;
mod loading;
mod game;

pub mod camera;
pub mod anim;
pub mod util;

fn main() {
    
    let mut app = App::new();
    
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(ScenePlugin)
        .add_plugins(GamePlugin)
        .add_plugins(LoadingPlugin)
        .add_plugins(CameraPlugin)
    
        .add_systems(Startup, initialize_essentials)
    ;
    
    
    //This is for running the game in a debug mode
    #[cfg(debug_assertions)]
    {
        app.add_plugins(debug::DebugPlugin);
    }
    
    app.run();
}

fn initialize_essentials(
    mut commands : Commands
) {
    let mut follower = CameraPathFollower::default();
    
    follower.push_node(Transform::from_xyz(-0.0, 1.0, 0.841).with_rotation(Quat::from_euler(EulerRot::XYZ, -3.072, -0.02, -3.1403)), 1.0, EasingFunction::EaseInOut);
    
    let computer_node = commands.spawn(CameraZoneBundle {
        camera_zone: CameraZone::new( Cuboid::new(0.58, 0.58, 0.42) , camera::zone::CameraZoneAction::Computer),
        spatial_bundle : SpatialBundle {
            transform: Transform::from_xyz(0.18, 1.09, 2.0),
            ..Default::default()
        }
    }).id();
    
    // Load the essientials first
    let desk_node = commands.spawn(CameraZoneBundle {
        camera_zone: CameraZone::new_movement_zone(Cuboid::default(), Transform::from_xyz(-0.0, 1.0, 0.841).with_rotation(Quat::from_euler(EulerRot::XYZ, -3.072, -0.02, -3.1403))),
        spatial_bundle : SpatialBundle {
            transform: Transform::from_xyz(0.0, 1.0, 2.0),
            ..Default::default()
        }
    }).add_child(computer_node).id();
    
    let root = commands.spawn((
        CameraZoneBundle {
            camera_zone: CameraZone::new_movement_zone(Cuboid::default(), Transform::from_xyz(-1.0, 1.0, -1.0).with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 3.8, 0.0))),
            spatial_bundle : SpatialBundle {
                transform: Transform::from_xyz(-1.0, 1.0, -1.0).with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 3.8, 0.0)),
                ..Default::default()
            },
        },
        Name::new("Root Camera Zone")
    )).add_child(desk_node).id();
    
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-1.0, 1.0, -1.0).with_rotation(Quat::from_euler(EulerRot::XYZ, 0.0, 3.8, 0.0)),
            projection : Projection::Perspective(PerspectiveProjection { 
                fov: 1.0, 
                ..Default::default()
            }),
            ..default()
        },
        RenderLayers::layer(0),
        RoomCamera,
        ActiveCamera,
        CurrentZone(root)
    ));
}
