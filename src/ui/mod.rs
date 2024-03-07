
pub mod boot_screen;

use bevy::prelude::*;
use bevy_ascii::prelude::*;

use self::boot_screen::LoadingScreenComponent;

//==============================================================================
//         Ui Plugin
//==============================================================================

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AsciiComponentPlugin::<LoadingScreenComponent>::default())
        ;
    }
}