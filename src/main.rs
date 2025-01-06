// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;

fn main() {
    App::new()
        .add_plugins((
            // Embedd assets folder into binary file.
            EmbeddedAssetPlugin {
                mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
            },
            DefaultPlugins
                .set(
                    // Comment this if you're not using pixel art.
                    // This sets image filtering to nearest
                    // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
                    // by linear filtering.
                    ImagePlugin::default_nearest(),
                )
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy project template".into(),
                        // mode: bevy::window::WindowMode::Fullscreen,
                        // resolution: RESOLUTION.into(),
                        ..default()
                    }),
                    ..default()
                }),
            EditorPlugin::default(),
        ))
        .init_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::AssetLoading)
                .continue_to_state(AppState::Main)
                .load_collection::<GameAssets>(),
        )
        .add_systems(Startup, (spawn_2d_camera,))
        .add_systems(OnExit(AppState::AssetLoading), spawn_player)
        .add_systems(Update, move_player.run_if(in_state(AppState::Main)))
        .run();
}

#[derive(AssetCollection, Resource)]
struct GameAssets {
    #[asset(path = "art/ball.png")]
    ball: Handle<Image>,
}

#[derive(States, Clone, Copy, Eq, PartialEq, Debug, Hash, Default, Reflect)]
enum AppState {
    #[default]
    AssetLoading,
    Main,
}

#[derive(Component)]
pub struct Player {
    speed: f32,
}

impl Default for Player {
    fn default() -> Player {
        Player { speed: 500. }
    }
}

fn spawn_2d_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

fn spawn_player(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        Name::new("Player"),
        Player::default(),
        SpriteBundle {
            texture: game_assets.ball.clone(),
            ..default()
        },
    ));
}

fn move_player(
    mut player_q: Query<(&mut Transform, &Player)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in player_q.iter_mut() {
        let mut direction = Vec2::ZERO;
        if keyboard.pressed(KeyCode::KeyW) {
            direction.y += 1.;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction.y -= 1.;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction.x += 1.;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            direction.x -= 1.;
        }

        let direction = direction.normalize_or_zero();
        transform.translation += direction.extend(0.) * player.speed * time.delta_seconds();
    }
}
