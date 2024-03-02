
use bevy::{asset::LoadState, prelude::*, scene::{InstanceId, SceneLoader}};
use bevy_debug_text_overlay::screen_print;

use crate::game::GameState;

//==============================================================================
//         LoadingTracker
//==============================================================================

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, check_for_loading_complete.run_if(in_state(GameState::Loading)))
            .add_systems(Update, check_for_scene_complete.run_if(in_state(GameState::PreparingScene)))
        
            .init_resource::<LoadingTracker>()
            .init_resource::<SceneTracker>()
        ;
    }
}

//==============================================================================
//         LoadingTracker
//==============================================================================

#[derive(Resource, Default)]
pub struct LoadingTracker {
    assets_to_load: Vec<UntypedHandle>,
}

impl LoadingTracker {
    pub fn push<A : Asset>(&mut self, value: Handle<A>) {
        self.assets_to_load.push(value.untyped());
    }
}

fn check_for_loading_complete(
    mut next_state : ResMut<NextState<GameState>>,
    loading_tracker: Res<LoadingTracker>,
    asset_server: Res<AssetServer>,
) {
    let number_left = loading_tracker.assets_to_load.iter().map(|handle| {
        if let Some(tracking) = asset_server.get_load_state(handle){
            match tracking {
                LoadState::Loaded => 0,
                LoadState::Loading => 1,
                LoadState::Failed => 1,
                LoadState::NotLoaded => 1,
            }
        } else {
            1
        }
    }).sum::<i32>();
    
    #[cfg(debug_assertions)]
    screen_print!(sec: 0.1, "Scene Progress: {}/{}", loading_tracker.assets_to_load.len() as i32 - number_left, loading_tracker.assets_to_load.len());
    
    if number_left == 0 {
        //Do something
        next_state.set(GameState::PreparingScene);
    }
}

//==============================================================================
//         SceneLoader
//==============================================================================

#[derive(Resource, Default)]
pub struct SceneTracker {
    instances: Vec<InstanceId>,
}

impl SceneTracker {
    pub fn push(&mut self, value: InstanceId) {
        self.instances.push(value);
    }
}

fn check_for_scene_complete (
    mut next_state : ResMut<NextState<GameState>>,
    scene_tracker: Res<SceneTracker>,
    scene_spawner: Res<SceneSpawner>,
) {
    let number_left = scene_tracker.instances.iter().map(|instance| {
        if scene_spawner.instance_is_ready(*instance) {
            0
        } else {
            1
        }
    }).sum::<i32>();
    
    #[cfg(debug_assertions)]
    screen_print!(sec: 0.1, "Scene Progress: {}/{}", scene_tracker.instances.len() as i32 - number_left, scene_tracker.instances.len());
    
    if number_left == 0 {
        next_state.set(GameState::Active);
    }
}
