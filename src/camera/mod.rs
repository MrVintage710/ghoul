pub mod path;
pub mod fly;
pub mod zone;

use bevy::prelude::*;

use self::{fly::FlyCamPlugin, path::CameraPathPlugin, zone::CameraZonePlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FlyCamPlugin)
            .add_plugins(CameraPathPlugin)
            .add_plugins(CameraZonePlugin)
        ;
    }
}
