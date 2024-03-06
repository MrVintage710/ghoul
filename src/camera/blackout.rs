use std::time::Duration;

use bevy::{prelude::*, render::view::RenderLayers};

use crate::util::DelayedEventPlugin;

//==============================================================================
//         Blackout Plugin
//==============================================================================

pub struct BlackoutPlugin;

impl Plugin for BlackoutPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DelayedEventPlugin::<BlackoutTransition>::default())
            
            .add_systems(Startup, init_blackout)
            .add_systems(PostUpdate, (start_blackout_transition, update_blackout))
        
            .add_event::<BlackoutTransition>()
            
            .register_type::<BlackoutTransition>()
        ;
    }
}

//==============================================================================
//         Blackout Event
//==============================================================================

#[derive(Debug, Event, Clone, Component, Reflect)]
pub enum BlackoutTransition {
    FadeIn(Timer),
    FadeOut(Timer),
}

impl BlackoutTransition {
    pub fn fade_in(seconds : f32) -> Self {
        Self::FadeIn(Timer::new(Duration::from_secs_f32(seconds), TimerMode::Once))
    }
    pub fn fade_out(seconds : f32) -> Self {
        Self::FadeOut(Timer::new(Duration::from_secs_f32(seconds), TimerMode::Once))
    }
}

//==============================================================================
//         Blackout Marker
//==============================================================================

#[derive(Component)]
pub struct BlackoutUI;

//==============================================================================
//         Blackout Systems
//==============================================================================

fn init_blackout(
    mut commands: Commands,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        },
        Name::new("Blackout UI"),
        RenderLayers::layer(0),
        BlackoutUI
    ));
}

fn start_blackout_transition(
    mut commands: Commands,
    mut event_reader: EventReader<BlackoutTransition>,
    blackout_ui: Query<Entity, With<BlackoutUI>>,
) {
    let Ok(blackout_entity) = blackout_ui.get_single() else { return };
    for event in event_reader.read() {
        commands.entity(blackout_entity).insert(event.clone());
    }
}

fn update_blackout(
    mut commands : Commands,
    mut blackout_ui : Query<(Entity, &mut BlackoutTransition, &mut BackgroundColor), With<BlackoutUI>>,
    time : Res<Time>
) {
    
    for (entity, mut blackout_transition, mut background_color) in blackout_ui.iter_mut() {
        match &mut *blackout_transition {
            BlackoutTransition::FadeIn(timer) => {
                let progress = timer.fraction_remaining();
                background_color.0.set_a(progress);
                timer.tick(time.delta());
                if timer.just_finished() {
                    commands.entity(entity).remove::<BlackoutTransition>();
                }
            },
            BlackoutTransition::FadeOut(timer) => {
                let progress = timer.fraction();
                background_color.0.set_a(progress);
                timer.tick(time.delta());
                if timer.just_finished() {
                    commands.entity(entity).remove::<BlackoutTransition>();
                }
            },
        }
    }
    
}