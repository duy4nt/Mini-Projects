use bevy::prelude::*;

const RESOLUTION_WIDTH: f32 = 1600.0;
const RESOLUTION_HEIGHT: f32 = 900.0;
const GRAVITATION_CONST: f32 = 4.0;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Mass {
    m: f32,
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
            primary_window: Some(Window {
                title: "Three Body Problem".to_string(),
                resolution: (RESOLUTION_WIDTH, RESOLUTION_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_sprite))
        .add_systems(
            Update,
            (calculate_acceleration, apply_acceleration, apply_velocity),
        )
        .run();
}
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
fn setup_sprite(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(24.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
        Transform::from_xyz(-200.0, -200.0, 0.0),
        Mass { m: 2.0 },
        Velocity { x: 100.0, y: 0.0 },
        Acceleration { x: 0.0, y: 0.0 },
    ));
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(28.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mass { m: 4.0 },
        Velocity { x: 50.0, y: 0.0 },
        Acceleration { x: 0.0, y: 0.0 },
    ));
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(32.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
        Transform::from_xyz(200.0, 200.0, 0.0),
        Mass { m: 8.0 },
        Velocity { x: -100.0, y: 0.0 },
        Acceleration { x: 0.0, y: 0.0 },
    ));
}
fn calculate_acceleration(
    mut query: Query<(Entity, &Mass, &Transform, &mut Acceleration), With<Mass>>,
) {
    let bodies: Vec<(Entity, f32, Vec3)> = query
        .iter()
        .map(|(entity, mass, transform, _)| (entity, mass.m, transform.translation))
        .collect();

    for (entity, mass, transform, mut acceleration) in query.iter_mut() {
        acceleration.x = 0.0;
        acceleration.y = 0.0;
        let current_mass = mass.m;
        let current_position = transform.translation;
        for (other_entity, other_mass, other_position) in &bodies {
            if entity == *other_entity {
                continue;
            }

            let dx = other_position.x - current_position.x;
            let dy = other_position.y - current_position.y;
            let distance = (dx * dx + dy * dy).sqrt();

            let acc_mag = GRAVITATION_CONST * current_mass * other_mass / distance * distance;
            //TODO: Change the acc_mag from a constant to now depend on mass.
            if distance > 0.1 {
                acceleration.x += (dx / distance) * acc_mag;
                acceleration.y += (dy / distance) * acc_mag;
            }
        }
    }
}
fn apply_acceleration(mut query: Query<(&mut Velocity, &Acceleration)>, time: Res<Time>) {
    for (mut velocity, acceleration) in &mut query {
        velocity.x += acceleration.x * time.delta_secs();
        velocity.y += acceleration.y * time.delta_secs();
    }
}
fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}
