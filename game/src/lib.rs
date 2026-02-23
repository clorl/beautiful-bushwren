#![allow(unused)]

use components::*;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};

pub mod components;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_systems(Startup, startup)
            .add_systems(Update, camera_target_focus);
    }
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 0.5,
            scaling_mode: bevy::camera::ScalingMode::WindowSize,
            ..default_2d()
        }
    ));

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 2, 2, 0, 0);
    commands.spawn((
        asset_server.load("human/head_m.png"),
        TextureAtlas {
            layout: texture_atlas_layouts.add(layout),
            index: 0,
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        CameraTarget {}
    ));
}

fn camera_target_focus(
    mut camera: Single<&mut Transform, With<Camera2d>>,
    target: Query<&Transform, (With<CameraTarget>, Changed<Transform>, Without<Camera2d>)>
) {
    if let (Ok(mut cam_t), Ok(target_t)) = (camera.get_single_mut(), target.get_single()) {
        cam_t.translation.x = target_t.translation.x;
        cam_t.translation.y = target_t.translation.y;
    }
}

#[derive(Component)]
struct CameraTarget {}

#[derive(Component, Default, Clone, Copy, Reflect)]
enum Facing {
    #[default]
    North,
    South,
    West,
    East
}

#[derive(Component, Default, Clone, Copy, Reflect)]
enum InheritFacing {}
