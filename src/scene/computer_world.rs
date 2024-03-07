use std::default;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*, render::{camera::RenderTarget, mesh::shape::Cube, render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, view::RenderLayers}, window::WindowRef};
use bevy_ascii::prelude::*;

use crate::{game::{ActiveCamera, GameState, GameWorld, OnGameWorldChangeEvent}, ui::boot_screen::LoadingScreenComponent};

use super::{RoomCamera};

//==============================================================================
//         ComputerWorldPlugin
//==============================================================================

pub struct ComputerWorldPlugin;

impl Plugin for ComputerWorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(AsciiShaderPlugin)
        
            .add_systems(Startup, init_computer_world)
        
            .init_resource::<ComputerState>()
        
            // .add_event::<SwitchToComputerWorld>()
        ;
    }
}

//==============================================================================
//         Marker Components
//==============================================================================

#[derive(Debug, Component)]
pub struct ComputerCamera;

//==============================================================================
//         ComputerState
//==============================================================================

#[derive(Resource, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComputerState {
    #[default]
    Off,
    OS,
    Game,
}

//==============================================================================
//         Load ComputerWorld
//==============================================================================

#[derive(Resource)]
pub struct ComputerWorldAssets {
    pub render_surface_mat : Handle<StandardMaterial>,
    pub render_surface_image : Handle<Image>,
}

//==============================================================================
//         Init ComputerWorld
//==============================================================================

fn init_computer_world(
    mut commands : Commands,
    mut images : ResMut<Assets<Image>>,
    mut materials : ResMut<Assets<StandardMaterial>>,
    mut meshes : ResMut<Assets<Mesh>>,
) {
    let size = Extent3d {
        width: 1228,
        height: 1024,
        ..default()
    };
    
    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("ComputerRenderSurface"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);
    
    let render_surface_image = images.add(image);
    
    let camera = commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
            camera : Camera {
                target : render_surface_image.clone().into(),
                ..Default::default()
            },
            ..Default::default()
        },
        AsciiCamera::default(),
        AsciiUi::default(),
        RenderLayers::layer(1),
        VisibilityBundle::default(),
        ComputerCamera,
    )).id();
    
    let render_surface_mat = materials.add(StandardMaterial {
        base_color_texture: Some(render_surface_image.clone()),
        emissive: Color::LIME_GREEN,
        ..Default::default()
    });
    
    commands.insert_resource(ComputerWorldAssets {
        render_surface_mat,
        render_surface_image,
    });
    
    commands.ascii_ui_with_parent(camera)
        .aligned(1.0, 1.0, HorizontalAlignment::Center, VerticalAlignment::Center, LoadingScreenComponent::default())
            .hidden().insert(RenderLayers::layer(1));
    ;
    
    // light
    // commands.spawn((
    //     DirectionalLightBundle {
    //         directional_light: DirectionalLight {
    //             shadows_enabled: true,
    //             ..default()
    //         },
    //         // This is a relatively small scene, so use tighter shadow
    //         // cascade bounds than the default for better quality.
    //         // We also adjusted the shadow map to be larger since we're
    //         // only using a single cascade.
    //         cascade_shadow_config: CascadeShadowConfigBuilder {
    //             num_cascades: 1,
    //             maximum_distance: 1.6,
    //             ..default()
    //         }
    //         .into(),
    //         ..default()
    //     },
    //     RenderLayers::layer(1)
    // ));
}

//==============================================================================
//         ComputerWorld Systems
//==============================================================================

fn on_enter_computer_world(
    mut gameworld_change_events : EventReader<OnGameWorldChangeEvent>,
    computer_state : Res<ComputerState>,
) {
    for game_world_change_event in gameworld_change_events.read() {
        if game_world_change_event.0 == GameWorld::Computer {
            match *computer_state {
                ComputerState::Off => todo!(),
                ComputerState::OS => {},
                ComputerState::Game => {},
            }
            
            println!("Entering Computer World");
        }
    }
}

//==============================================================================
//         BootUp Computer
//==============================================================================

