use bevy::{prelude::*, render::view::RenderLayers};
use camera::CameraPlugin;
use game::{ActiveCamera, GamePlugin};
use loading::LoadingPlugin;
use scene::{RoomCamera, ScenePlugin};

#[cfg(debug_assertions)]
mod debug;

mod scene;
mod loading;
mod game;

pub mod camera;

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
    // Load the essientials first
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
    ));
}
