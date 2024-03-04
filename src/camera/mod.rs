pub mod path;
pub mod fly;

use bevy::prelude::*;

use self::path::CameraPathPlugin;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(fly::FlyCamPlugin)
            .add_plugins(CameraPathPlugin)
        ;
    }
}
