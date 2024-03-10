use std::{collections::VecDeque, fmt::format, time::Duration};

use bevy::prelude::*;
use bevy_ascii::prelude::*;

use super::commands::TerminalCommandEvent;

#[derive(Component)]
pub struct TerminalComponent {
    lines : VecDeque<String>,
    current_input : String,
    scroll : u32,
    blink_timer : Timer,
    blink : bool,
}

impl Default for TerminalComponent {
    fn default() -> Self {
        let lines = ["G64 Os 1.36.7 (build 10)".to_string(), "Welcome! Type 'help' for help.".to_string(), "".to_string()];
        Self {
            lines : lines.into(),
            current_input : String::from(""),
            scroll : 0,
            blink_timer : Timer::new(Duration::from_millis(500), TimerMode::Repeating),
            blink : true,
        }
    }
}

impl AsciiComponent for TerminalComponent {
    type UpdateQuery<'w, 's> = ();

    fn render(&self, buffer: &mut AsciiBuffer) {
        let buffer = buffer.padding((0, 0, 0, 1));
        
        { 
            let buffer = buffer.top(3);
            buffer.text("--- Genifore 64 Plus V4 ---")
                .vertical_alignment(VerticalAlignment::Center)
                .horizontal_alignment(HorizontalAlignment::Center)
                .draw();
        }
        
        let buffer = buffer.padding((3, 3, 3, 3));
        
        let max_lines = (buffer.bounds.height - 1) as usize;
        let terminal_history_height = self.lines.len();
        
        let string = self.lines.iter()
            .skip(self.scroll as usize)
            .take(max_lines.min(terminal_history_height))
            .fold(String::new(), |accum, value| format!("{}\n{}", accum, value));

        buffer.text(&string).draw();
        
        let command_line_y = terminal_history_height as u32 - self.scroll + 1;
        
        let command_line = buffer.relative(0, command_line_y as i32, 1.0, 1);
        command_line.text(&format!("User:> {}", self.current_input)).horizontal_alignment(HorizontalAlignment::Left).draw();
        if self.blink {
            command_line.relative(self.current_input.len() as i32 + 7, 0, 2, 2).text("_").draw();
            
        }
        
    }

    fn set_up(app: &mut App) {
        app
            .add_systems(Update, terminal_input)
        ;
    }
}

//==============================================================================
//         General Terminal Systems
//==============================================================================

fn terminal_input (
    mut terminals : Query<(&mut TerminalComponent, &AsciiNode, &InheritedVisibility)>,
    mut char_input : EventReader<ReceivedCharacter>,
    mut mark_ui_dirty : EventWriter<AsciiMarkDirtyEvent>,
    mut terminal_command_event : EventWriter<TerminalCommandEvent>,
    key_input : Res<ButtonInput<KeyCode>>,
    time : Res<Time>,
) {
    let Ok((mut terminal, node, visability)) = terminals.get_single_mut() else { return };
    if !visability.get() { return }
    
    let pressed_enter = key_input.just_pressed(KeyCode::Enter);
    let pressed_backspace = key_input.just_pressed(KeyCode::Backspace);
    let mut input_string = char_input.read().fold(String::new(), |mut accum, value| {
        for c in value.char.as_str().chars() {
            match bevy_ascii::prelude::Character::from(c) {
                Character::Nil | 
                Character::ArrowUp |
                Character::ArrowLeft => {},
                _ => accum.push(c),
            }
        }
        
        accum
    });

    terminal.blink_timer.tick(time.delta());
    if terminal.blink_timer.finished() {
        terminal.blink = !terminal.blink;
        mark_ui_dirty.send(AsciiMarkDirtyEvent);
    }
    
    if pressed_enter || pressed_backspace || !input_string.is_empty() {
        
        println!("Input String {input_string} | Backspace {pressed_backspace} | Enter {pressed_enter}");
        
        terminal.current_input.push_str(&input_string);
        if pressed_backspace {
            terminal.current_input.pop();
        }
        
        if pressed_enter {
            let input = terminal.current_input.clone();
            terminal.lines.push_back(input.clone());
            terminal.current_input.clear();
            terminal_command_event.send(TerminalCommandEvent(input));
        }
        
        terminal.blink = true;
        terminal.blink_timer.reset();
        mark_ui_dirty.send(AsciiMarkDirtyEvent);
    }
}

//==============================================================================
//         Terminal Commands
//==============================================================================

