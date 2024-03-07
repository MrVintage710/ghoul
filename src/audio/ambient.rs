use bevy::{audio::{PlaybackMode, Volume}, prelude::*};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumProperty};
use std::{ops::{Deref, DerefMut}, string::ToString, time::Duration};

use crate::util::{DelayedEvent, DelayedEventPlugin};

use super::AudioAssets;

//==============================================================================
//         AudioPlugin
//==============================================================================

pub struct AmbientAudioPlugin;

impl Plugin for AmbientAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DelayedEventPlugin::<AmbientAudioEvent>::default())
            
            .add_systems(Startup, init_ambient_audio.after(super::load_audio))
            .add_systems(PreUpdate, update_ambient_audio_events)
            .add_systems(PostUpdate, catch_ambient_audio_events)
        
            .register_type::<AmbientAudioEvent>()
            
            .add_event::<AmbientAudioEvent>()
        ;
    }
}

//==============================================================================
//         Ambient Audio Marker
//==============================================================================

#[derive(Debug, Component)]
pub struct AmbientAudio(AmbientAudioType);

impl AmbientAudio {
    fn is_type(&self, audio_type : &AmbientAudioType) -> bool {
        self.0 == *audio_type
    }
}

//==============================================================================
//         Ambient Audio Type
//==============================================================================

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect, EnumIter, strum_macros::Display)]
pub enum AmbientAudioType {
    Computer,
    Storm
}

//==============================================================================
//         Ambient Audio Fade In
//==============================================================================

#[derive(Clone, Event, Component, Reflect)]
pub enum AmbientAudioEvent {
    VolumeFade {
        audio_type : AmbientAudioType,
        timer : Timer,
        target_volume : f32,
        starting_volume : f32
    },
    FadeOut {
        audio_type : AmbientAudioType,
        timer : Timer,
        starting_volume : f32
    },
    FadeIn {
        audio_type : AmbientAudioType,
        timer : Timer,
        target_volume : f32
    },
    Play {
        audio_type : AmbientAudioType,
        volume : f32
    }
}

impl AmbientAudioEvent {
    pub fn volume_fade(audio_type : AmbientAudioType, seconds : f32, target_volume : f32) -> Self {
        Self::VolumeFade {
            audio_type,
            timer : Timer::new(Duration::from_secs_f32(seconds), TimerMode::Once),
            target_volume,
            starting_volume : 0.0
        }
    }
    
    pub fn fade_out(audio_type : AmbientAudioType, seconds : f32) -> Self {
        Self::FadeOut {
            audio_type,
            timer : Timer::new(Duration::from_secs_f32(seconds), TimerMode::Once),
            starting_volume : 0.0
        }
    }
    
    pub fn fade_in(audio_type : AmbientAudioType, seconds : f32, target_volume : f32) -> Self {
        Self::FadeIn {
            audio_type,
            timer : Timer::new(Duration::from_secs_f32(seconds), TimerMode::Once),
            target_volume
        }
    }
    
    pub fn play(audio_type : AmbientAudioType, volume : f32) -> Self {
        Self::Play {
            audio_type,
            volume
        }
    }
    
    pub fn ambient_type(&self) -> AmbientAudioType {
        match self {
            Self::VolumeFade { audio_type : event_type, .. } => *event_type,
            Self::FadeOut { audio_type : event_type, .. } => *event_type,
            Self::FadeIn { audio_type : event_type, .. } => *event_type,
            Self::Play { audio_type : event_type, .. } => *event_type
        }
    }
    
    pub fn set_starting_volume(&mut self, starting_volume : f32) {
        match self {
            Self::VolumeFade { starting_volume : volume, .. } => *volume = starting_volume,
            Self::FadeOut { starting_volume : volume, .. } => *volume = starting_volume,
            Self::FadeIn { .. } => {},
            Self::Play { .. } => {},
        }
    }
    
    pub fn get_starting_volume(&self) -> f32 {
        match self {
            Self::VolumeFade { starting_volume, .. } => *starting_volume,
            Self::FadeOut { starting_volume, .. } => *starting_volume,
            Self::FadeIn { .. } => 0.0,
            Self::Play { .. } => 0.0,
        }
    }
}

fn catch_ambient_audio_events(
    mut commands : Commands,
    mut ambient_audio_fade_in_events : EventReader<AmbientAudioEvent>,
    ambient_audio_sources : Query<(Entity, &AmbientAudio, &AudioSink)>,
) {
    for event in ambient_audio_fade_in_events.read() {
        let Some((audio_type_entity, _, playback)) = 
            ambient_audio_sources.iter().find(|(_, audio, _)| audio.is_type(&event.ambient_type()))
            else { continue };
        
        let mut event = event.clone();
        event.set_starting_volume(playback.volume());
        
        commands.entity(audio_type_entity).insert(event);
    }
}

fn update_ambient_audio_events(
    mut commands : Commands,
    mut ambient_audio_sources : Query<(Entity, &mut AmbientAudioEvent, &mut AudioSink), With<AmbientAudio>>,
    time : Res<Time>
) {
    for (source_entity, mut event, mut playback) in ambient_audio_sources.iter_mut() {
        match event.deref_mut() {
            AmbientAudioEvent::VolumeFade { timer, target_volume, starting_volume, .. } => {
                playback.play();
                let progress = timer.fraction();
                let current_volume = starting_volume.lerp(*target_volume, progress);
                playback.set_volume(current_volume);
                
                if timer.tick(time.delta()).just_finished() {
                    commands.entity(source_entity).remove::<AmbientAudioEvent>();
                }
            },
            AmbientAudioEvent::FadeOut { timer, starting_volume, .. } => {
                playback.play();
                let progress = timer.fraction();
                let current_volume = starting_volume.lerp(0.0, progress);
                playback.set_volume(current_volume);
                
                if timer.tick(time.delta()).just_finished() {
                    playback.pause();
                    commands.entity(source_entity).remove::<AmbientAudioEvent>();
                }
            },
            AmbientAudioEvent::FadeIn { timer, target_volume, .. } => {
                playback.play();
                let progress = timer.fraction();
                let current_volume = 0.0.lerp(*target_volume, progress);
                playback.set_volume(current_volume);
                
                if timer.tick(time.delta()).just_finished() {
                    commands.entity(source_entity).remove::<AmbientAudioEvent>();
                }
            }
            AmbientAudioEvent::Play { volume, .. } => {
                playback.play();
                playback.set_volume(*volume);
                commands.entity(source_entity).remove::<AmbientAudioEvent>();
            }
        }
    }
}

//==============================================================================
//         Ambient Audio
//==============================================================================

pub fn init_ambient_audio (
    mut commands : Commands,
    audio_assets: Res<AudioAssets>,
) {
    
    let assets = [&audio_assets.computer_ambient, &audio_assets.storm_ambient];
    
    for (index, ambient_aduio_type) in AmbientAudioType::iter().enumerate() {
        commands.spawn((
            AudioSourceBundle {
                source: assets[index].clone(),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Loop,
                    volume: Volume::new(0.0),
                    paused: true,
                    ..Default::default()
                },
                ..default()
            },
            AmbientAudio(ambient_aduio_type.clone()),
            Name::new(format!("{} Ambient", ambient_aduio_type.to_string()))
        ));
    }
}