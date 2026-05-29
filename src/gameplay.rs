use bevy::prelude::*;

use crate::components::{AttackClock, CharacterVisual, Enemy, Health, MapEntity, Player, UiState};
use crate::constants::{
    ENEMY_ATTACK_RANGE, PLAYER_ATTACK_RANGE, PLAYER_START_X, PLAYER_Y, SPAWN_AHEAD_DISTANCE,
};
use crate::data::{
    GameDatabase, ItemDestination, LootRng, PlayerProfile, RunState, RunStatus, damage_after_armor,
    describe_item, roll_item,
};
use crate::visual::{
    spawn_floating_text, spawn_health_bar, spawn_loot_flash, spawn_placeholder_actor,
};

pub(crate) fn begin_current_map(run: &mut RunState, database: &GameDatabase) {
    let map = &database.maps[run.map_index];
    run.status = RunStatus::Running;
    run.next_pack_index = 0;
    run.enemies_spawned = 0;
    run.enemies_defeated = 0;
    run.enemies_total = map.total_enemies();
    run.transition_remaining = 0.0;
    run.message = format!("{} map opened", map.name);
}

pub(crate) fn handle_map_selection_requests(
    mut ui_state: ResMut<UiState>,
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    mut run: ResMut<RunState>,
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut Health, &mut AttackClock), With<Player>>,
    cleanup_query: Query<Entity, With<MapEntity>>,
) {
    let Some(requested_map_index) = ui_state.requested_map_index.take() else {
        return;
    };

    if requested_map_index >= database.maps.len()
        || !profile.map_unlocked(&database, requested_map_index)
    {
        return;
    }

    for entity in &cleanup_query {
        commands.entity(entity).despawn();
    }

    if let Ok((mut transform, mut health, mut clock)) = player_query.single_mut() {
        let stats = profile.derived_stats(&database);
        transform.translation.x = PLAYER_START_X;
        transform.translation.y = PLAYER_Y;
        health.max = stats.max_health;
        health.current = stats.max_health;
        clock.remaining = 0.0;
    }

    run.map_index = requested_map_index;
    run.atlas_tier = 1;
    begin_current_map(&mut run, &database);
}

pub(crate) fn handle_map_transitions(
    time: Res<Time>,
    database: Res<GameDatabase>,
    mut run: ResMut<RunState>,
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut Health, &mut AttackClock), With<Player>>,
    cleanup_query: Query<Entity, With<MapEntity>>,
    mut profile: ResMut<PlayerProfile>,
) {
    if run.status == RunStatus::Running {
        return;
    }

    run.transition_remaining -= time.delta_secs();
    if run.transition_remaining > 0.0 {
        return;
    }

    if run.status == RunStatus::Cleared {
        run.map_index = (run.map_index + 1).min(profile.highest_unlocked_map_index(&database));
        run.atlas_tier = 1;
    }

    for entity in &cleanup_query {
        commands.entity(entity).despawn();
    }

    if let Ok((mut transform, mut health, mut clock)) = player_query.single_mut() {
        let stats = profile.derived_stats(&database);
        transform.translation.x = PLAYER_START_X;
        transform.translation.y = PLAYER_Y;
        health.max = stats.max_health;
        health.current = stats.max_health;
        clock.remaining = 0.0;
    }

    if run.status == RunStatus::Dead {
        profile.respawns += 1;
    }

    begin_current_map(&mut run, &database);
}

pub(crate) fn spawn_enemy_packs(
    mut commands: Commands,
    database: Res<GameDatabase>,
    mut run: ResMut<RunState>,
    player_query: Query<&Transform, With<Player>>,
) {
    if run.status != RunStatus::Running {
        return;
    }

    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let map = &database.maps[run.map_index];

    while let Some(pack) = map.packs.get(run.next_pack_index) {
        if pack.spawn_x > player_transform.translation.x + SPAWN_AHEAD_DISTANCE {
            break;
        }

        let archetype = pack.kind.archetype();
        for index in 0..pack.count {
            let spawn_x = pack.spawn_x + index as f32 * 54.0;
            let tier_scale =
                1.0 + (run.atlas_tier - 1) as f32 * 0.28 + (map.area_level - 1) as f32 * 0.16;
            let enemy_id = run.next_enemy_id;
            run.next_enemy_id += 1;
            let enemy = spawn_placeholder_actor(
                &mut commands,
                archetype.visual,
                Vec3::new(spawn_x, PLAYER_Y + 4.0, 3.0),
                (
                    Enemy {
                        id: enemy_id,
                        name: archetype.name,
                        gold_reward: (archetype.gold_reward as f32 * tier_scale) as u32,
                        xp_reward: (archetype.xp_reward as f32 * tier_scale) as u32,
                        item_chance: archetype.item_chance,
                        damage: archetype.damage * tier_scale,
                        armor: archetype.armor * tier_scale,
                        attacks_per_second: archetype.attacks_per_second,
                        move_speed: archetype.move_speed,
                    },
                    Health {
                        current: archetype.max_health * tier_scale,
                        max: archetype.max_health * tier_scale,
                    },
                    AttackClock { remaining: 0.45 },
                    CharacterVisual {
                        base_color: archetype.visual.color,
                    },
                    MapEntity,
                ),
            );
            spawn_health_bar(
                &mut commands,
                enemy,
                archetype.visual.size.x + 12.0,
                42.0,
                true,
            );
        }

        run.enemies_spawned += pack.count;
        run.next_pack_index += 1;
    }
}

pub(crate) fn move_player(
    time: Res<Time>,
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    run: Res<RunState>,
    mut player_query: Query<&mut Transform, With<Player>>,
    enemy_query: Query<(&Transform, &Health), (With<Enemy>, Without<Player>)>,
) {
    if run.status != RunStatus::Running {
        return;
    }

    let Ok(mut player_transform) = player_query.single_mut() else {
        return;
    };

    let enemy_blocks_path = enemy_query.iter().any(|(enemy_transform, health)| {
        health.current > 0.0
            && enemy_transform.translation.x > player_transform.translation.x - 8.0
            && enemy_transform.translation.x - player_transform.translation.x < PLAYER_ATTACK_RANGE
    });

    if !enemy_blocks_path {
        let stats = profile.derived_stats(&database);
        player_transform.translation.x += stats.move_speed * time.delta_secs();
    }
}

pub(crate) fn regenerate_player_health(
    time: Res<Time>,
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    run: Res<RunState>,
    mut player_query: Query<&mut Health, With<Player>>,
) {
    let Ok(mut health) = player_query.single_mut() else {
        return;
    };

    let stats = profile.derived_stats(&database);
    health.max = stats.max_health;
    health.current = health.current.min(health.max);

    if run.status != RunStatus::Running || health.current <= 0.0 {
        return;
    }

    health.current =
        (health.current + stats.health_regeneration * time.delta_secs()).min(health.max);
}

pub(crate) fn move_enemies(
    time: Res<Time>,
    run: Res<RunState>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &Enemy, &Health), Without<Player>>,
) {
    if run.status != RunStatus::Running {
        return;
    }

    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (mut enemy_transform, enemy, health) in &mut enemy_query {
        if health.current <= 0.0 {
            continue;
        }
        let distance = enemy_transform.translation.x - player_transform.translation.x;
        if distance > ENEMY_ATTACK_RANGE {
            enemy_transform.translation.x -= enemy.move_speed * time.delta_secs();
        }
        if enemy_transform.translation.x < player_transform.translation.x + 32.0 {
            enemy_transform.translation.x = player_transform.translation.x + 32.0;
        }
    }
}

pub(crate) fn player_attack(
    time: Res<Time>,
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    run: Res<RunState>,
    mut rng: ResMut<LootRng>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut AttackClock), With<Player>>,
    mut enemy_targets: ParamSet<(
        Query<(Entity, &Transform, &Health), With<Enemy>>,
        Query<(&Transform, &mut Health, &Enemy), With<Enemy>>,
    )>,
) {
    if run.status != RunStatus::Running {
        return;
    }

    let Ok((player_transform, mut clock)) = player_query.single_mut() else {
        return;
    };
    clock.remaining = (clock.remaining - time.delta_secs()).max(0.0);
    if clock.remaining > 0.0 {
        return;
    }

    let mut target = None;
    let mut best_distance = f32::MAX;
    for (entity, enemy_transform, health) in &enemy_targets.p0() {
        let distance = enemy_transform.translation.x - player_transform.translation.x;
        if health.current > 0.0
            && (0.0..=PLAYER_ATTACK_RANGE).contains(&distance)
            && distance < best_distance
        {
            target = Some(entity);
            best_distance = distance;
        }
    }

    let Some(target) = target else {
        return;
    };

    let stats = profile.derived_stats(&database);
    if let Ok((enemy_transform, mut health, enemy)) = enemy_targets.p1().get_mut(target) {
        let is_crit = rng.percent() <= stats.crit_chance;
        let raw_damage = if is_crit {
            stats.damage * (1.0 + stats.crit_damage / 100.0)
        } else {
            stats.damage
        };
        let damage = damage_after_armor(raw_damage, enemy.armor);
        health.current -= damage;
        clock.remaining = 1.0 / stats.attacks_per_second;
        spawn_floating_text(
            &mut commands,
            if is_crit {
                format!("CRIT -{damage:.0}")
            } else {
                format!("-{damage:.0}")
            },
            enemy_transform.translation + Vec3::new(0.0, 46.0, 20.0),
            if is_crit {
                Color::srgb(1.0, 0.96, 0.32)
            } else {
                Color::srgb(1.0, 0.83, 0.45)
            },
        );
    }
}

pub(crate) fn enemies_attack(
    time: Res<Time>,
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    run: Res<RunState>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Health), With<Player>>,
    mut enemy_query: Query<(&Transform, &Enemy, &Health, &mut AttackClock), Without<Player>>,
) {
    if run.status != RunStatus::Running {
        return;
    }

    let Ok((player_transform, mut player_health)) = player_query.single_mut() else {
        return;
    };
    let player_stats = profile.derived_stats(&database);

    for (enemy_transform, enemy, enemy_health, mut clock) in &mut enemy_query {
        if enemy_health.current <= 0.0 {
            continue;
        }
        clock.remaining = (clock.remaining - time.delta_secs()).max(0.0);
        let distance = enemy_transform.translation.x - player_transform.translation.x;
        if distance > ENEMY_ATTACK_RANGE || clock.remaining > 0.0 {
            continue;
        }

        let damage = damage_after_armor(enemy.damage, player_stats.armor);
        player_health.current -= damage;
        clock.remaining = 1.0 / enemy.attacks_per_second;
        spawn_floating_text(
            &mut commands,
            format!("-{damage:.0}"),
            player_transform.translation + Vec3::new(0.0, 58.0, 20.0),
            Color::srgb(1.0, 0.35, 0.35),
        );
    }
}

pub(crate) fn resolve_combat_outcomes(
    database: Res<GameDatabase>,
    mut profile: ResMut<PlayerProfile>,
    mut run: ResMut<RunState>,
    mut rng: ResMut<LootRng>,
    mut commands: Commands,
    enemy_query: Query<(Entity, &Enemy, &Health, &Transform), Without<Player>>,
    mut player_query: Query<&mut Health, With<Player>>,
) {
    if run.status != RunStatus::Running {
        return;
    }

    let mut leveled = false;
    for (entity, enemy, health, transform) in &enemy_query {
        if health.current > 0.0 {
            continue;
        }

        commands.entity(entity).despawn();
        run.enemies_defeated += 1;
        profile.gold += enemy.gold_reward;
        leveled |= profile.gain_xp(enemy.xp_reward, &database);
        run.message = format!(
            "{} #{} defeated: +{} gold",
            enemy.name, enemy.id, enemy.gold_reward
        );
        if let Ok(mut player_health) = player_query.single_mut() {
            let recovery = (player_health.max * 0.045).max(6.0);
            player_health.current = (player_health.current + recovery).min(player_health.max);
            spawn_floating_text(
                &mut commands,
                format!("+{recovery:.0}hp"),
                transform.translation + Vec3::new(0.0, 92.0, 20.0),
                Color::srgb(0.36, 1.0, 0.48),
            );
        }
        spawn_floating_text(
            &mut commands,
            format!("+{}g", enemy.gold_reward),
            transform.translation + Vec3::new(0.0, 72.0, 20.0),
            Color::srgb(0.95, 0.71, 0.28),
        );

        let loot_chance = enemy.item_chance + profile.derived_stats(&database).loot_bonus;
        if rng.percent() <= loot_chance {
            let item = roll_item(
                &mut rng,
                &database,
                run.atlas_tier + database.maps[run.map_index].area_level,
            );
            let item_name = describe_item(&item, &database);
            let item_color = database.items[item.def_id].tint;
            let destination = profile.add_item(item);
            run.message = match destination {
                ItemDestination::Inventory => format!("Looted {item_name}"),
                ItemDestination::Stash => format!("Stashed {item_name}"),
                ItemDestination::Lost => format!("No room for {item_name}"),
            };
            spawn_loot_flash(&mut commands, transform.translation, item_color);
        }
    }

    if leveled {
        if let Ok(mut player_health) = player_query.single_mut() {
            let stats = profile.derived_stats(&database);
            player_health.max = stats.max_health;
            player_health.current =
                (player_health.current + stats.max_health * 0.35).min(stats.max_health);
        }
        run.message = format!("Level {} reached", profile.level);
    }

    if let Ok(player_health) = player_query.single() {
        if player_health.current <= 0.0 {
            run.status = RunStatus::Dead;
            run.transition_remaining = 2.3;
            run.message = "Defeated. Rebuilding the map".into();
            return;
        }
    }

    if run.enemies_defeated >= run.enemies_total
        && run.next_pack_index >= database.maps[run.map_index].packs.len()
    {
        let cleared_map_index = run.map_index;
        let unlocked_map_index = profile.unlock_next_map(&database, cleared_map_index);
        run.status = RunStatus::Cleared;
        run.transition_remaining = 2.0;
        run.message = if let Some(unlocked_map_index) = unlocked_map_index {
            format!(
                "{} cleared. {} unlocked",
                database.maps[cleared_map_index].name, database.maps[unlocked_map_index].name
            )
        } else {
            format!("{} cleared", database.maps[cleared_map_index].name)
        };
    }
}
