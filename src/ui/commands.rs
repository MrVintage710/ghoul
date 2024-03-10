use bevy::prelude::*;

use super::terminal::TerminalComponent;

//==============================================================================
//         Commands Plungin
//==============================================================================

pub struct TerminalCommandPlugin;

impl Plugin for TerminalCommandPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostUpdate, on_terminal_command)
        
            .add_event::<TerminalCommandEvent>()
        ;
    }
}

//==============================================================================
//         Terminal Command Event
//==============================================================================

#[derive(Event)]
pub struct TerminalCommandEvent(pub String);

pub fn on_terminal_command(
    mut commands: Commands,
    mut terminal : Query<(&mut TerminalComponent, &InheritedVisibility)>,
    mut reader: EventReader<TerminalCommandEvent>,
) {
    let Ok((terminal, visibility)) = terminal.get_single_mut() else { return };
    
    for command in reader.read() {
        match command.0.as_str() {
            "help" => {},
            _ => {}
        }
        
        println!("Command: {}", command.0);
    }
}