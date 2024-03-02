use std::default;

use bevy::prelude::*;

//==============================================================================
//         Game Plugin
//==============================================================================

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .init_resource::<GameFlags>()
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
