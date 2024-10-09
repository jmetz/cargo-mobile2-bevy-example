use bevy::window::WindowMode;
use bevy::{color::palettes::css::PURPLE, prelude::*};
use winit::event_loop::EventLoop;

#[cfg(target_os = "android")]
use android_activity::AndroidApp;
#[cfg(target_os = "android")]
use winit::platform::android::EventLoopBuilderExtAndroid;

#[cfg(target_os = "android")]
fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Trace)
            .with_tag("hello-bevy"),
    );
}

#[cfg(not(target_os = "android"))]
fn init_logging() {
    env_logger::init();
}

#[bevy_main]
pub fn main() {
    init_logging();

    App::new()
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)))
        // .init_asset::<bevy::prelude::AudioSource>()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }),))
        .init_resource::<Actions>()
        .add_systems(Startup, (setup, setup_audio, setup_player))
        .add_systems(Update, (update, handle_touch, move_player))
        .run();
}

// INFO: If needed, use the following to have different mobile and desktop
//       setups; remember to call this function in gen/bin/desktop.rs
//
// pub fn main_desktop() {
//     init_logging();
//     App::new()
//         .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)))
//         // .init_asset::<bevy::prelude::AudioSource>()
//         .add_plugins((DefaultPlugins.set(WindowPlugin {
//             primary_window: Some(Window {
//                 resizable: false,
//                 mode: WindowMode::BorderlessFullscreen,
//                 ..default()
//             }),
//             ..default()
//         }),))
//         .init_resource::<Actions>()
//         .add_systems(Startup, (setup, setup_audio, setup_player))
//         .add_systems(Update, (update, handle_touch, move_player))
//         .run();
//}

#[derive(Component)]
pub struct Player;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource, Deref)]
struct MoveSound(Handle<AudioSource>);

fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create sound resource
    let sound = asset_server.load("sounds/boing.ogg");
    commands.insert_resource(MoveSound(sound));
    // PLACEHOLDER: In v0.15 looks like this might be the way to go
    // AudioPlayer::<AudioSource>(asset_server.load("sounds/boing.ogg")),
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("bevy.png"),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        })
        .insert(Player);
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
}

pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
}

impl GameControl {
    pub fn pressed(&self, keyboard_input: &Res<ButtonInput<KeyCode>>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight)
            }
        }
    }
}

pub fn get_movement(control: GameControl, input: &Res<ButtonInput<KeyCode>>) -> f32 {
    if control.pressed(input) {
        1.0
    } else {
        0.0
    }
}

pub const FOLLOW_EPSILON: f32 = 5.;

pub fn handle_touch(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touch_input: Res<Touches>,
    player: Query<&Transform, With<Player>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let mut player_movement = Vec2::new(
        get_movement(GameControl::Right, &keyboard_input)
            - get_movement(GameControl::Left, &keyboard_input),
        get_movement(GameControl::Up, &keyboard_input)
            - get_movement(GameControl::Down, &keyboard_input),
    );

    if let Some(touch_position) = touch_input.first_pressed_position() {
        let (camera, camera_transform) = camera.single();
        if let Some(touch_position) = camera.viewport_to_world_2d(camera_transform, touch_position)
        {
            let diff = touch_position - player.single().translation.xy();
            if diff.length() > FOLLOW_EPSILON {
                player_movement = diff.normalize();
            }
        }
    }

    if player_movement != Vec2::ZERO {
        actions.player_movement = Some(player_movement.normalize());
    } else {
        actions.player_movement = None;
    }
}

fn move_player(
    mut commands: Commands,
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
    audio: Res<MoveSound>,
    audio_control: Query<&AudioSink>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
    // commands.spawn(AudioBundle {
    //     source: audio.clone(),
    //     // auto-despawn the entity when playback finishes
    //     settings: PlaybackSettings::DESPAWN,
    // });
    match audio_control.get_single() {
        // Already playing move sound
        Ok(_) => {}
        Err(_) => {
            commands.spawn((AudioBundle {
                source: audio.clone(),
                settings: PlaybackSettings::DESPAWN,
                ..default()
            },));
        }
    }
}

// PLACEHOLDER: Do other scene updates here
fn update(mut commands: Commands) {}
