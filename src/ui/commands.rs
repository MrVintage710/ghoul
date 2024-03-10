use bevy::prelude::*;
use bevy_ascii::prelude::AsciiNode;

use crate::{camera::{blackout::BlackoutTransition, path::CameraPathFollower, zone::{CameraZone, CameraZoneAction, CurrentZone}}, game::{ActiveCamera, ToggleGameWorldEvent}};

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
    mut room_camera : Query<(Entity, &Camera, &GlobalTransform, &mut CurrentZone)>,
    mut terminal : Query<(&mut TerminalComponent, &AsciiNode, &InheritedVisibility)>,
    mut reader: EventReader<TerminalCommandEvent>,
    mut world_event : EventWriter<ToggleGameWorldEvent>,
    mut fade_event : EventWriter<BlackoutTransition>,
    comp_camera : Query<Entity, With<ActiveCamera>>,
    camera_zones : Query<(Entity, &CameraZone, &Transform, Option<&Children>)>,
) {
    let Ok((mut terminal, node, visibility)) = terminal.get_single_mut() else { return };
    
    for command in reader.read() {
        match command.0.as_str() {
            "help" => {
                terminal.add_line("", node.bounds.width - 6);
                terminal.add_line("Available commands:", node.bounds.width - 6);
                terminal.add_line("  help - display this message", node.bounds.width - 6);
                terminal.add_line("  clear - clear the terminal", node.bounds.width - 6);
                terminal.add_line("  exit - leave the computer", node.bounds.width - 6);
                terminal.add_line("  off - turn the computer off", node.bounds.width - 6);
                terminal.add_line("  load - loads program off of cartridge.", node.bounds.width - 6);
                terminal.add_line("", node.bounds.width - 6);
            },
            "clear" => {
                terminal.lines.clear();
            },
            "exit" => {
                let Ok((cam_entity, camera, transform, mut current_zone)) = room_camera.get_single_mut() else {return};
                let Ok((zone_entity, zone, zone_transform, children)) = camera_zones.get(current_zone.0) else {return};
                
                let CameraZoneAction::Move(target_transform) = zone.event else {return};
                
                world_event.send(ToggleGameWorldEvent);
                fade_event.send(BlackoutTransition::fade_in(0.5));
                
                commands.entity(cam_entity).insert(CameraPathFollower::to_transform(
                    target_transform, 
                    0.5
                ));
            }
            _ => {
                terminal.add_line(&format!("Unknown command: {}", command.0), node.bounds.width - 6);
            }
        }
    }
}