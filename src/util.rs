
use std::{marker::PhantomData, time::Duration};

use bevy::{math::cubic_splines::CubicCurve, prelude::*};

//==============================================================================
//         UtilPLugin
//==============================================================================

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app
        
        ;
    }
}

//==============================================================================
//         Easing Functions
//==============================================================================

pub enum EasingFunction {
    Instant,
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl Into<CubicSegment<Vec2>> for EasingFunction {
    fn into(self) -> CubicSegment<Vec2> {
        match self {
            EasingFunction::Instant => CubicSegment::new_bezier((1.0, 1.0), (1.0, 1.0)),
            EasingFunction::Linear => CubicSegment::new_bezier((0.0, 0.0), (1.0, 1.0)),
            EasingFunction::EaseIn => CubicSegment::new_bezier((0.42, 0.0), (1.0, 1.0)),
            EasingFunction::EaseOut => CubicSegment::new_bezier((0.0, 0.0), (0.58, 1.0)),
            EasingFunction::EaseInOut => CubicSegment::new_bezier((0.42, 0.0), (0.58, 1.0)),
        }    
    }
}

//==============================================================================
//         DelayedEvent
//==============================================================================

pub struct DelayedEventPlugin<E : Event + Clone>(PhantomData<E>);

impl <E : Event + Clone> Default for DelayedEventPlugin<E> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl <E : Event + Clone> Plugin for DelayedEventPlugin<E> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostUpdate, catch_delayed_events::<E>)
            .add_systems(PreUpdate, run_delayed_events::<E>)
            
            .init_resource::<DelayedEvents<E>>()
            
            .add_event::<DelayedEvent<E>>()
        ;
    }
}

#[derive(Resource)]
pub struct DelayedEvents<E : Event + Clone> {
    events : Vec<DelayedEvent<E>>,
}

impl <E : Event + Clone> Default for DelayedEvents<E> {
    fn default() -> Self {
        Self {
            events : Vec::new(),
        }
    }
}

#[derive(Event, Clone)]
pub struct DelayedEvent<E : Event + Clone> {
    pub event : E,
    pub delay : Timer,
}

impl<E: Event + Clone> DelayedEvent<E> {
    pub fn new(event: E, seconds : f32) -> Self {
        Self { event, delay : Timer::new(Duration::from_secs_f32(seconds), TimerMode::Once) }
    }
}

fn run_delayed_events<E : Event + Clone>(
    mut delayed_events : ResMut<DelayedEvents<E>>,
    mut event_writer : EventWriter<E>,
    time : Res<Time>,
) {
    for event in delayed_events.events.iter_mut() {
        if event.delay.tick(time.delta()).just_finished() {
            event_writer.send(event.event.clone());
            println!("Event Sent")
        }
    }
    
    delayed_events.events.retain(|event| if event.delay.finished() { 
        println!("Removing Event");
        false 
    } else { 
        true 
    });
}

fn catch_delayed_events<E : Event + Clone>(
    mut events : EventReader<DelayedEvent<E>>,
    mut delayed_events : ResMut<DelayedEvents<E>>,
) {
    for event in events.read() {
        delayed_events.events.push(event.clone());
    }
}


