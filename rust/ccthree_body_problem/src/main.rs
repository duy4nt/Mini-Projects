use bevy::prelude::*;

const RESOLUTION_WIDTH: f32 = 1600.0;
const RESOLUTION_HEIGHT: f32 = 900.0;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Acceleration {
    x: f32,
    y: f32,
}

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window : Some(Window {
                title : "Three Body Problem".to_string(),
                resolution : (RESOLUTION_WIDTH, RESOLUTION_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_sprite)).run();
}
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn setup_sprite(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(32.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
        Transform::from_xyz(-200.0, 0.0, 0.0),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(32.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(32.5))),
        MeshMaterial2d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
        Transform::from_xyz(200.0, 0.0, 0.0),
    ));
}
