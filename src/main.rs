use bevy::{input::keyboard::KeyboardInput, prelude::*};
use rand::prelude::*;

const PADDLE_SPEED: f32 = 500.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_WIDTH: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(FixedUpdate, move_paddle)
        .add_systems(Update, handle_input)
        .add_systems(FixedUpdate, (apply_velocity, apply_collision).chain())
        .add_systems(Update, display_score)
        .run();
}

#[derive(PartialEq)]
enum Player {
    One,
    Two,
}

#[derive(Component)]
struct Paddle {
    player: Player,
}

#[derive(Component)]
struct Ball {
    velocity: Vec3,
}

use bevy::prelude::Resource;

#[derive(Resource, Default)]
struct Score {
    player_one: u32,
    player_two: u32,
}

#[derive(Resource, Default)]
struct PressedState {
    key_w: bool,
    key_s: bool,
    arrow_up: bool,
    arrow_down: bool,
}

#[derive(Component)]
struct ScoreText();

fn setup(mut commands: Commands, window: Query<&Window>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..Default::default()
    });
    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));

    let sizes: Vec<(f32, f32)> = window.iter().map(|w| (w.width(), w.height())).collect();
    let (width, _) = sizes[0];

    commands.insert_resource(Score {
        ..Default::default()
    });

    commands.insert_resource(PressedState {
        ..Default::default()
    });

    //spawn paddle one
    commands
        .spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-width / 2.0 + PADDLE_WIDTH, 0.0, 0.0),

                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Paddle {
            player: Player::One,
        });

    //spawn paddle two
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(width / 2.0 - PADDLE_WIDTH, 0.0, 0.0),

                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle {
            player: Player::Two,
        },
    ));

    //spawn ball
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Ball {
            velocity: Vec3::new(
                random::<f32>() * 2.0 - 1.0,
                random::<f32>() * 2.0 - 1.0,
                0.0,
            )
            .normalize()
                * 200.0,
        },
    ));

    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(90.0),

                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!("{} - {}", 0, 0),
                    TextStyle {
                        font_size: 50.0,
                        color: Color::rgb(1.0, 1.0, 1.0),
                        ..Default::default()
                    },
                ),
                ScoreText {},
            ));
        });
}

fn handle_input(
    mut state: ResMut<PressedState>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    for event in keyboard_input_events.read() {
        match event.key_code {
            KeyCode::KeyW => {
                state.key_w = event.state.is_pressed();
            }
            KeyCode::KeyS => {
                state.key_s = event.state.is_pressed();
            }
            KeyCode::ArrowUp => {
                state.arrow_up = event.state.is_pressed();
            }
            KeyCode::ArrowDown => {
                state.arrow_down = event.state.is_pressed();
            }
            _ => {}
        }
    }
}

fn move_paddle(
    mut query: Query<(&Paddle, &mut Transform)>,
    time_step: Res<Time>,
    windows: Query<&Window>,
    state: Res<PressedState>,
) {
    let sizes: Vec<(f32, f32)> = windows.iter().map(|w| (w.width(), w.height())).collect();
    let (_, height) = sizes[0];

    for (paddle, mut transform) in query.iter_mut() {
        let mut direction = 0.0;
        if state.key_w && paddle.player == Player::One {
            direction = 1.0;
        }
        if state.key_s && paddle.player == Player::One {
            direction = -1.0;
        }
        if state.arrow_up && paddle.player == Player::Two {
            direction = 1.0;
        }
        if state.arrow_down && paddle.player == Player::Two {
            direction = -1.0;
        }

        let new_y = transform.translation.y + direction * PADDLE_SPEED * time_step.delta_seconds();
        let new_y = new_y
            .min(height / 2.0 - PADDLE_HEIGHT / 2.0)
            .max(-height / 2.0 + PADDLE_HEIGHT / 2.0);
        transform.translation.y = new_y;
    }
}

fn apply_velocity(
    mut query: Query<(&mut Ball, &mut Transform)>,
    time_step: Res<Time>,
    windows: Query<&Window>,
    mut score: ResMut<Score>,
) {
    let sizes: Vec<(f32, f32)> = windows.iter().map(|w| (w.width(), w.height())).collect();
    let (width, height) = sizes[0];

    for (mut ball, mut transform) in query.iter_mut() {
        transform.translation += ball.velocity * time_step.delta_seconds();

        if transform.translation.y > height / 2.0 || transform.translation.y < -height / 2.0 {
            ball.velocity.y *= -1.0;
        }

        if transform.translation.x > width / 2.0 {
            transform.translation = Vec3::new(0.0, 0.0, 0.0);
            ball.velocity = Vec3::new(
                random::<f32>() * 2.0 - 1.0,
                random::<f32>() * 2.0 - 1.0,
                0.0,
            )
            .normalize()
                * 200.0;
            score.player_one += 1;
        } else if transform.translation.x < -width / 2.0 {
            transform.translation = Vec3::new(0.0, 0.0, 0.0);
            ball.velocity = Vec3::new(
                random::<f32>() * 2.0 - 1.0,
                random::<f32>() * 2.0 - 1.0,
                0.0,
            )
            .normalize()
                * 200.0;
            score.player_two += 1;
        }
    }
}

fn apply_collision(
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
    paddle_query: Query<(&Paddle, &Transform), Without<Ball>>,
) {
    for (mut ball, ball_transform) in ball_query.iter_mut() {
        for (_, paddle_transform) in paddle_query.iter() {
            if (ball_transform.translation.x < paddle_transform.translation.x + PADDLE_WIDTH / 2.0)
                && (ball_transform.translation.x
                    > paddle_transform.translation.x - PADDLE_WIDTH / 2.0)
                && (ball_transform.translation.y
                    < paddle_transform.translation.y + PADDLE_HEIGHT / 2.0)
                && (ball_transform.translation.y
                    > paddle_transform.translation.y - PADDLE_HEIGHT / 2.0)
            {
                ball.velocity.x *= -1.0;
            }
        }
    }
}

fn display_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{} - {}", score.player_one, score.player_two);
    }
}
