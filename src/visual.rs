use bevy::prelude::*;

use crate::components::{
    AttackClock, CharacterVisual, Health, HealthBar, MainCamera, MapEntity, Player,
    ProgressBarFill, ScreenFixed, TimedDespawn,
};
use crate::constants::{PLAYER_START_X, PLAYER_Y};
use crate::data::{
    GameDatabase, MapDefinition, PlayerProfile, RunState, VisualProfile, seed_starting_equipment,
};
use crate::gameplay::begin_current_map;
use crate::ui::spawn_screen_layout;

pub(crate) fn setup(
    mut commands: Commands,
    database: Res<GameDatabase>,
    mut profile: ResMut<PlayerProfile>,
    mut run: ResMut<RunState>,
) {
    commands.spawn((Camera2d, MainCamera));
    spawn_background(&mut commands, &database.maps[run.map_index]);

    let stats = profile.derived_stats(&database);
    let class = profile.class(&database);
    let player = spawn_placeholder_actor(
        &mut commands,
        class.visual,
        Vec3::new(PLAYER_START_X, PLAYER_Y, 4.0),
        (
            Player,
            Health {
                current: stats.max_health,
                max: stats.max_health,
            },
            AttackClock { remaining: 0.0 },
            CharacterVisual {
                base_color: class.visual.color,
            },
        ),
    );
    spawn_health_bar(&mut commands, player, 68.0, 50.0, false);
    spawn_screen_layout(&mut commands);
    seed_starting_equipment(&mut profile, &database);
    begin_current_map(&mut run, &database);
}

fn spawn_background(commands: &mut Commands, map: &MapDefinition) {
    spawn_rect(
        commands,
        map.background,
        Vec2::new(3800.0, 960.0),
        Vec3::new(1150.0, 0.0, -10.0),
    );
    spawn_rect(
        commands,
        Color::srgba(0.09, 0.13, 0.13, 0.85),
        Vec2::new(3900.0, 190.0),
        Vec3::new(1150.0, -355.0, -4.0),
    );

    for index in 0..9 {
        let x = -360.0 + index as f32 * 430.0;
        let height = 150.0 + (index % 3) as f32 * 46.0;
        spawn_rect(
            commands,
            Color::srgba(0.08, 0.17, 0.18, 0.9),
            Vec2::new(250.0, height),
            Vec3::new(x, -145.0 + height * 0.25, -8.0),
        );
    }

    for index in 0..18 {
        let x = -360.0 + index as f32 * 210.0;
        spawn_rect(
            commands,
            Color::srgb(0.14, 0.29, 0.20),
            Vec2::new(160.0, 24.0),
            Vec3::new(x, PLAYER_Y - 48.0, 0.0),
        );
        spawn_rect(
            commands,
            Color::srgb(0.08, 0.12, 0.10),
            Vec2::new(160.0, 28.0),
            Vec3::new(x, PLAYER_Y - 72.0, -1.0),
        );
    }
}

pub(crate) fn spawn_placeholder_actor<T: Bundle>(
    commands: &mut Commands,
    visual: VisualProfile,
    translation: Vec3,
    bundle: T,
) -> Entity {
    let _asset_key = visual.asset_key;
    commands
        .spawn((
            Sprite::from_color(visual.color, visual.size),
            Transform::from_translation(translation),
            bundle,
        ))
        .id()
}

fn spawn_rect(commands: &mut Commands, color: Color, size: Vec2, translation: Vec3) -> Entity {
    commands
        .spawn((
            Sprite::from_color(color, size),
            Transform::from_translation(translation),
        ))
        .id()
}

pub(crate) fn spawn_health_bar(
    commands: &mut Commands,
    target: Entity,
    width: f32,
    y_offset: f32,
    is_map_entity: bool,
) {
    let mut background = commands.spawn((
        Sprite::from_color(Color::srgba(0.03, 0.03, 0.03, 0.95), Vec2::new(width, 6.0)),
        Transform::from_xyz(0.0, 0.0, 18.0),
        HealthBar {
            target,
            width,
            y_offset,
            is_fill: false,
        },
    ));
    if is_map_entity {
        background.insert(MapEntity);
    }

    let mut fill = commands.spawn((
        Sprite::from_color(Color::srgb(0.32, 0.82, 0.34), Vec2::new(width, 6.0)),
        Transform::from_xyz(0.0, 0.0, 19.0),
        HealthBar {
            target,
            width,
            y_offset,
            is_fill: true,
        },
    ));
    if is_map_entity {
        fill.insert(MapEntity);
    }
}

pub(crate) fn spawn_floating_text(
    commands: &mut Commands,
    text: String,
    translation: Vec3,
    color: Color,
) {
    commands.spawn((
        Text2d::new(text),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(Justify::Center),
        bevy::sprite::Anchor::CENTER,
        Transform::from_translation(translation),
        TimedDespawn {
            remaining: 0.75,
            total: 0.75,
            drift_y: 32.0,
        },
        MapEntity,
    ));
}

pub(crate) fn spawn_loot_flash(commands: &mut Commands, translation: Vec3, color: Color) {
    commands.spawn((
        Sprite::from_color(color, Vec2::splat(18.0)),
        Transform {
            translation: translation + Vec3::new(0.0, 28.0, 10.0),
            rotation: Quat::from_rotation_z(0.785),
            ..default()
        },
        TimedDespawn {
            remaining: 1.15,
            total: 1.15,
            drift_y: 10.0,
        },
        MapEntity,
    ));
}

pub(crate) fn sync_character_visuals(mut query: Query<(&mut Sprite, &CharacterVisual, &Health)>) {
    for (mut sprite, visual, health) in &mut query {
        let health_ratio = (health.current / health.max).clamp(0.0, 1.0);
        sprite.color = if health_ratio < 0.35 {
            visual.base_color.mix(&Color::srgb(0.95, 0.22, 0.18), 0.45)
        } else {
            visual.base_color
        };
    }
}

pub(crate) fn sync_health_bars(
    mut commands: Commands,
    health_query: Query<(&Health, &Transform), Without<HealthBar>>,
    mut bar_query: Query<(Entity, &HealthBar, &mut Transform, &mut Sprite)>,
) {
    for (bar_entity, bar, mut transform, mut sprite) in &mut bar_query {
        let Ok((health, target_transform)) = health_query.get(bar.target) else {
            commands.entity(bar_entity).despawn();
            continue;
        };

        let ratio = (health.current / health.max).clamp(0.0, 1.0);
        let width = if bar.is_fill {
            (bar.width * ratio).max(1.0)
        } else {
            bar.width
        };
        sprite.custom_size = Some(Vec2::new(width, 6.0));
        transform.translation.x = target_transform.translation.x - (bar.width - width) * 0.5;
        transform.translation.y = target_transform.translation.y + bar.y_offset;
    }
}

pub(crate) fn tick_timed_entities(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut TimedDespawn,
        Option<&mut Transform>,
        Option<&mut TextColor>,
    )>,
) {
    for (entity, mut timed, transform, text_color) in &mut query {
        timed.remaining -= time.delta_secs();
        if let Some(mut transform) = transform {
            transform.translation.y += timed.drift_y * time.delta_secs();
        }
        if let Some(mut text_color) = text_color {
            let alpha = (timed.remaining / timed.total).clamp(0.0, 1.0);
            text_color.0 = text_color.0.with_alpha(alpha);
        }
        if timed.remaining <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub(crate) fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };
    camera_transform.translation.x = player_transform.translation.x + 125.0;
}

pub(crate) fn sync_screen_fixed_entities(
    camera_query: Query<&Transform, (With<MainCamera>, Without<ScreenFixed>)>,
    mut fixed_query: Query<(&ScreenFixed, &mut Transform), Without<MainCamera>>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };
    for (fixed, mut transform) in &mut fixed_query {
        transform.translation.x = camera_transform.translation.x + fixed.offset.x;
        transform.translation.y = camera_transform.translation.y + fixed.offset.y;
        transform.translation.z = fixed.offset.z;
    }
}

pub(crate) fn sync_progress_bar(
    database: Res<GameDatabase>,
    run: Res<RunState>,
    player_query: Query<&Transform, (With<Player>, Without<ProgressBarFill>)>,
    mut query: Query<(&mut Sprite, &mut Transform), (With<ProgressBarFill>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok((mut sprite, mut transform)) = query.single_mut() else {
        return;
    };

    let map = &database.maps[run.map_index];
    let travel_progress = ((player_transform.translation.x - PLAYER_START_X)
        / (map.finish_x - PLAYER_START_X))
        .clamp(0.0, 1.0);
    let kill_progress = if run.enemies_total == 0 {
        0.0
    } else {
        run.enemies_defeated as f32 / run.enemies_total as f32
    };
    let progress = travel_progress.max(kill_progress).clamp(0.0, 1.0);
    let width = 320.0 * progress;
    sprite.custom_size = Some(Vec2::new(width.max(1.0), 12.0));
    transform.translation.x = transform.translation.x - transform.translation.x.fract();
}
