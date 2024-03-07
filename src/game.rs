use std::default;

use bevy::{prelude::*, render::camera::RenderTarget, window::WindowRef};

use crate::{audio::{sound::PlaySoundEvent, AudioAssets}, scene::{computer_world::{ComputerCamera, ComputerWorldAssets}, RoomCamera}, util::DelayedEventPlugin};

//==============================================================================
//         Game Plugin
//==============================================================================

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DelayedEventPlugin::<ToggleGameWorldEvent>::default())
            .add_plugins(DelayedEventPlugin::<OnGameWorldChangeEvent>::default())
            
            .add_systems(PostUpdate, switch_game_world)
            
            .init_resource::<GameFlags>()
            .init_resource::<CurrentGameWorld>()
            
            .init_state::<GameState>()
        
            .add_event::<ToggleGameWorldEvent>()
            .add_event::<OnGameWorldChangeEvent>()
        ;
    }
}

//==============================================================================
//         GameState
//==============================================================================

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    PreparingScene,
    Active,
    Paused,
}

//==============================================================================
//         GameFlags
//==============================================================================

#[derive(Resource, Default)]
pub struct GameFlags {}

//==============================================================================
//         Active Camera
//==============================================================================

#[derive(Component)]
pub struct ActiveCamera;

//==============================================================================
//         Current GameWorld
//==============================================================================

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameWorld {
    Computer,
    #[default]
    Room
}

#[derive(Resource, Default)]
pub struct CurrentGameWorld(GameWorld);

impl CurrentGameWorld {
    pub fn toggle(&mut self) {
        self.0 = match self.0 {
            GameWorld::Computer => GameWorld::Room,
            GameWorld::Room => GameWorld::Computer,
        }
    }
}

#[derive(Event, Debug, Clone, Copy)]
pub struct ToggleGameWorldEvent;

pub fn switch_game_world(
    mut commands : Commands,
    mut computer_world_camera : Query<(Entity, &mut Camera, &mut Projection), (With<ComputerCamera>, Without<RoomCamera>)>,
    mut room_camera : Query<(Entity, &mut Camera), (With<RoomCamera>, Without<ComputerCamera>)>,
    mut current_game_world : ResMut<CurrentGameWorld>,
    mut events : EventReader<ToggleGameWorldEvent>,
    mut on_game_world_change_events : EventWriter<OnGameWorldChangeEvent>,
    computer_world_assets : Res<ComputerWorldAssets>,
    input : Res<ButtonInput<KeyCode>>,
) {
    
    if !events.is_empty() || input.just_pressed(KeyCode::F1) {
        println!("Event Recieved");
        let Ok((comp_cam_entity, mut comp_cam, mut comp_projection)) = computer_world_camera.get_single_mut() else { return };
        let Ok((room_cam_entity, mut room_cam)) = room_camera.get_single_mut() else { return };
        
        match current_game_world.0 {
            GameWorld::Room => {
                comp_cam.target = RenderTarget::Window(WindowRef::Primary);
                *comp_projection = Projection::default();
                
                room_cam.is_active = false;
                commands.entity(room_cam_entity).remove::<ActiveCamera>();
                commands.entity(comp_cam_entity).insert(ActiveCamera);
                on_game_world_change_events.send(OnGameWorldChangeEvent(GameWorld::Computer));
            },
            GameWorld::Computer => {
                comp_cam.target = RenderTarget::Image(computer_world_assets.render_surface_image.clone());
                *comp_projection = Projection::default();
                
                room_cam.is_active = true;
                commands.entity(comp_cam_entity).remove::<ActiveCamera>();
                commands.entity(room_cam_entity).insert(ActiveCamera);
                on_game_world_change_events.send(OnGameWorldChangeEvent(GameWorld::Room));
            },
        }
        
        current_game_world.toggle();
    }
    
    events.clear();
}

//==============================================================================
//         OnGameWorldChange
//==============================================================================

#[derive(Event, Clone)]
pub struct OnGameWorldChangeEvent(pub GameWorld);