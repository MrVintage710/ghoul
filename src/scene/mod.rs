mod computer_world;

use bevy::{audio::{PlaybackMode, Volume}, gltf::Gltf, prelude::*, render::view::RenderLayers, scene::InstanceId};
use crate::{game::GameState, loading::{LoadingTracker, SceneTracker}};

//==============================================================================
//         Scene Plugin
//==============================================================================

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
        
            .add_systems(Startup, load_scene)
            .add_systems(OnEnter(GameState::PreparingScene), start_room_initialization)
            .add_systems(OnEnter(GameState::Active), setup_room_scene_when_finished)
        ;
    }
}

//==============================================================================
//         Scene Loading
//==============================================================================

#[derive(Resource)]
pub struct RoomSceneAssets{
    //Models
    room_model : Handle<Scene>,
    
    //Audio
    ambient_storm : Handle<AudioSource>,
    thunder : Handle<AudioSource>,
    
    //Scene ID
    room_scene : Option<InstanceId>
}

fn load_scene(
    mut commands : Commands,
    mut loading_tracker: ResMut<LoadingTracker>,
    asset_server: Res<AssetServer>,
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
    ));
    // Setup for the loading stage
    let room_handle = asset_server.load("models/ghoul_room.glb#Scene0");
    let ambient_storm_handle = asset_server.load("audio/ambient_rain.ogg");
    let thunder_handle = asset_server.load("audio/thunder.ogg");
    commands.insert_resource(RoomSceneAssets{
        room_model: room_handle.clone(),
        ambient_storm: ambient_storm_handle.clone(),
        thunder: thunder_handle.clone(),
        room_scene: None
    });
    loading_tracker.push(room_handle);
    loading_tracker.push(ambient_storm_handle);
    loading_tracker.push(thunder_handle);
}

//==============================================================================
//         Prepare Scene systems
//==============================================================================

fn start_room_initialization(
    mut commands : Commands,
    mut room_assets : ResMut<RoomSceneAssets>,
    mut scene_spawner : ResMut<SceneSpawner>,
    mut scene_tracker : ResMut<SceneTracker>,
) {    
    let scene_instance = scene_spawner.spawn(room_assets.room_model.clone());
    room_assets.room_scene = Some(scene_instance);
    scene_tracker.push(scene_instance);
    
    commands.spawn((
        PointLightBundle {
            transform: Transform::from_xyz(1.3, 1.2, 0.3),
            point_light: PointLight {
                intensity: 50_000.0,
                range: 3.0,
                ..default()
            },
            ..default()
        },
        Name::new("Room Light")
    ));
    
    commands.spawn((
        AudioSourceBundle {
            source: room_assets.ambient_storm.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(0.2),
                ..Default::default()
            },
            ..default()
        },
        Name::new("Ambient Storm")
    ));
    
    commands.spawn((
        AudioSourceBundle {
            source: room_assets.thunder.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                volume: Volume::new(0.3),
                ..Default::default()
            },
            ..default()
        },
        Name::new("Lightning")
    ));
}

fn setup_room_scene_when_finished(
    mut commands : Commands,
    room_scene_assets : Res<RoomSceneAssets>,
    scene_spawner : ResMut<SceneSpawner>,
) {
    let Some(instance) = room_scene_assets.room_scene else { return };
    
    if scene_spawner.instance_is_ready(instance) {
        for entity in scene_spawner.iter_instance_entities(instance) {
            commands.entity(entity).insert(RenderLayers::layer(0));
        }
    }
}