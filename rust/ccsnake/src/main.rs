use bevy::prelude::*;

#[derive(Component)]
struct SnakeHead;

const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
