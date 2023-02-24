use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct FreeCameraPlugin;
#[derive(Component)]
pub struct FreeCameraTag;
#[derive(Component)]
pub struct FreeCameraData;

impl Plugin for FreeCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(camera_look)
            .add_system(camera_movement);
    }
}
fn setup(mut commands: Commands) {
    commands
        .spawn_empty()
        .insert(FreeCameraTag)
        .insert(FreeCameraData)
        .insert_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        });
}
fn camera_movement(
    mut transform_query: Query<&mut Transform, With<FreeCameraTag>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in transform_query.iter_mut() {
        let mut move_vec = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) {
            let forw_vec = transform.forward();
            move_vec += forw_vec;
        }
        if keyboard_input.pressed(KeyCode::S) {
            let forw_vec = transform.forward();
            move_vec -= forw_vec;
        }
        if keyboard_input.pressed(KeyCode::A) {
            let right_vec = transform.right();
            move_vec -= right_vec;
        }
        if keyboard_input.pressed(KeyCode::D) {
            let right_vec = transform.right();
            move_vec += right_vec;
        }
        transform.translation += move_vec * 30.0 * time.delta_seconds();
    }
}
fn camera_look(
    mut cameratransform: Query<&mut Transform, With<FreeCameraTag>>,
    mut mouse_evr: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let mut delta = Vec2::ZERO;
    for ev in mouse_evr.iter() {
        delta += ev.delta * time.delta_seconds() * 0.4;
    }
    for mut transform in cameratransform.iter_mut() {
        let pitch = Quat::from_rotation_x(-delta.y);
        transform.rotation = transform.rotation * pitch;

        let yaw = Quat::from_rotation_y(-delta.x);
        transform.rotation = yaw * transform.rotation; // rotate around global y axis
    }
}
fn main() {}
