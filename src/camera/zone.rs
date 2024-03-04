use bevy::prelude::*;

//==============================================================================
//         CameraZone Plugin
//==============================================================================

pub struct CameraZonePlugin;

impl Plugin for CameraZonePlugin {
    fn build(&self, app: &mut App) {
        app 
            .register_type::<CameraZone>()
        ;
    }
}

//==============================================================================
//         CameraZone Component
//==============================================================================

#[derive(Component, Reflect, Default)]
pub struct CameraZone {
    pub bounds : Cuboid,
    pub target_transform : Transform,
}

//==============================================================================
//         CameraZoneBundle
//==============================================================================

#[derive(Default, Bundle)]
pub struct CameraZoneBundle {
    camera_zone : CameraZone,
    transform : Transform,
}

impl CameraZoneBundle {
    pub fn new(camera_zone: CameraZone, transform: Transform) -> Self {
        Self { camera_zone, transform }
    }
}