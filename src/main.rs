use bevy::prelude::*;

const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_FLOOR_GAP: f32 = 30.0;
const PADDLE_SPEED: f32 = 500.0;

#[derive(Component)]
struct Paddle;

const BALL_RADIUS: f32 = 20.0;

#[derive(Component)]
struct Ball;

const BRICK_SIZE: Vec2 = Vec2::new(80.0, 10.0);
const BRICK_GAP: Vec2 = Vec2::new(5.0, 5.0);
const BRICK_ROWS: u8 = 14;
const CELL_WIDTH: f32 = BRICK_SIZE.x + BRICK_GAP.x;
const CELL_HEIGHT: f32 = BRICK_SIZE.y + BRICK_GAP.y;

#[derive(Component)]
struct Brick;

#[derive(Resource)]
struct PaddleBounds {
    left: f32,
    right: f32,
}

#[derive(Resource)]
struct WindowBounds {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}

#[derive(Resource)]
struct GridState {
    start_y: f32,
    bricks_per_row: u8,
    left_edge: f32,
}

const GRID_SPAWN_SECONDS: f32 = 5.0;
#[derive(Resource)]
struct GridSpawnTimer(Timer);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    let window = windows.single().unwrap();
    let window_height = window.height();
    let top = window_height / 2.0;
    let bottom = -window_height / 2.0;
    let left = -window.width() / 2.0;
    let right = window.width() / 2.0;
    let paddle_half_width = PADDLE_SIZE.x / 2.0;

    commands.insert_resource(GridSpawnTimer(Timer::from_seconds(
        GRID_SPAWN_SECONDS,
        TimerMode::Repeating,
    )));

    commands.insert_resource(PaddleBounds {
        left: left + paddle_half_width,
        right: right - paddle_half_width,
    });

    commands.insert_resource(WindowBounds {
        top,
        bottom,
        left,
        right,
    });

    let bricks_per_row = (window.width() / CELL_WIDTH).floor() as u8;
    let start_y = top - (CELL_HEIGHT / 2.0);
    let left_edge = -(bricks_per_row as f32 * CELL_WIDTH) / 2.0;

    commands.insert_resource(GridState {
        start_y,
        bricks_per_row,
        left_edge,
    });

    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            custom_size: Some(PADDLE_SIZE),
            color: Color::WHITE,
            ..default()
        },
        Transform::from_xyz(0.0, bottom + PADDLE_FLOOR_GAP, 0.0),
        Paddle,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_xyz(
            0.0,
            bottom + PADDLE_FLOOR_GAP + (PADDLE_SIZE.y / 2.0) + BALL_RADIUS,
            0.0,
        ),
        Ball,
    ));

    for row in 0..BRICK_ROWS {
        let y = start_y - row as f32 * CELL_HEIGHT;
        create_brick_row(&mut commands, bricks_per_row, left_edge, y);
    }
}

fn spawn_brick_row(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<GridSpawnTimer>,
    grid_state: Res<GridState>,
    mut bricks: Query<&mut Transform, With<Brick>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        // shift existing bricks
        for mut t in bricks.iter_mut() {
            t.translation.y -= CELL_HEIGHT;
        }

        create_brick_row(
            &mut commands,
            grid_state.bricks_per_row,
            grid_state.left_edge,
            grid_state.start_y,
        );
    }
}

fn create_brick_row(commands: &mut Commands, bricks_per_row: u8, grid_left_edge: f32, y: f32) {
    for column in 0..bricks_per_row {
        let x = grid_left_edge + (CELL_WIDTH / 2.0) + column as f32 * CELL_WIDTH;
        create_brick(commands, x, y);
    }
}

fn create_brick(commands: &mut Commands, x: f32, y: f32) {
    commands.spawn((
        Sprite {
            custom_size: Some(BRICK_SIZE),
            color: Color::WHITE,
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
        Brick,
    ));
}

fn handle_input(
    input: Res<ButtonInput<KeyCode>>,
    mut transform: Single<&mut Transform, With<Paddle>>,
    time: Res<Time>,
    paddle_bounds: Res<PaddleBounds>,
) {
    let mut direction = 0.0;
    if input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    } else if input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    let new_position = transform.translation.x + direction * PADDLE_SPEED * time.delta_secs();

    transform.translation.x = new_position.clamp(paddle_bounds.left, paddle_bounds.right);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, handle_input)
        .add_systems(Update, spawn_brick_row)
        .run();
}
