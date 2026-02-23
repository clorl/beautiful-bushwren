
#![allow(unused)]

use std::{f32::consts::FRAC_PI_2, ops::Range};
use bevy::{
    prelude::*,
    input::mouse::AccumulatedMouseMotion,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, orbital_camera_system);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    //

    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanControls::default(),
        OrbitalCamera::default(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(1.0, 2.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_translation(Vec3::ZERO)
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}


#[derive(Component, Debug)]
#[require(Camera3d, Transform)]
pub struct OrbitalCamera {
    pub orbit_distance: f32,
    pub pitch_speed: f32,
    pub pitch_range: Range<f32>,
    pub roll_speed: f32,
    pub yaw_speed: f32,
}

impl Default for OrbitalCamera {
    fn default() -> Self {
        let pitch_limit = FRAC_PI_2 - 0.01;
        Self {
            orbit_distance: 20.0,
            pitch_speed: 0.003,
            pitch_range: -pitch_limit..pitch_limit,
            roll_speed: 1.0,
            yaw_speed: 0.004,
        }
    }
}

#[derive(Component, Debug)]
#[require(Transform)]
pub struct PanControls {
    pub velocity: Vec2,
    pub max_speed: f32,
    pub limits: Option<Vec2>,
    pub drag: f32,
    pub acceleration: f32
}

impl Default for PanControls {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            max_speed: 2.0,
            limits: None,
            drag: 0.5,
            acceleration: 5.0,
        }
    }
}

fn pan_controls_system(
    mut query: Query<(&mut Transform, &mut PanControls)>,
    kb: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {
    let input = (
        kb.pressed(KeyCode::KeyD) as i32 - kb.pressed(KeyCode::KeyA) as i32,
        kb.pressed(KeyCode::KeyW) as i32 - kb.pressed(KeyCode::KeyS) as i32
    );
    for (mut transform, mut controller) in &mut query {
    }
}

fn orbital_camera_system(
    mut query: Query<(&OrbitalCamera, &mut Transform), With<Camera3d>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>
) {
    if !mouse_buttons.pressed(MouseButton::Left) {
        return;
    }

    for (camera_settings, mut transform) in &mut query {
        let delta = mouse_motion.delta;
        let delta_roll = 0.0;
        let delta_pitch = delta.y * camera_settings.pitch_speed;
        let delta_yaw = delta.x * camera_settings.yaw_speed;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

        // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
        let pitch = (pitch + delta_pitch).clamp(
            camera_settings.pitch_range.start,
            camera_settings.pitch_range.end,
        );
        let roll = roll + delta_roll;
        let yaw = yaw + delta_yaw;
        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

        // Adjust the translation to maintain the correct orientation toward the orbit target.
        // In our example it's a static target, but this could easily be customized.
        let target = Vec3::ZERO;
        transform.translation = target - transform.forward() * camera_settings.orbit_distance;
    }
}

fn update() {
}
