use bevy::time::common_conditions::*;
use bevy::window::WindowResolution;
use bevy::{picking::window, prelude::*, window::PrimaryWindow};
use core::time::Duration;
use rand::prelude::random;

#[derive(Component)]
struct Food;

#[derive(Component)]
struct SnakeHead;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

const FOOD_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;
const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
//const SNAKE_SPEED: Duration = Duration::from_secs_f32(1.0 / 6.0);

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_snake(mut commands: Commands) {
    println!("Spawning Snek!");
    commands.spawn((
        Sprite {
            color: SNAKE_HEAD_COLOR,
            ..default()
        },
        Transform::default(),
        SnakeHead,
        Position { x: 3, y: 3 },
        Size::square(0.8),
        Direction::Up,
    ));
    println!("Snek Spawned");
}

fn snake_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<SnakeHead>>,
) {
    println!("Snek Move!");
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 2.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 2.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += 2.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= 2.0;
        }
    }
}

fn size_scaling(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>,
) {
    println!("Size Scaling");
    let Ok(window) = window_query.single() else {
        return;
    };
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width(),
            sprite_size.height / ARENA_HEIGHT as f32 * window.height(),
            1.0,
        );
    }
}

fn position_translation(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let title_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.0) + (title_size / 2.0)
    }
    let Ok(window) = window_query.single() else {
        return;
    };

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width(), ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height(), ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn food_spawner(mut commands: Commands) {
    println!("Spawning Food!");
    let x = (random::<f32>() * ARENA_WIDTH as f32) as i32;
    let y = (random::<f32>() * ARENA_HEIGHT as f32) as i32;
    commands.spawn((
        Sprite {
            color: FOOD_COLOR,
            ..default()
        },
        Food,
        Position { x, y },
        Size::square(0.8),
    ));
    println!("Food Spawned at ({}, {})", x, y);
}

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(800, 600),
                title: "Snek!".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_snake)
        .add_systems(Startup, food_spawner)
        .add_systems(Update, snake_movement)
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .run();
    println!("App finished");
}
