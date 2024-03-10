use std::time::Duration;

use bevy::prelude::*;

use bevy_ascii::prelude::*;
use bevy_debug_text_overlay::screen_print;

use crate::{audio::{ambient::{AmbientAudioEvent, AmbientAudioType}, sound::PlaySoundEvent, AudioAssets}, game::OnGameWorldChangeEvent, scene::computer_world::ComputerState, util::DelayedEvent};

use super::terminal::TerminalComponent;

//==============================================================================
//         loading Screen components
//==============================================================================

#[derive(Component)]
pub struct LoadingScreenComponent {
    progress: Timer,
    is_loading: bool,
}

impl Default for LoadingScreenComponent {
    fn default() -> Self {
        Self {
            progress: Timer::from_seconds(8.0, TimerMode::Once),
            is_loading: false,
        }
    }
}

impl AsciiComponent for LoadingScreenComponent {
    type UpdateQuery<'w, 's> = ();
    
    fn render(&self, buffer: &mut AsciiBuffer) {
        let elapsed = self.progress.elapsed_secs();
        
        if elapsed <= 2.5 {
            let text = match elapsed {
                ..=0.8 => "Initializing boot...",
                0.8..=1.5 => "Initializing boot...\nLoading G64 unified Os...",
                1.5..=1.8 => "Initializing boot...\nLoading G64 unified Os...\n64MB RAM detected...",
                1.8..=2.0 => "Initializing boot...\nLoading G64 unified Os...\n64MB RAM detected...\nG64 Power+ APU detected...",
                2.0.. => "Initializing boot...\nLoading G64 unified Os...\n64MB RAM detected...\nG64 Power+ APU detected...\nLoading Interface...",
                _ => ""
            };
            
            buffer.padding((1, 1, 1, 1)).text(text).draw();
        } else {
            let progress = ((elapsed - 2.5) / 0.5).min(1.0);
            
            let width = (progress * 22.0).round() as i32;
            let height = 10;
            
            let inner = buffer.center(width, height).square().border(BorderType::Full).draw();
            
            if progress >= 1.0 {
               if let Some(inner) = inner {
                   inner.text("G64+ Pro").horizontal_alignment(HorizontalAlignment::Center).vertical_alignment(VerticalAlignment::Center).draw();
               }
            }
        }
        
      //   ____  __   _  _   
      //  / ___|/ /_ | || |  
      // | |  _| '_ \| || |_ 
      // | |_| | (_) |__   _|
      //  \____|\___/   |_| 
        
        
        
    }

    fn set_up(app: &mut App) {
        app
            .add_systems(PostUpdate, begin_startup)
            .add_systems(Update, update_loading_screen)
        ;
    }
}

fn begin_startup(
    mut loading_screens: Query<(&mut LoadingScreenComponent, &mut Visibility)>,
    mut events : EventReader<OnGameWorldChangeEvent>,
    mut sound_effect_events : EventWriter<PlaySoundEvent>,
    mut ambient_sound_events : EventWriter<DelayedEvent<AmbientAudioEvent>>,
    audio_assets : Res<AudioAssets>,
    computer_state : Res<ComputerState>,
) {
    
    if !events.is_empty() && *computer_state == ComputerState::Off {
        println!("begin_startup");
        for mut loading_screen in loading_screens.iter_mut() {
            sound_effect_events.send(PlaySoundEvent::new(audio_assets.computer_rev.clone(), 0.1, None));
            ambient_sound_events.send(DelayedEvent::new(AmbientAudioEvent::play(AmbientAudioType::Computer, 0.1), 7.99));
            loading_screen.0.is_loading = true;
            *loading_screen.1 = Visibility::Visible;
        }
    }
    
    events.clear();
}

fn update_loading_screen (
    mut loading_screens: Query<(&mut LoadingScreenComponent, &AsciiNode, &mut Visibility), Without<TerminalComponent>>,
    mut terminals: Query<(&mut TerminalComponent, &AsciiNode, &mut Visibility), Without<LoadingScreenComponent>>,
    mut computer_state : ResMut<ComputerState>,
    mut mark_dirty : EventWriter<AsciiMarkDirtyEvent>,
    time : Res<Time>,
) { 
    for (mut loading_screen, node, mut vis) in loading_screens.iter_mut() {
        if loading_screen.is_loading {
            mark_dirty.send(AsciiMarkDirtyEvent);
            loading_screen.progress.tick(time.delta());
            
            if loading_screen.progress.finished() {
                loading_screen.is_loading = false;
                loading_screen.progress.reset();
                *vis = Visibility::Hidden;
                *computer_state = ComputerState::OS;
                for (_, _, mut visability) in terminals.iter_mut() {
                    *visability = Visibility::Visible;
                }
            }
        }
    }
}