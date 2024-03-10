pub mod boot_screen;
pub mod terminal;
pub mod commands;

use bevy::prelude::*;
use bevy_ascii::prelude::*;

use self::{boot_screen::LoadingScreenComponent, commands::TerminalCommandPlugin, terminal::TerminalComponent};

//==============================================================================
//         Ui Plugin
//==============================================================================

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TerminalCommandPlugin)
            
            .add_plugins(AsciiComponentPlugin::<LoadingScreenComponent>::default())
            .add_plugins(AsciiComponentPlugin::<TerminalComponent>::default())
        ;
    }
}