use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::components::{
    ActivePanel, BottomButton, BottomButtonLabel, CharacterPanelPiece, CharacterPanelText,
    DraggedItem, DraggedItemVisual, Health, HudText, InventoryCell, InventoryCellLabel,
    InventoryPanelPiece, InventorySource, ItemTooltipBackground, ItemTooltipText, Player,
    ProgressBarFill, ScreenFixed, UiState,
};
use crate::constants::{
    BOTTOM_BUTTON_SIZE, INVENTORY_CELL_SIZE, TOOLTIP_PADDING, TOOLTIP_WIDTH, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};
use crate::data::{
    GameDatabase, ItemInstance, ItemLocation, ItemSlot, PlayerProfile, Rarity, RunState,
    TalentGrant, item_armor_bonus, item_attack_speed_bonus, item_damage_bonus,
    item_health_regen_bonus, item_life_bonus, item_move_speed_bonus, item_slot_effect,
    rarity_color, rarity_effect,
};

pub(crate) fn spawn_screen_layout(commands: &mut Commands) {
    spawn_fixed_rect(
        commands,
        Vec3::new(0.0, 420.0, 30.0),
        Vec2::new(668.0, 92.0),
        Color::srgba(0.08, 0.08, 0.09, 0.93),
    );
    spawn_fixed_rect(
        commands,
        Vec3::new(0.0, 350.0, 30.0),
        Vec2::new(668.0, 34.0),
        Color::srgba(0.25, 0.13, 0.08, 0.92),
    );
    spawn_inventory_panel_rect(
        commands,
        Vec3::new(0.0, -342.0, 30.0),
        Vec2::new(668.0, 242.0),
        Color::srgba(0.07, 0.07, 0.08, 0.94),
    );
    spawn_fixed_rect(
        commands,
        Vec3::new(-168.0, 322.0, 31.0),
        Vec2::new(320.0, 12.0),
        Color::srgba(0.02, 0.02, 0.02, 0.90),
    );

    commands.spawn((
        Sprite::from_color(Color::srgb(0.94, 0.66, 0.22), Vec2::new(1.0, 12.0)),
        Transform::from_xyz(-328.0, 322.0, 32.0),
        ScreenFixed {
            offset: Vec3::new(-328.0, 322.0, 32.0),
        },
        ProgressBarFill,
    ));

    spawn_fixed_text(
        commands,
        HudText::Header,
        Vec3::new(-315.0, 448.0, 35.0),
        18.0,
    );
    spawn_fixed_text(
        commands,
        HudText::Stats,
        Vec3::new(-315.0, 405.0, 35.0),
        15.0,
    );
    spawn_fixed_text(
        commands,
        HudText::Message,
        Vec3::new(-315.0, 360.0, 35.0),
        16.0,
    );

    spawn_inventory_panel_label(commands, "Inventory", Vec3::new(-290.0, -236.0, 35.0), 17.0);
    spawn_inventory_panel_label(commands, "Stash", Vec3::new(60.0, -236.0, 35.0), 17.0);
    spawn_inventory_panel_label(commands, "Equipped", Vec3::new(60.0, -384.0, 35.0), 15.0);

    spawn_inventory_cells(
        commands,
        InventorySource::Inventory,
        -292.0,
        -282.0,
        6,
        4,
        42.0,
    );
    spawn_inventory_cells(commands, InventorySource::Stash, 60.0, -282.0, 5, 3, 42.0);
    spawn_inventory_cells(
        commands,
        InventorySource::Equipment,
        60.0,
        -414.0,
        4,
        2,
        38.0,
    );
    spawn_bottom_button(
        commands,
        ActivePanel::Inventory,
        "Inventory",
        Vec3::new(-292.0, -447.0, 35.0),
    );
    spawn_bottom_button(
        commands,
        ActivePanel::Character,
        "Character",
        Vec3::new(-164.0, -447.0, 35.0),
    );
    spawn_character_panel(commands);
    spawn_item_tooltip(commands);
    spawn_dragged_item_visual(commands);
}

fn spawn_inventory_cells(
    commands: &mut Commands,
    source: InventorySource,
    start_x: f32,
    start_y: f32,
    columns: usize,
    rows: usize,
    step: f32,
) {
    for row in 0..rows {
        for column in 0..columns {
            let index = row * columns + column;
            let offset = Vec3::new(
                start_x + column as f32 * step,
                start_y - row as f32 * step,
                34.0,
            );
            commands.spawn((
                Sprite::from_color(
                    Color::srgba(0.10, 0.10, 0.11, 0.98),
                    Vec2::splat(INVENTORY_CELL_SIZE),
                ),
                Transform::from_translation(offset),
                Visibility::Visible,
                ScreenFixed { offset },
                InventoryCell { source, index },
                InventoryPanelPiece,
            ));

            commands.spawn((
                Text2d::new(""),
                TextFont {
                    font_size: 9.0,
                    ..default()
                },
                TextColor(Color::srgb(0.18, 0.16, 0.13)),
                TextLayout::new_with_justify(Justify::Center),
                Anchor::CENTER,
                Transform::from_translation(offset + Vec3::new(0.0, 0.0, 1.0)),
                Visibility::Visible,
                ScreenFixed {
                    offset: offset + Vec3::new(0.0, 0.0, 1.0),
                },
                InventoryCellLabel { source, index },
                InventoryPanelPiece,
            ));
        }
    }
}

fn spawn_fixed_rect(commands: &mut Commands, offset: Vec3, size: Vec2, color: Color) {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(offset),
        ScreenFixed { offset },
    ));
}

fn spawn_inventory_panel_rect(commands: &mut Commands, offset: Vec3, size: Vec2, color: Color) {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(offset),
        Visibility::Visible,
        ScreenFixed { offset },
        InventoryPanelPiece,
    ));
}

fn spawn_fixed_text(commands: &mut Commands, kind: HudText, offset: Vec3, font_size: f32) {
    commands.spawn((
        Text2d::new(""),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(Color::srgb(0.92, 0.89, 0.80)),
        TextLayout::new_with_justify(Justify::Left),
        Anchor::TOP_LEFT,
        Transform::from_translation(offset),
        ScreenFixed { offset },
        kind,
    ));
}

fn spawn_inventory_panel_label(
    commands: &mut Commands,
    label: &'static str,
    offset: Vec3,
    font_size: f32,
) {
    commands.spawn((
        Text2d::new(label),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(Color::srgb(0.96, 0.70, 0.32)),
        TextLayout::new_with_justify(Justify::Left),
        Anchor::TOP_LEFT,
        Transform::from_translation(offset),
        Visibility::Visible,
        ScreenFixed { offset },
        InventoryPanelPiece,
    ));
}

fn spawn_bottom_button(
    commands: &mut Commands,
    panel: ActivePanel,
    label: &'static str,
    offset: Vec3,
) {
    commands.spawn((
        Sprite::from_color(Color::srgba(0.19, 0.11, 0.07, 0.98), BOTTOM_BUTTON_SIZE),
        Transform::from_translation(offset),
        ScreenFixed { offset },
        BottomButton {
            panel,
            size: BOTTOM_BUTTON_SIZE,
        },
    ));

    let text_offset = offset + Vec3::new(-48.0, 9.0, 1.0);
    commands.spawn((
        Text2d::new(label),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.96, 0.70, 0.32)),
        TextLayout::new_with_justify(Justify::Left),
        Anchor::TOP_LEFT,
        Transform::from_translation(text_offset),
        ScreenFixed {
            offset: text_offset,
        },
        BottomButtonLabel { panel },
    ));
}

fn spawn_character_panel(commands: &mut Commands) {
    spawn_character_panel_rect(
        commands,
        Vec3::new(0.0, 96.0, 42.0),
        Vec2::new(650.0, 420.0),
        Color::srgba(0.06, 0.06, 0.065, 0.97),
    );
    spawn_character_panel_rect(
        commands,
        Vec3::new(0.0, 279.0, 43.0),
        Vec2::new(620.0, 42.0),
        Color::srgba(0.25, 0.13, 0.08, 0.95),
    );
    spawn_character_panel_rect(
        commands,
        Vec3::new(-164.0, 101.0, 43.0),
        Vec2::new(288.0, 306.0),
        Color::srgba(0.10, 0.10, 0.11, 0.95),
    );
    spawn_character_panel_rect(
        commands,
        Vec3::new(164.0, 101.0, 43.0),
        Vec2::new(288.0, 306.0),
        Color::srgba(0.10, 0.10, 0.11, 0.95),
    );

    spawn_character_panel_text(
        commands,
        CharacterPanelText::Header,
        Vec3::new(-298.0, 295.0, 44.0),
        18.0,
        Color::srgb(0.96, 0.70, 0.32),
    );
    spawn_character_panel_text(
        commands,
        CharacterPanelText::Status,
        Vec3::new(-292.0, 244.0, 44.0),
        13.0,
        Color::srgb(0.92, 0.89, 0.80),
    );
    spawn_character_panel_text(
        commands,
        CharacterPanelText::Combat,
        Vec3::new(-292.0, 118.0, 44.0),
        13.0,
        Color::srgb(0.92, 0.89, 0.80),
    );
    spawn_character_panel_text(
        commands,
        CharacterPanelText::Attributes,
        Vec3::new(-292.0, 28.0, 44.0),
        13.0,
        Color::srgb(0.92, 0.89, 0.80),
    );
    spawn_character_panel_text(
        commands,
        CharacterPanelText::Equipment,
        Vec3::new(36.0, 24.0, 44.0),
        13.0,
        Color::srgb(0.92, 0.89, 0.80),
    );
    spawn_character_panel_text(
        commands,
        CharacterPanelText::Talents,
        Vec3::new(36.0, 244.0, 44.0),
        14.0,
        Color::srgb(0.92, 0.89, 0.80),
    );
    spawn_character_panel_text(
        commands,
        CharacterPanelText::Upgrades,
        Vec3::new(36.0, 166.0, 44.0),
        13.0,
        Color::srgb(0.92, 0.89, 0.80),
    );
}

fn spawn_character_panel_rect(commands: &mut Commands, offset: Vec3, size: Vec2, color: Color) {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(offset),
        Visibility::Hidden,
        ScreenFixed { offset },
        CharacterPanelPiece,
    ));
}

fn spawn_character_panel_text(
    commands: &mut Commands,
    kind: CharacterPanelText,
    offset: Vec3,
    font_size: f32,
    color: Color,
) {
    commands.spawn((
        Text2d::new(""),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(Justify::Left),
        Anchor::TOP_LEFT,
        Transform::from_translation(offset),
        Visibility::Hidden,
        ScreenFixed { offset },
        CharacterPanelPiece,
        kind,
    ));
}

fn spawn_item_tooltip(commands: &mut Commands) {
    let background_offset = Vec3::new(0.0, 0.0, 56.0);
    let text_offset = Vec3::new(0.0, 0.0, 57.0);

    commands.spawn((
        Sprite::from_color(
            Color::srgba(0.05, 0.045, 0.04, 0.96),
            Vec2::new(TOOLTIP_WIDTH, 120.0),
        ),
        Transform::from_translation(background_offset),
        Visibility::Hidden,
        ScreenFixed {
            offset: background_offset,
        },
        ItemTooltipBackground,
    ));

    commands.spawn((
        Text2d::new(""),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.93, 0.90, 0.82)),
        TextLayout::new_with_justify(Justify::Left),
        Anchor::TOP_LEFT,
        Transform::from_translation(text_offset),
        Visibility::Hidden,
        ScreenFixed {
            offset: text_offset,
        },
        ItemTooltipText,
    ));
}

fn spawn_dragged_item_visual(commands: &mut Commands) {
    let offset = Vec3::new(0.0, 0.0, 58.0);
    commands.spawn((
        Sprite::from_color(
            Color::srgba(0.72, 0.72, 0.68, 0.92),
            Vec2::splat(INVENTORY_CELL_SIZE * 0.88),
        ),
        Transform::from_translation(offset),
        Visibility::Hidden,
        ScreenFixed { offset },
        DraggedItemVisual,
    ));
}

pub(crate) fn handle_bottom_buttons(
    mut ui_state: ResMut<UiState>,
    mouse: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window>,
    mut button_query: Query<(&BottomButton, &ScreenFixed, &mut Sprite)>,
    mut label_query: Query<(&BottomButtonLabel, &mut TextColor)>,
) {
    let cursor_offset = window_query.single().ok().and_then(|window| {
        window.cursor_position().map(|position| {
            Vec2::new(
                position.x - WINDOW_WIDTH as f32 * 0.5,
                WINDOW_HEIGHT as f32 * 0.5 - position.y,
            )
        })
    });
    let mut next_panel = ui_state.active_panel;

    for (button, fixed, mut sprite) in &mut button_query {
        let hovered = cursor_offset.is_some_and(|cursor| {
            let half_size = button.size * 0.5;
            (cursor.x - fixed.offset.x).abs() <= half_size.x
                && (cursor.y - fixed.offset.y).abs() <= half_size.y
        });

        if hovered && mouse.just_pressed(MouseButton::Left) {
            next_panel = if ui_state.active_panel == button.panel {
                ActivePanel::None
            } else {
                button.panel
            };
        }

        let active = next_panel == button.panel;
        sprite.color = if active {
            Color::srgba(0.55, 0.28, 0.10, 0.98)
        } else if hovered {
            Color::srgba(0.34, 0.18, 0.09, 0.98)
        } else {
            Color::srgba(0.19, 0.11, 0.07, 0.98)
        };
    }

    ui_state.active_panel = next_panel;

    for (label, mut text_color) in &mut label_query {
        text_color.0 = if ui_state.active_panel == label.panel {
            Color::srgb(1.0, 0.82, 0.42)
        } else {
            Color::srgb(0.96, 0.70, 0.32)
        };
    }
}

pub(crate) fn sync_inventory_panel(
    ui_state: Res<UiState>,
    mut visibility_query: Query<&mut Visibility, With<InventoryPanelPiece>>,
) {
    let is_visible = ui_state.active_panel == ActivePanel::Inventory;
    for mut visibility in &mut visibility_query {
        *visibility = if is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub(crate) fn handle_inventory_input(
    mut ui_state: ResMut<UiState>,
    database: Res<GameDatabase>,
    mut profile: ResMut<PlayerProfile>,
    mouse: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window>,
    cell_query: Query<(&InventoryCell, &ScreenFixed)>,
) {
    if ui_state.active_panel != ActivePanel::Inventory {
        ui_state.dragged_item = None;
        return;
    }

    let Some(cursor_offset) = cursor_offset(&window_query) else {
        if mouse.just_released(MouseButton::Left) {
            ui_state.dragged_item = None;
        }
        return;
    };
    let hovered_cell = hovered_inventory_cell(cursor_offset, &cell_query);

    if mouse.just_pressed(MouseButton::Right) {
        if let Some((source, index)) = hovered_cell {
            ui_state.dragged_item = None;
            profile.use_item_at(item_location(source, index), &database);
        }
        return;
    }

    if mouse.just_pressed(MouseButton::Left) {
        ui_state.dragged_item = hovered_cell.and_then(|(source, index)| {
            profile
                .item_at(item_location(source, index))
                .cloned()
                .map(|item| DraggedItem {
                    source,
                    index,
                    item,
                })
        });
    }

    if mouse.just_released(MouseButton::Left) {
        if let Some(dragged_item) = ui_state.dragged_item.take() {
            if let Some((target_source, target_index)) = hovered_cell {
                profile.move_item(
                    item_location(dragged_item.source, dragged_item.index),
                    item_location(target_source, target_index),
                    &database,
                );
            }
        }
    }
}

pub(crate) fn sync_dragged_item_visual(
    database: Res<GameDatabase>,
    ui_state: Res<UiState>,
    window_query: Query<&Window>,
    mut query: Query<(&mut ScreenFixed, &mut Sprite, &mut Visibility), With<DraggedItemVisual>>,
) {
    let Ok((mut fixed, mut sprite, mut visibility)) = query.single_mut() else {
        return;
    };
    let Some(dragged_item) = &ui_state.dragged_item else {
        *visibility = Visibility::Hidden;
        return;
    };
    let Some(cursor_offset) = cursor_offset(&window_query) else {
        *visibility = Visibility::Hidden;
        return;
    };
    if ui_state.active_panel != ActivePanel::Inventory {
        *visibility = Visibility::Hidden;
        return;
    }

    fixed.offset = Vec3::new(cursor_offset.x, cursor_offset.y, fixed.offset.z);
    sprite.color = item_cell_color(&dragged_item.item, &database);
    *visibility = Visibility::Visible;
}

pub(crate) fn sync_character_panel(
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    run: Res<RunState>,
    ui_state: Res<UiState>,
    player_query: Query<&Health, With<Player>>,
    mut visibility_query: Query<&mut Visibility, With<CharacterPanelPiece>>,
    mut text_query: Query<(&CharacterPanelText, &mut Text2d)>,
) {
    let is_visible = ui_state.active_panel == ActivePanel::Character;
    for mut visibility in &mut visibility_query {
        *visibility = if is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    if !is_visible {
        return;
    }

    let map = &database.maps[run.map_index];
    let class = profile.class(&database);
    let stats = profile.derived_stats(&database);
    let attributes = profile.attributes(&database);
    let health_text = player_query
        .single()
        .map(|health| format!("{:.0}/{:.0}", health.current.max(0.0), health.max))
        .unwrap_or_else(|_| "--".into());

    for (kind, mut text) in &mut text_query {
        text.0 = match kind {
            CharacterPanelText::Header => format!("Character Profile  |  {}", class.name),
            CharacterPanelText::Status => format!(
                "Status\nLevel {}\nEXP {}/{}\nHP {}\nGold {}\nMap {} {}/{}\nRespawns {}",
                profile.level,
                profile.xp,
                profile.xp_to_next_level(),
                health_text,
                profile.gold,
                map.name,
                run.enemies_defeated,
                run.enemies_total,
                profile.respawns,
            ),
            CharacterPanelText::Combat => format!(
                "Combat\nDamage {:.0}\nArmor {:.0}\nAttacks/sec {:.2}\nMove speed {:.0}\nRegen {:.1}/s\nLoot bonus +{:.0}%",
                stats.damage,
                stats.armor,
                stats.attacks_per_second,
                stats.move_speed,
                stats.health_regeneration,
                stats.loot_bonus,
            ),
            CharacterPanelText::Attributes => format!(
                "Attributes\nStrength {}\nDexterity {}\nIntelligence {}\nVitality {}",
                attributes.strength,
                attributes.dexterity,
                attributes.intelligence,
                attributes.vitality
            ),
            CharacterPanelText::Equipment => equipment_summary(&profile, &database),
            CharacterPanelText::Talents => talent_summary(&profile, &database),
            CharacterPanelText::Upgrades => upgrade_summary(&profile, &database),
        };
    }
}

pub(crate) fn update_item_tooltip(
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    ui_state: Res<UiState>,
    window_query: Query<&Window>,
    cell_query: Query<
        (&InventoryCell, &ScreenFixed),
        (Without<ItemTooltipBackground>, Without<ItemTooltipText>),
    >,
    mut background_query: Query<
        (&mut ScreenFixed, &mut Sprite, &mut Visibility),
        (With<ItemTooltipBackground>, Without<ItemTooltipText>),
    >,
    mut text_query: Query<
        (
            &mut ScreenFixed,
            &mut Text2d,
            &mut TextColor,
            &mut Visibility,
        ),
        (With<ItemTooltipText>, Without<ItemTooltipBackground>),
    >,
) {
    let Ok((mut background_fixed, mut background_sprite, mut background_visibility)) =
        background_query.single_mut()
    else {
        return;
    };
    let Ok((mut text_fixed, mut tooltip_text, mut tooltip_color, mut text_visibility)) =
        text_query.single_mut()
    else {
        return;
    };

    if ui_state.active_panel != ActivePanel::Inventory || ui_state.dragged_item.is_some() {
        *background_visibility = Visibility::Hidden;
        *text_visibility = Visibility::Hidden;
        return;
    }

    let Ok(window) = window_query.single() else {
        *background_visibility = Visibility::Hidden;
        *text_visibility = Visibility::Hidden;
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        *background_visibility = Visibility::Hidden;
        *text_visibility = Visibility::Hidden;
        return;
    };

    let cursor_offset = Vec2::new(
        cursor_position.x - WINDOW_WIDTH as f32 * 0.5,
        WINDOW_HEIGHT as f32 * 0.5 - cursor_position.y,
    );
    let hovered_item = cell_query.iter().find_map(|(cell, fixed)| {
        let half_cell = INVENTORY_CELL_SIZE * 0.5;
        let within_x = (cursor_offset.x - fixed.offset.x).abs() <= half_cell;
        let within_y = (cursor_offset.y - fixed.offset.y).abs() <= half_cell;
        if within_x && within_y {
            item_for_cell(cell, &profile)
        } else {
            None
        }
    });

    let Some(item) = hovered_item else {
        *background_visibility = Visibility::Hidden;
        *text_visibility = Visibility::Hidden;
        return;
    };

    tooltip_text.0 = item_tooltip_text(item, &database);
    let line_count = tooltip_text.0.lines().count() as f32;
    let tooltip_height = (line_count * 17.0 + TOOLTIP_PADDING * 2.0).max(110.0);
    let mut top_left = cursor_offset + Vec2::new(18.0, -18.0);
    let right_edge = WINDOW_WIDTH as f32 * 0.5 - TOOLTIP_PADDING;
    let left_edge = -(WINDOW_WIDTH as f32) * 0.5 + TOOLTIP_PADDING;
    let top_edge = WINDOW_HEIGHT as f32 * 0.5 - TOOLTIP_PADDING;
    let bottom_edge = -(WINDOW_HEIGHT as f32) * 0.5 + TOOLTIP_PADDING;

    if top_left.x + TOOLTIP_WIDTH > right_edge {
        top_left.x = cursor_offset.x - TOOLTIP_WIDTH - 18.0;
    }
    if top_left.x < left_edge {
        top_left.x = left_edge;
    }
    if top_left.y > top_edge {
        top_left.y = top_edge;
    }
    if top_left.y - tooltip_height < bottom_edge {
        top_left.y = bottom_edge + tooltip_height;
    }

    let rarity_tint = rarity_color(item.rarity);
    background_sprite.custom_size = Some(Vec2::new(TOOLTIP_WIDTH, tooltip_height));
    background_sprite.color = rarity_tint.mix(&Color::srgba(0.03, 0.025, 0.025, 0.96), 0.82);
    background_fixed.offset = Vec3::new(
        top_left.x + TOOLTIP_WIDTH * 0.5,
        top_left.y - tooltip_height * 0.5,
        background_fixed.offset.z,
    );
    text_fixed.offset = Vec3::new(
        top_left.x + TOOLTIP_PADDING,
        top_left.y - TOOLTIP_PADDING,
        text_fixed.offset.z,
    );
    tooltip_color.0 = Color::srgb(0.95, 0.92, 0.84);
    *background_visibility = Visibility::Visible;
    *text_visibility = Visibility::Visible;
}

pub(crate) fn sync_inventory_grid(
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    ui_state: Res<UiState>,
    mut query: Query<(&InventoryCell, &mut Sprite)>,
    mut label_query: Query<(&InventoryCellLabel, &mut Text2d, &mut TextColor)>,
) {
    for (cell, mut sprite) in &mut query {
        let item = item_for_cell(cell, &profile);
        let is_drag_source = ui_state.dragged_item.as_ref().is_some_and(|dragged_item| {
            dragged_item.source == cell.source && dragged_item.index == cell.index
        });

        let color = if let Some(item) = item {
            item_cell_color(item, &database)
        } else {
            Color::srgba(0.10, 0.10, 0.11, 0.98)
        };
        sprite.color = if is_drag_source {
            color.mix(&Color::srgba(0.02, 0.02, 0.02, 0.98), 0.45)
        } else {
            color
        };
    }

    for (label, mut text, mut text_color) in &mut label_query {
        let cell = InventoryCell {
            source: label.source,
            index: label.index,
        };
        let item = item_for_cell(&cell, &profile);
        let is_drag_source = ui_state.dragged_item.as_ref().is_some_and(|dragged_item| {
            dragged_item.source == label.source && dragged_item.index == label.index
        });

        if is_drag_source {
            text.0.clear();
            continue;
        }

        if let Some(item) = item {
            let definition = &database.items[item.def_id];
            text.0 = slot_abbreviation(definition.slot).into();
            text_color.0 = item_label_color(item.rarity);
        } else if label.source == InventorySource::Equipment {
            text.0 = ItemSlot::all()
                .get(label.index)
                .map(|slot| slot_abbreviation(*slot))
                .unwrap_or("")
                .into();
            text_color.0 = Color::srgba(0.58, 0.53, 0.45, 0.62);
        } else {
            text.0.clear();
        }
    }
}

fn cursor_offset(window_query: &Query<&Window>) -> Option<Vec2> {
    window_query.single().ok().and_then(|window| {
        window.cursor_position().map(|position| {
            Vec2::new(
                position.x - WINDOW_WIDTH as f32 * 0.5,
                WINDOW_HEIGHT as f32 * 0.5 - position.y,
            )
        })
    })
}

fn hovered_inventory_cell(
    cursor_offset: Vec2,
    cell_query: &Query<(&InventoryCell, &ScreenFixed)>,
) -> Option<(InventorySource, usize)> {
    cell_query.iter().find_map(|(cell, fixed)| {
        let half_cell = INVENTORY_CELL_SIZE * 0.5;
        let within_x = (cursor_offset.x - fixed.offset.x).abs() <= half_cell;
        let within_y = (cursor_offset.y - fixed.offset.y).abs() <= half_cell;
        if within_x && within_y {
            Some((cell.source, cell.index))
        } else {
            None
        }
    })
}

fn item_location(source: InventorySource, index: usize) -> ItemLocation {
    match source {
        InventorySource::Inventory => ItemLocation::Inventory(index),
        InventorySource::Stash => ItemLocation::Stash(index),
        InventorySource::Equipment => ItemLocation::Equipment(index),
    }
}

fn item_cell_color(item: &ItemInstance, database: &GameDatabase) -> Color {
    let definition = &database.items[item.def_id];
    let _asset_key = definition.asset_key;
    rarity_color(item.rarity)
}

fn item_label_color(rarity: Rarity) -> Color {
    match rarity {
        Rarity::Common | Rarity::Uncommon | Rarity::Rare | Rarity::Legendary => {
            Color::srgb(0.10, 0.09, 0.07)
        }
        Rarity::Magic | Rarity::Epic => Color::srgb(0.96, 0.95, 0.90),
    }
}

fn slot_abbreviation(slot: ItemSlot) -> &'static str {
    match slot {
        ItemSlot::Weapon => "WPN",
        ItemSlot::Shield => "SHD",
        ItemSlot::Head => "HEAD",
        ItemSlot::Chest => "CHST",
        ItemSlot::Gloves => "GLV",
        ItemSlot::Legs => "LEG",
        ItemSlot::Boots => "BOOT",
        ItemSlot::Trinket => "TRNK",
    }
}

fn item_for_cell<'a>(cell: &InventoryCell, profile: &'a PlayerProfile) -> Option<&'a ItemInstance> {
    match cell.source {
        InventorySource::Inventory => profile.inventory.get(cell.index).and_then(Option::as_ref),
        InventorySource::Stash => profile.stash.get(cell.index).and_then(Option::as_ref),
        InventorySource::Equipment => profile.equipment.get(cell.index).and_then(Option::as_ref),
    }
}

pub(crate) fn sync_hud_text(
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    run: Res<RunState>,
    player_query: Query<&Health, With<Player>>,
    mut query: Query<(&HudText, &mut Text2d)>,
) {
    let map = &database.maps[run.map_index];
    let class = profile.class(&database);
    let stats = profile.derived_stats(&database);
    let attributes = profile.attributes(&database);
    let health_text = player_query
        .single()
        .map(|health| format!("HP {:.0}/{:.0}", health.current.max(0.0), health.max))
        .unwrap_or_else(|_| "HP --".into());

    for (kind, mut text) in &mut query {
        text.0 = match kind {
            HudText::Header => format!(
                "{} Lv.{}  |  Gold {}  |  Atlas Tier {}",
                class.name, profile.level, profile.gold, run.atlas_tier
            ),
            HudText::Stats => format!(
                "{}  |  DMG {:.0}  ARM {:.0}  APS {:.2}  MS {:.0}\nRegen {:.1}/s  |  STR {}  DEX {}  INT {}  VIT {}",
                health_text,
                stats.damage,
                stats.armor,
                stats.attacks_per_second,
                stats.move_speed,
                stats.health_regeneration,
                attributes.strength,
                attributes.dexterity,
                attributes.intelligence,
                attributes.vitality
            ),
            HudText::Message => format!(
                "{}  |  {} {}/{}",
                run.message, map.name, run.enemies_defeated, run.enemies_total
            ),
        };
    }
}

fn item_tooltip_text(item: &ItemInstance, database: &GameDatabase) -> String {
    let definition = &database.items[item.def_id];
    let damage = item_damage_bonus(item, definition);
    let armor = item_armor_bonus(item, definition);
    let life = item_life_bonus(item, definition);
    let move_speed = item_move_speed_bonus(item);
    let attack_speed = item_attack_speed_bonus(item);
    let health_regen = item_health_regen_bonus(item);
    let mut lines = vec![
        definition.name.to_string(),
        format!("{} {}", item.rarity.name(), definition.slot.name()),
        format!("Item level {}  |  Power {}", item.item_level, item.power),
        String::new(),
        definition.description.to_string(),
        String::new(),
    ];

    if damage > 0.0 {
        lines.push(format!("Damage +{damage:.0}"));
    }
    if armor > 0.0 {
        lines.push(format!("Armor +{armor:.0}"));
    }
    if life > 0.0 {
        lines.push(format!("Life +{life:.0}"));
    }
    if move_speed > 0.0 {
        lines.push(format!("Move speed +{move_speed:.1}"));
    }
    if attack_speed > 0.0 {
        lines.push(format!("Attack speed +{attack_speed:.1}%"));
    }
    if health_regen > 0.0 {
        lines.push(format!("Health regen +{health_regen:.1}/s"));
    }
    lines.push(item_slot_effect(definition.slot).to_string());
    if let Some(extra_effect) = rarity_effect(item.rarity) {
        lines.push(extra_effect.to_string());
    }

    lines.join("\n")
}

fn equipment_summary(profile: &PlayerProfile, database: &GameDatabase) -> String {
    let mut lines = vec!["Equipment".to_string()];
    for slot in ItemSlot::all() {
        let text = profile.equipment[slot.index()]
            .as_ref()
            .map(|item| {
                let definition = &database.items[item.def_id];
                format!(
                    "{} +{} ilvl {}",
                    definition.name, item.power, item.item_level
                )
            })
            .unwrap_or_else(|| "Empty".into());
        lines.push(format!("{}: {}", slot.name(), text));
    }
    lines.join("\n")
}

fn talent_summary(profile: &PlayerProfile, database: &GameDatabase) -> String {
    let mut lines = vec![format!("Talents  Points {}", profile.talent_points)];
    for (index, talent) in database.talents.iter().enumerate() {
        lines.push(format!(
            "{} {}/{}",
            talent.name, profile.allocated_talents[index], talent.max_points
        ));
    }
    lines.join("\n")
}

fn upgrade_summary(profile: &PlayerProfile, database: &GameDatabase) -> String {
    let class = profile.class(database);
    let mut damage_bonus = 0.0;
    let mut health_bonus = 0.0;
    let mut loot_bonus = 0.0;

    for (index, points) in profile.allocated_talents.iter().enumerate() {
        let points = *points as f32;
        match database.talents[index].grant {
            TalentGrant::DamagePercent(percent) => damage_bonus += percent * points,
            TalentGrant::HealthPercent(percent) => health_bonus += percent * points,
            TalentGrant::LootChance(percent) => loot_bonus += percent * points,
        }
    }

    format!(
        "Upgrades\nDamage talents +{damage_bonus:.0}%\nHealth talents +{health_bonus:.0}%\nLoot chance +{loot_bonus:.0}%\n\nNext level gains\n+{} STR  +{} DEX\n+{} INT  +{} VIT",
        class.growth.strength,
        class.growth.dexterity,
        class.growth.intelligence,
        class.growth.vitality,
    )
}
