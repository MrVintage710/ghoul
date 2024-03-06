pub mod ambient;
pub mod sound;

use bevy::prelude::*;

use crate::loading::LoadingTracker;

use self::{ambient::AmbientAudioPlugin, sound::SoundEffectPlugin};

//==============================================================================
//         AudioPlugin
//==============================================================================

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AmbientAudioPlugin)
            .add_plugins(SoundEffectPlugin)
        
            .add_systems(Startup, load_audio)
        ;
    }
}

//==============================================================================
//         Audio init loading
//==============================================================================

#[derive(Debug, Clone, Eq, PartialEq, Hash, Resource)]
pub struct AudioAssets {
    pub storm_ambient : Handle<AudioSource>,
    pub storm_thunder : Handle<AudioSource>,
    pub computer_ambient : Handle<AudioSource>,
    pub computer_rev : Handle<AudioSource>,
    pub computer_medium_write : Handle<AudioSource>,
    pub computer_light_write : Handle<AudioSource>,
}

pub fn load_audio (
    mut commands : Commands,
    mut loading_tracker: ResMut<LoadingTracker>,
    asset_server: Res<AssetServer>,
) {
    let storm_ambient = asset_server.load("audio/storm_ambient.ogg");
    let storm_thunder = asset_server.load("audio/storm_thunder.ogg");
    
    let computer_ambient = asset_server.load("audio/computer_ambient.ogg");
    let computer_rev = asset_server.load("audio/computer_rev.ogg");
    let computer_medium_write = asset_server.load("audio/computer_medium_write.ogg");
    let computer_light_write = asset_server.load("audio/computer_light_write.ogg");
    
    loading_tracker.push(storm_ambient.clone());
    loading_tracker.push(storm_thunder.clone());
    
    loading_tracker.push(computer_ambient.clone());
    loading_tracker.push(computer_rev.clone());
    loading_tracker.push(computer_medium_write.clone());
    loading_tracker.push(computer_light_write.clone());
    
    let audio_assets = AudioAssets {
        storm_ambient,
        storm_thunder,
        computer_ambient,
        computer_rev,
        computer_medium_write,
        computer_light_write,
    };
    
    commands.insert_resource(audio_assets);
}