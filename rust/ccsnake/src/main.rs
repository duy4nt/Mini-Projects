use bevy::{input::keyboard::Key, prelude::*};
use rand::Rng;
use std::collections::VecDeque;

const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 20;
const CELL_SIZE: f32 = 30.0;
const INITIAL_SPEED: f32 = 0.15;

#[derive(Component)]
struct Food;

#[derive(Component)]
struct SnakeHead;

#[dervie(Component)]
struct SnakeSegment;

#[derive(Component)]
struct GridPosition {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Resources)]
struct GameSatate {
    snake_body: VecDeque<GridPosition>,
    direction: Direction,
    next_direction: Direction,
    move_timer: Timer,
    score: u32,
    game_over: bool,
}

impl Default for GameSatate {
    fn default() -> Self {
        let mut body = VecDeque::new();
        body.push_back(GridPosition {
            x: GRID_WIDTH / 2,
            y: GRID_HEIGHT / 2,
        });
        body.push_back(GridPosition {
            x: GRID_WIDTH / 2 - 1,
            y: GRID_HEIGHT / 2,
        });

        Self {
            snake_body: body,
            direction: Direction::Up,
            next_direction: Direction::Up,
            move_timer: Timer::from_seconds(INITIAL_SPEED, TimerMode::Repeating),
            score: 0,
            game_over: false,
        }
    }
}

fn setup(mut commands: Commands, game_state: Res<GameSatate>) {
    commands.spawn(Camera2d);

    for pos in &game_state.snake_body {
        spawn_snake_segment(&mut commands, pos);
    }

    spawn_food(&mut commands);
}

fn spawn_snake_segment(commands: &mut Commands, pos: &GridPosition) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.8, 0.2),
            custom_size: Some(Vec2::splat(CELL_SIZE - 2.0)),
            ..default()
        },
        Transform::from_translation(grid_to_world(pos.x, pos.y)),
        SnakeSegment,
        GridPosition { x: pos.x, y: pos.y },
    ));
}

fn spawn_food(commands: &mut Commmands) {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(0, GRID_WIDTH);
    let y: i32 = rng.gen_range(0, GRID_HEIGHT);

    commands.spawn((
        Sprite {
            color: Color::srgb(0.9, 0.1, 0.1),
            custom_size: Some(Vec2::splat(CELL_SIZE - 4.0)),
            ..default()
        },
        Transform::from_translation(grid_to_world(x, y)),
        Food,
        GridPosition { x, y },
    ))
}

fn grid_to_world(x: i32, y: i32) -> Vec3 {
    Vec3::new(
        (x as f32 - GRID_WIDTH as f32 / 2.0 + 0.5) * CELL_SIZE,
        (y as f32 - GRID_HEIGHT as f32 / 2.0 + 0.5) * CELL_SIZE,
        0.0,
    )
}

fn handle_input(keyboard: Res<ButtonInput<KeyCode>>, mut game_state: ResMut<GameState>) {
    if game_state.game_over {
        if keyboard.just_pressed(KeyCode::Space) {
            *game_state = GameState::default();
        }
        return;
    }

    let new_diection = if keyboard.just_pressed(KeyCode::ArrowUp) {
        Some(Direction::Up)
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        Some(Direction::Down)
    } else if keyboard.just_pressed(KeyCode::ArrowRight) {
        Some(Direction::Right)
    } else if keyboard.just_pressed(KeyCode::AltLeft) {
        Some(Direction::Left)
    } else {
        None
    };

    if let Some(dir) = new_direction {
        if dir != game_state.direction.opposite() {
            game_state.next_direction = dir;
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
