pub mod computer_world;

use bevy::{audio::{PlaybackMode, Volume}, gltf::Gltf, prelude::*, render::{camera::RenderTarget, view::RenderLayers}, scene::InstanceId};
use bevy_inspector_egui::bevy_egui::setup_new_windows_system;
use crate::{audio::{ambient::{AmbientAudioEvent, AmbientAudioType}, sound::PlaySoundEvent, AudioAssets}, camera::blackout::BlackoutTransition, game::{ActiveCamera, GameState}, loading::{LoadingTracker, SceneTracker}};

use self::computer_world::{ComputerCamera, ComputerWorldAssets, ComputerWorldPlugin};

//==============================================================================
//         Scene Plugin
//==============================================================================

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ComputerWorldPlugin)
        
            .add_systems(Startup, load_scene)
            .add_systems(OnEnter(GameState::PreparingScene), start_room_initialization)
            .add_systems(OnEnter(GameState::Active), setup_room_scene_when_finished)
            // .add_systems(PostUpdate, switch_to_room_world)
        
            // .add_event::<SwitchToRoom>()
        ;
    }
}

//==============================================================================
//         Labels
//==============================================================================

#[derive(Debug, Component)]
pub struct RoomCamera;

//==============================================================================
//         Scene Loading
//==============================================================================

#[derive(Resource)]
pub struct RoomSceneAssets{
    //Models
    room_model : Handle<Scene>,
    
    //Scene ID
    room_scene : Option<InstanceId>
}

fn load_scene(
    mut commands : Commands,
    mut loading_tracker: ResMut<LoadingTracker>,
    asset_server: Res<AssetServer>,
) {
    
    // Setup for the loading stage
    let room_handle = asset_server.load("models/ghoul_room.glb#Scene0");
    commands.insert_resource(RoomSceneAssets{
        room_model: room_handle.clone(),
        room_scene: None
    });
    loading_tracker.push(room_handle);
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
    
    // commands.spawn((
    //     AudioSourceBundle {
    //         source: room_assets.ambient_storm.clone(),
    //         settings: PlaybackSettings {
    //             mode: PlaybackMode::Loop,
    //             volume: Volume::new(0.2),
    //             ..Default::default()
    //         },
    //         ..default()
    //     },
    //     Name::new("Ambient Storm")
    // ));
    
    // commands.spawn((
    //     AudioSourceBundle {
    //         source: room_assets.thunder.clone(),
    //         settings: PlaybackSettings {
    //             mode: PlaybackMode::Once,
    //             volume: Volume::new(0.3),
    //             ..Default::default()
    //         },
    //         ..default()
    //     },
    //     Name::new("Lightning")
    // ));
}

fn setup_room_scene_when_finished(
    mut commands : Commands,
    mut fade_in_event : EventWriter<BlackoutTransition>,
    mut ambient_audio_event : EventWriter<AmbientAudioEvent>,
    mut sound_effect_event : EventWriter<PlaySoundEvent>,
    room_scene_assets : Res<RoomSceneAssets>,
    scene_spawner : ResMut<SceneSpawner>,
    named_assets : Query<(&Name, &Parent)>,
    game_world_assets : Res<ComputerWorldAssets>,
    sounds : Res<AudioAssets>
) {
    fade_in_event.send(BlackoutTransition::fade_in(1.0));
    ambient_audio_event.send(AmbientAudioEvent::fade_in(AmbientAudioType::Storm, 1.0, 0.2));
    // sound_effect_event.send(PlaySoundEvent::new(sounds.storm_thunder.clone(), 0.3, None));
    
    let Some(instance) = room_scene_assets.room_scene else { return };
    
    if scene_spawner.instance_is_ready(instance) {
        for entity in scene_spawner.iter_instance_entities(instance) {
            commands.entity(entity).insert(RenderLayers::layer(0));
            
            if let Ok((name, parent)) = named_assets.get(entity) {
                if &**name == "Mesh" {
                    if let Ok((parrent_name, _)) = named_assets.get(**parent) {
                        if &**parrent_name == "Screen" {
                            commands.entity(entity).insert(game_world_assets.render_surface_mat.clone());
                        }
                    }
                }
            }
        }
    }
}

//==============================================================================
//         
//==============================================================================
