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

#[derive(Component)]
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

#[derive(Resource)]
struct GameState {
    snake_body: VecDeque<GridPosition>,
    direction: Direction,
    next_direction: Direction,
    move_timer: Timer,
    score: u32,
    game_over: bool,
}

impl Default for GameState {
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

fn setup(mut commands: Commands, game_state: Res<GameState>) {
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

fn spawn_food(commands: &mut Commands) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(0..GRID_WIDTH);
    let y = rng.gen_range(0..GRID_HEIGHT);

    commands.spawn((
        Sprite {
            color: Color::srgb(0.9, 0.1, 0.1),
            custom_size: Some(Vec2::splat(CELL_SIZE - 4.0)),
            ..default()
        },
        Transform::from_translation(grid_to_world(x, y)),
        Food,
        GridPosition { x, y },
    ));
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

    let new_direction = if keyboard.just_pressed(KeyCode::ArrowUp) {
        Some(Direction::Up)
    } else if keyboard.just_pressed(KeyCode::ArrowDown) {
        Some(Direction::Down)
    } else if keyboard.just_pressed(KeyCode::ArrowRight) {
        Some(Direction::Right)
    } else if keyboard.just_pressed(KeyCode::ArrowLeft) {
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

fn move_snake(
    mut commands: Commands,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    food_query: Query<(Entity, &GridPosition), With<Food>>,
) {
    if game_state.game_over {
        return;
    }

    game_state.move_timer.tick(time.delta());

    if !game_state.move_timer.just_finished() {
        return;
    }

    game_state.direction = game_state.next_direction;

    let head = game_state.snake_body.front().unwrap();
    let new_head = GridPosition {
        x: match game_state.direction {
            Direction::Left => head.x - 1,
            Direction::Right => head.x + 1,
            _ => head.x,
        },
        y: match game_state.direction {
            Direction::Up => head.y + 1,
            Direction::Down => head.y - 1,
            _ => head.y,
        },
    };

    let mut food_eaten = false;
    for (entity, food_pos) in food_query.iter() {
        if new_head.x == food_pos.x && new_head.y == food_pos.y {
            commands.entity(entity).despawn();
            spawn_food(&mut commands);
            game_state.score += 1;
            food_eaten = true;
            break;
        }
    }

    game_state.snake_body.push_front(new_head);

    if !food_eaten {
        game_state.snake_body.pop_back();
    }
}

fn check_collision(mut game_state: ResMut<GameState>, mut commands: Commands) {
    if game_state.game_over {
        return;
    }

    let head = game_state.snake_body.front().unwrap();

    if head.x < 0 || head.x >= GRID_WIDTH || head.y < 0 || head.y >= GRID_HEIGHT {
        game_state.game_over = true;
        spawn_game_over_text(&mut commands, game_state.score);
        return;
    }

    for segment in game_state.snake_body.iter().skip(1) {
        if head.x == segment.x && head.y == segment.y {
            game_state.game_over = true;
            spawn_game_over_text(&mut commands, game_state.score);
            return;
        }
    }
}

fn spawn_game_over_text(commands: &mut Commands, score: u32) {
    commands.spawn((
        Text::new(format!(
            "Game Over! Score: {}\nPress SPACE to restart",
            score
        )),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            ..default()
        },
    ));
}

fn update_visuals(
    mut commands: Commands,
    game_state: Res<GameState>,
    segment_query: Query<Entity, With<SnakeSegment>>,
) {
    if !game_state.is_changed() {
        return;
    }

    // Despawn all segments
    for entity in segment_query.iter() {
        commands.entity(entity).despawn();
    }

    // Respawn segments at new positions
    for pos in &game_state.snake_body {
        spawn_snake_segment(&mut commands, pos);
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Snek!".to_string(),
                    resolution: (
                        GRID_WIDTH as f32 * CELL_SIZE,
                        GRID_HEIGHT as f32 * CELL_SIZE,
                    )
                        .into(),
                    ..default()
                }),
                ..default()
            }),
        )
        .init_resource::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (handle_input, move_snake, check_collision, update_visuals).chain(),
        )
        .run();
}
