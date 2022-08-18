use bevy::{prelude::*, time::FixedTimestep};
use bevy_kira_audio::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    components::{FromPlayer, Laser, Moveable, Player, SpriteSize, Velocity},
    AudioSources, GameTextures, PlayerState, WinSize, BASE_SPEED, PLAYER_LASER_SIZE,
    PLAYER_RESPAWN_DELAY, PLAYER_SIZE, SPRITE_SCALE, TIME_STEP,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(0.5))
                    .with_system(player_spawn_system),
            )
            .add_system(player_keyboard_event_system)
            .add_system(player_fire_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let now = time.seconds_since_startup();
    let last_shot = player_state.last_shot;

    if !player_state.on && (last_shot == -1. || now > last_shot + PLAYER_RESPAWN_DELAY) {
        // add player
        let bottom = -win_size.h / 2.;
        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.player.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        0.,
                        bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.,
                        10.,
                    ),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..default()
                },
                ..default()
            })
            .insert(Player)
            .insert(SpriteSize::from(PLAYER_SIZE))
            .insert(Moveable { auto_despan: false })
            .insert(Velocity { x: 0., y: 0. });

        player_state.spawned();
    }
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    audio_sources: Res<AudioSources>,
    audio: Res<Audio>,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.pressed(KeyCode::Space) {
            player_state.shooting_timer.tick(time.delta());
        } else {
            player_state.shooting_timer.reset();
        }

        // Fire when space bar initially pressed, and then repeat while being held down
        let should_fire =
            kb.just_pressed(KeyCode::Space) || player_state.shooting_timer.just_finished();

        if (should_fire) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);
            let x_offset = PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 5.;
            let y_offset = 15.;

            let mut spawn_laser = |x_offset: f32| {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: game_textures.player_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(x + x_offset, y + y_offset, 0.),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(FromPlayer)
                    .insert(Laser)
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                    .insert(Moveable { auto_despan: true })
                    .insert(Velocity { x: 0., y: 2. });
            };
            spawn_laser(x_offset);
            spawn_laser(-x_offset);

            audio.play(audio_sources.player_laser.clone());
        }
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.
        } else if kb.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        }
    }
}
