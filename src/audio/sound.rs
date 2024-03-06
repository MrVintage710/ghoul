use bevy::{audio::{PlaybackMode, Volume}, prelude::*};

use crate::util::DelayedEventPlugin;

//==============================================================================
//         SoundPlugin
//==============================================================================

pub struct SoundEffectPlugin;

impl Plugin for SoundEffectPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DelayedEventPlugin::<PlaySoundEvent>::default())
        
            .add_systems(PreUpdate, update_sound_effects)
            .add_systems(PostUpdate, catch_play_sound_events)
            
            .add_event::<PlaySoundEvent>()
            .add_event::<SoundFinishedEvent>()
        ;
        
    }
}

//==============================================================================
//         SoundEffect Component
//==============================================================================

#[derive(Component)]
pub struct SoundEffect;

//==============================================================================
//         PlaySoundEvent
//==============================================================================

#[derive(Debug, Event, Clone)]
pub struct PlaySoundEvent {
    pub sound : Handle<AudioSource>,
    pub volume : f32,
    pub position : Option<Vec3>,
}

impl PlaySoundEvent {
    pub fn new(sound: Handle<AudioSource>, volume: f32, position: Option<Vec3>) -> Self {
        Self { sound, volume, position }
    }
}

#[derive(Debug, Event, Clone)]
pub struct SoundFinishedEvent(Handle<AudioSource>);

//==============================================================================
//         PlaySound Systems
//==============================================================================

pub fn catch_play_sound_events (
    mut commands : Commands,
    mut events : EventReader<PlaySoundEvent>,
) {
    for event in events.read() {
        let spatial = event.position.is_some();
        commands.spawn((
            AudioSourceBundle {
                source: event.sound.clone(),
                settings: PlaybackSettings {
                    mode: PlaybackMode::Once,
                    volume: Volume::new(event.volume),
                    spatial,
                    ..Default::default()
                },
                ..default()
            },
            SoundEffect,
            Name::new("Sound Effect")
        ));
    }
}

pub fn update_sound_effects(
    mut commands : Commands,
    sound_effects : Query<(Entity, Option<&AudioSink>, Option<&SpatialAudioSink>), With<SoundEffect>>,
) {
    for (entity, sink, spacial_sink) in sound_effects.iter() {
        if let Some(sink) = sink {
            if sink.empty() {
                commands.entity(entity).despawn();
            }
        }
        
        if let Some(spacial_sink) = spacial_sink {
            if spacial_sink.empty() {
                commands.entity(entity).despawn();
            }
        }
    }
}