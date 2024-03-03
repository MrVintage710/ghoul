use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*, render::{camera::RenderTarget, mesh::shape::Cube, render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, view::RenderLayers}, window::WindowRef};
use bevy_ascii::prelude::*;

use crate::game::{ActiveCamera, GameState};

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
            .add_systems(Update, rotate_test_cubes.run_if(in_state(GameState::Active)))
            // .add_systems(PostUpdate, switch_to_computer_world)
        
            // .add_event::<SwitchToComputerWorld>()
        ;
    }
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
    
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
            camera : Camera {
                target : render_surface_image.clone().into(),
                ..Default::default()
            },
            ..Default::default()
        },
        AsciiCamera::default(),
        RenderLayers::layer(1),
        ComputerCamera,
    ));
    
    let render_surface_mat = materials.add(StandardMaterial {
        base_color_texture: Some(render_surface_image.clone()),
        emissive: Color::LIME_GREEN,
        ..Default::default()
    });
    
    commands.insert_resource(ComputerWorldAssets {
        render_surface_mat,
        render_surface_image,
    });
    
    let mesh = meshes.add(Cube::new(1.0));
    let red_material = materials.add(Color::RED);
    
    commands.spawn((
        PbrBundle {
            mesh,
            material: red_material,
            ..Default::default()
        },
        TestCube,
        RenderLayers::layer(1)
    ));
    
    // light
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            // This is a relatively small scene, so use tighter shadow
            // cascade bounds than the default for better quality.
            // We also adjusted the shadow map to be larger since we're
            // only using a single cascade.
            cascade_shadow_config: CascadeShadowConfigBuilder {
                num_cascades: 1,
                maximum_distance: 1.6,
                ..default()
            }
            .into(),
            ..default()
        },
        RenderLayers::layer(1)
    ));
}

//==============================================================================
//         Switch to ComputerWorld Event
//==============================================================================

// #[derive(Event, Debug)]
// pub struct SwitchToComputerWorld;

// pub fn switch_to_computer_world(
//     mut commands : Commands,
//     mut events : EventReader<SwitchToComputerWorld>,
//     mut computer_world_camera : Query<(Entity, &mut Camera, &mut Projection), (With<ComputerCamera>, Without<RoomCamera>)>,
//     mut room_camera : Query<(Entity, &mut Camera), (With<RoomCamera>, With<ActiveCamera>, Without<ComputerCamera>)>,
//     input : Res<ButtonInput<KeyCode>>,
// ) {
//     if !events.is_empty() || input.just_pressed(KeyCode::F1) {
//         let Ok((comp_cam_entity, mut comp_cam, mut comp_projection)) = computer_world_camera.get_single_mut() else { return };
//         let Ok((room_cam_entity, mut room_cam)) = room_camera.get_single_mut() else { return };
//         println!("Switching to Computer World");
//         comp_cam.target = RenderTarget::Window(WindowRef::Primary);
//         room_cam.is_active = false;
//         *comp_projection = Projection::default();
//         commands.entity(room_cam_entity).remove::<ActiveCamera>();
//         commands.entity(comp_cam_entity).insert(ActiveCamera);
//     }
    
//     events.read();
// }

//==============================================================================
//         Marker Components
//==============================================================================

#[derive(Debug, Component)]
pub struct ComputerCamera;

//==============================================================================
//         Test
//==============================================================================

#[derive(Component)]
struct TestCube;

fn rotate_test_cubes(
    time : Res<Time>,
    mut query : Query<&mut Transform, With<TestCube>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds()));
    }
}