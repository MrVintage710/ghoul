use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*, render::{mesh::shape::Cube, render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages}, view::RenderLayers}};
use bevy_ascii::prelude::*;

use crate::game::GameState;

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
        ;
    }
}

//==============================================================================
//         Load ComputerWorld
//==============================================================================

#[derive(Resource)]
pub struct ComputerWorldAssets {
    pub render_surface_mat : Handle<StandardMaterial>,
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
    
    let surface_handle = images.add(image);
    
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
            camera : Camera {
                target : surface_handle.clone().into(),
                ..Default::default()
            },
            ..Default::default()
        },
        AsciiCamera::default(),
        RenderLayers::layer(1)
    ));
    
    let render_surface_mat = materials.add(StandardMaterial {
        base_color_texture: Some(surface_handle.clone()),
        emissive: Color::LIME_GREEN,
        ..Default::default()
    });
    
    commands.insert_resource(ComputerWorldAssets {
        render_surface_mat,
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