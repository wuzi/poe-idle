use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::components::{
    ActivePanel, BottomButton, BottomButtonLabel, CharacterPanelPiece, CharacterPanelText,
    DraggedItem, DraggedItemVisual, EquippedTooltipBackground, EquippedTooltipText, Health,
    HudText, InventoryCell, InventoryCellLabel, InventoryPanelPiece, InventorySource,
    ItemTooltipBackground, ItemTooltipText, Player, PortalPanelPiece, PortalToggleButton,
    PortalToggleButtonLabel, ProgressBarFill, ScreenFixed, UiState,
};
use crate::constants::{
    BOTTOM_BUTTON_SIZE, INVENTORY_CELL_SIZE, MAP_PROGRESS_LEFT, MAP_PROGRESS_WIDTH, MAP_PROGRESS_Y,
    TOOLTIP_PADDING, TOOLTIP_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH,
};

const TOOLTIP_LINE_HEIGHT: f32 = 17.0;
const TOOLTIP_GAP: f32 = 10.0;
const TOOLTIP_WRAP_CHARS: usize = 32;
use crate::data::{
    GameDatabase, ItemInstance, ItemLocation, ItemSlot, PlayerProfile, Rarity, RunState, RunStatus,
    TalentGrant, item_armor_bonus, item_attack_speed_bonus, item_crit_chance_bonus,
    item_crit_damage_bonus, item_damage_bonus, item_health_regen_bonus, item_life_bonus,
    item_move_speed_bonus, item_slot_effect, rarity_color, rarity_effect,
};

pub(crate) fn spawn_screen_layout(commands: &mut Commands) {
    spawn_fixed_rect(
        commands,
        Vec3::new(0.0, 368.0, 31.0),
        Vec2::new(152.0, 24.0),
        Color::srgba(0.13, 0.11, 0.08, 0.96),
    );
    spawn_fixed_rect(
        commands,
        Vec3::new(0.0, 352.0, 30.0),
        Vec2::new(WINDOW_WIDTH as f32, 10.0),
        Color::srgba(0.03, 0.025, 0.025, 0.98),
    );

    spawn_portal_panel_frame(commands, Vec3::new(394.0, 112.0, 30.0), "PORTAL");
    spawn_portal_panel_rect(
        commands,
        Vec3::new(394.0, 100.0, 31.0),
        Vec2::new(312.0, 312.0),
        Color::srgba(0.58, 0.50, 0.34, 0.94),
    );
    spawn_portal_panel_rect(
        commands,
        Vec3::new(394.0, 244.0, 32.0),
        Vec2::new(244.0, 28.0),
        Color::srgba(0.74, 0.64, 0.42, 0.94),
    );
    spawn_portal_panel_rect(
        commands,
        Vec3::new(394.0, 126.0, 32.0),
        Vec2::new(276.0, 250.0),
        Color::srgba(0.78, 0.68, 0.46, 0.94),
    );
    spawn_portal_panel_rect(
        commands,
        Vec3::new(394.0, 232.0, 33.0),
        Vec2::new(244.0, 28.0),
        Color::srgba(0.62, 0.46, 0.24, 0.92),
    );
    spawn_portal_panel_label(
        commands,
        "MAP DETAILS",
        Vec3::new(330.0, 242.0, 35.0),
        15.0,
        Color::srgb(0.14, 0.08, 0.035),
    );
    spawn_portal_panel_rect(
        commands,
        Vec3::new(
            MAP_PROGRESS_LEFT + MAP_PROGRESS_WIDTH * 0.5,
            MAP_PROGRESS_Y,
            32.0,
        ),
        Vec2::new(MAP_PROGRESS_WIDTH, 12.0),
        Color::srgba(0.05, 0.04, 0.035, 0.95),
    );

    commands.spawn((
        Sprite::from_color(Color::srgb(0.94, 0.66, 0.22), Vec2::new(1.0, 12.0)),
        Transform::from_xyz(MAP_PROGRESS_LEFT, MAP_PROGRESS_Y, 33.0),
        ScreenFixed {
            offset: Vec3::new(MAP_PROGRESS_LEFT, MAP_PROGRESS_Y, 33.0),
        },
        PortalPanelPiece,
        ProgressBarFill,
    ));

    spawn_fixed_text(
        commands,
        HudText::Header,
        Vec3::new(-62.0, 378.0, 35.0),
        14.0,
    );
    spawn_portal_text(
        commands,
        HudText::Message,
        Vec3::new(270.0, 212.0, 35.0),
        13.0,
    );

    spawn_inventory_panel_frame(commands, Vec3::new(-394.0, 112.0, 30.0), "STASH");
    spawn_inventory_panel_frame(commands, Vec3::new(0.0, 112.0, 30.0), "HERO");
    spawn_inventory_panel_label(commands, "Inventory", Vec3::new(-134.0, 54.0, 35.0), 15.0);
    spawn_inventory_panel_label(commands, "Equipped", Vec3::new(-134.0, 286.0, 35.0), 15.0);

    spawn_inventory_cells(commands, InventorySource::Stash, -532.0, 270.0, 6, 5, 46.0);
    spawn_inventory_cells(
        commands,
        InventorySource::Equipment,
        -134.0,
        250.0,
        4,
        2,
        46.0,
    );
    spawn_inventory_cells(
        commands,
        InventorySource::Inventory,
        -134.0,
        18.0,
        6,
        4,
        46.0,
    );
    spawn_bottom_button(
        commands,
        ActivePanel::Inventory,
        "STASH",
        Vec3::new(-50.0, -338.0, 35.0),
    );
    spawn_bottom_button(
        commands,
        ActivePanel::Character,
        "HERO",
        Vec3::new(50.0, -338.0, 35.0),
    );
    spawn_portal_toggle_button(commands, "PORTAL", Vec3::new(150.0, -338.0, 35.0));
    spawn_character_panel(commands);
    spawn_item_tooltip(commands);
    spawn_dragged_item_visual(commands);
}

fn spawn_portal_panel_frame(commands: &mut Commands, center: Vec3, title: &'static str) {
    spawn_portal_panel_rect(
        commands,
        center,
        Vec2::new(368.0, 502.0),
        Color::srgba(0.025, 0.025, 0.025, 0.98),
    );
    spawn_portal_panel_rect(
        commands,
        center + Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(356.0, 490.0),
        Color::srgba(0.19, 0.18, 0.17, 0.96),
    );
    spawn_portal_panel_rect(
        commands,
        center + Vec3::new(0.0, -18.0, 2.0),
        Vec2::new(340.0, 434.0),
        Color::srgba(0.10, 0.095, 0.085, 0.96),
    );
    spawn_portal_panel_rect(
        commands,
        center + Vec3::new(0.0, 218.0, 3.0),
        Vec2::new(338.0, 40.0),
        Color::srgba(0.56, 0.10, 0.07, 0.98),
    );
    spawn_portal_panel_rect(
        commands,
        center + Vec3::new(0.0, 195.0, 4.0),
        Vec2::new(338.0, 4.0),
        Color::srgba(0.98, 0.56, 0.12, 0.92),
    );
    spawn_portal_panel_label(
        commands,
        title,
        center + Vec3::new(-54.0, 232.0, 5.0),
        22.0,
        Color::srgb(1.0, 0.72, 0.20),
    );
}

fn spawn_inventory_panel_frame(commands: &mut Commands, center: Vec3, title: &'static str) {
    spawn_inventory_panel_rect(
        commands,
        center,
        Vec2::new(368.0, 502.0),
        Color::srgba(0.025, 0.025, 0.025, 0.98),
    );
    spawn_inventory_panel_rect(
        commands,
        center + Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(356.0, 490.0),
        Color::srgba(0.19, 0.18, 0.17, 0.96),
    );
    spawn_inventory_panel_rect(
        commands,
        center + Vec3::new(0.0, -18.0, 2.0),
        Vec2::new(340.0, 434.0),
        Color::srgba(0.09, 0.085, 0.08, 0.97),
    );
    spawn_inventory_panel_rect(
        commands,
        center + Vec3::new(0.0, 218.0, 3.0),
        Vec2::new(338.0, 40.0),
        Color::srgba(0.56, 0.10, 0.07, 0.98),
    );
    spawn_inventory_panel_rect(
        commands,
        center + Vec3::new(0.0, 195.0, 4.0),
        Vec2::new(338.0, 4.0),
        Color::srgba(0.98, 0.56, 0.12, 0.92),
    );
    spawn_inventory_panel_label(commands, title, center + Vec3::new(-54.0, 232.0, 5.0), 22.0);
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
                Visibility::Hidden,
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
                Visibility::Hidden,
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

fn spawn_portal_panel_rect(commands: &mut Commands, offset: Vec3, size: Vec2, color: Color) {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(offset),
        Visibility::Visible,
        ScreenFixed { offset },
        PortalPanelPiece,
    ));
}

fn spawn_inventory_panel_rect(commands: &mut Commands, offset: Vec3, size: Vec2, color: Color) {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(offset),
        Visibility::Hidden,
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

fn spawn_portal_text(commands: &mut Commands, kind: HudText, offset: Vec3, font_size: f32) {
    commands.spawn((
        Text2d::new(""),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(Color::srgb(0.10, 0.07, 0.04)),
        TextLayout::new_with_justify(Justify::Left),
        Anchor::TOP_LEFT,
        Transform::from_translation(offset),
        Visibility::Visible,
        ScreenFixed { offset },
        PortalPanelPiece,
        kind,
    ));
}

fn spawn_portal_panel_label(
    commands: &mut Commands,
    label: &'static str,
    offset: Vec3,
    font_size: f32,
    color: Color,
) {
    commands.spawn((
        Text2d::new(label),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(Justify::Left),
        Anchor::TOP_LEFT,
        Transform::from_translation(offset),
        Visibility::Visible,
        ScreenFixed { offset },
        PortalPanelPiece,
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
        Visibility::Hidden,
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
        Sprite::from_color(Color::srgba(0.30, 0.11, 0.04, 0.98), BOTTOM_BUTTON_SIZE),
        Transform::from_translation(offset),
        ScreenFixed { offset },
        BottomButton {
            panel,
            size: BOTTOM_BUTTON_SIZE,
        },
    ));

    let text_offset = offset + Vec3::new(0.0, 7.0, 1.0);
    commands.spawn((
        Text2d::new(label),
        TextFont {
            font_size: 13.0,
            ..default()
        },
        TextColor(Color::srgb(0.96, 0.70, 0.32)),
        TextLayout::new_with_justify(Justify::Center),
        Anchor::CENTER,
        Transform::from_translation(text_offset),
        ScreenFixed {
            offset: text_offset,
        },
        BottomButtonLabel { panel },
    ));
}

fn spawn_portal_toggle_button(commands: &mut Commands, label: &'static str, offset: Vec3) {
    commands.spawn((
        Sprite::from_color(Color::srgba(0.30, 0.11, 0.04, 0.98), BOTTOM_BUTTON_SIZE),
        Transform::from_translation(offset),
        ScreenFixed { offset },
        PortalToggleButton {
            size: BOTTOM_BUTTON_SIZE,
        },
    ));

    let text_offset = offset + Vec3::new(0.0, 7.0, 1.0);
    commands.spawn((
        Text2d::new(label),
        TextFont {
            font_size: 13.0,
            ..default()
        },
        TextColor(Color::srgb(0.18, 0.07, 0.02)),
        TextLayout::new_with_justify(Justify::Center),
        Anchor::CENTER,
        Transform::from_translation(text_offset),
        ScreenFixed {
            offset: text_offset,
        },
        PortalToggleButtonLabel,
    ));
}

fn spawn_character_panel(commands: &mut Commands) {
    spawn_character_panel_frame(commands, Vec3::new(-394.0, 112.0, 42.0), "STATUS");
    spawn_character_panel_frame(commands, Vec3::new(0.0, 112.0, 42.0), "HERO");

    spawn_character_panel_text(
        commands,
        CharacterPanelText::Header,
        Vec3::new(-40.0, 310.0, 47.0),
        15.0,
        Color::srgb(0.96, 0.70, 0.32),
    );
    spawn_character_panel_text(
        commands,
        CharacterPanelText::Status,
        Vec3::new(-548.0, 286.0, 47.0),
        13.0,
        Color::srgb(0.92, 0.89, 0.80),
    );
    spawn_character_panel_text(
        commands,
        CharacterPanelText::Combat,
        Vec3::new(-548.0, 148.0, 47.0),
        12.0,
        Color::srgb(0.92, 0.89, 0.80),
    );
    spawn_character_panel_text(
        commands,
        CharacterPanelText::Attributes,
        Vec3::new(-548.0, 38.0, 47.0),
        12.0,
        Color::srgb(0.92, 0.89, 0.80),
    );
    spawn_character_panel_text(
        commands,
        CharacterPanelText::Equipment,
        Vec3::new(-150.0, 272.0, 47.0),
        12.0,
        Color::srgb(0.92, 0.89, 0.80),
    );
    spawn_character_panel_text(
        commands,
        CharacterPanelText::Talents,
        Vec3::new(-548.0, -58.0, 47.0),
        12.0,
        Color::srgb(0.92, 0.89, 0.80),
    );
    spawn_character_panel_text(
        commands,
        CharacterPanelText::Upgrades,
        Vec3::new(-150.0, 82.0, 47.0),
        12.0,
        Color::srgb(0.92, 0.89, 0.80),
    );
}

fn spawn_character_panel_frame(commands: &mut Commands, center: Vec3, title: &'static str) {
    spawn_character_panel_rect(
        commands,
        center,
        Vec2::new(368.0, 502.0),
        Color::srgba(0.025, 0.025, 0.025, 0.98),
    );
    spawn_character_panel_rect(
        commands,
        center + Vec3::new(0.0, 0.0, 1.0),
        Vec2::new(356.0, 490.0),
        Color::srgba(0.19, 0.18, 0.17, 0.96),
    );
    spawn_character_panel_rect(
        commands,
        center + Vec3::new(0.0, -18.0, 2.0),
        Vec2::new(340.0, 434.0),
        Color::srgba(0.10, 0.095, 0.085, 0.97),
    );
    spawn_character_panel_rect(
        commands,
        center + Vec3::new(0.0, 218.0, 3.0),
        Vec2::new(338.0, 40.0),
        Color::srgba(0.56, 0.10, 0.07, 0.98),
    );
    spawn_character_panel_rect(
        commands,
        center + Vec3::new(0.0, 195.0, 4.0),
        Vec2::new(338.0, 4.0),
        Color::srgba(0.98, 0.56, 0.12, 0.92),
    );
    spawn_character_panel_static_label(
        commands,
        title,
        center + Vec3::new(-54.0, 232.0, 5.0),
        22.0,
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

fn spawn_character_panel_static_label(
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
        TextColor(Color::srgb(1.0, 0.72, 0.20)),
        TextLayout::new_with_justify(Justify::Left),
        Anchor::TOP_LEFT,
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
        EquippedTooltipBackground,
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
        EquippedTooltipText,
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
            Color::srgba(0.92, 0.50, 0.08, 0.98)
        } else if hovered {
            Color::srgba(0.58, 0.20, 0.06, 0.98)
        } else {
            Color::srgba(0.30, 0.11, 0.04, 0.98)
        };
    }

    ui_state.active_panel = next_panel;

    for (label, mut text_color) in &mut label_query {
        text_color.0 = if ui_state.active_panel == label.panel {
            Color::srgb(0.18, 0.07, 0.02)
        } else {
            Color::srgb(0.96, 0.70, 0.32)
        };
    }
}

pub(crate) fn handle_portal_button(
    mut ui_state: ResMut<UiState>,
    mouse: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window>,
    mut button_query: Query<(&PortalToggleButton, &ScreenFixed, &mut Sprite)>,
    mut label_query: Query<&mut TextColor, With<PortalToggleButtonLabel>>,
) {
    let cursor_offset = cursor_offset(&window_query);

    for (button, fixed, mut sprite) in &mut button_query {
        let hovered = cursor_offset.is_some_and(|cursor| {
            let half_size = button.size * 0.5;
            (cursor.x - fixed.offset.x).abs() <= half_size.x
                && (cursor.y - fixed.offset.y).abs() <= half_size.y
        });

        if hovered && mouse.just_pressed(MouseButton::Left) {
            ui_state.portal_visible = !ui_state.portal_visible;
        }

        sprite.color = if ui_state.portal_visible {
            Color::srgba(0.92, 0.50, 0.08, 0.98)
        } else if hovered {
            Color::srgba(0.58, 0.20, 0.06, 0.98)
        } else {
            Color::srgba(0.30, 0.11, 0.04, 0.98)
        };
    }

    for mut text_color in &mut label_query {
        text_color.0 = if ui_state.portal_visible {
            Color::srgb(0.18, 0.07, 0.02)
        } else {
            Color::srgb(0.96, 0.70, 0.32)
        };
    }
}

pub(crate) fn sync_portal_panel(
    ui_state: Res<UiState>,
    mut visibility_query: Query<&mut Visibility, With<PortalPanelPiece>>,
) {
    for mut visibility in &mut visibility_query {
        *visibility = if ui_state.portal_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
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
            CharacterPanelText::Header => format!("{}  Lv.{}", class.name, profile.level),
            CharacterPanelText::Status => format!(
                "Level        {}\nEXP          {}/{}\nHP           {}\nGold         {}\nMap          {}\nEnemies      {}/{}\nRespawns     {}",
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
                "Combat\nDamage       {:.0}\nArmor        {:.0}\nAttack speed {:.2}/s\nCrit         {:.1}% / +{:.0}%\nMove speed   {:.0}\nRegen        {:.1}/s\nLoot bonus   +{:.0}%",
                stats.damage,
                stats.armor,
                stats.attacks_per_second,
                stats.crit_chance,
                stats.crit_damage,
                stats.move_speed,
                stats.health_regeneration,
                stats.loot_bonus,
            ),
            CharacterPanelText::Attributes => format!(
                "Attributes\nSTR {}   DEX {}\nINT {}   VIT {}",
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
        (
            Without<ItemTooltipBackground>,
            Without<ItemTooltipText>,
            Without<EquippedTooltipBackground>,
            Without<EquippedTooltipText>,
        ),
    >,
    mut background_query: Query<
        (&mut ScreenFixed, &mut Sprite, &mut Visibility),
        (
            With<ItemTooltipBackground>,
            Without<ItemTooltipText>,
            Without<EquippedTooltipBackground>,
            Without<EquippedTooltipText>,
        ),
    >,
    mut text_query: Query<
        (
            &mut ScreenFixed,
            &mut Text2d,
            &mut TextColor,
            &mut Visibility,
        ),
        (
            With<ItemTooltipText>,
            Without<ItemTooltipBackground>,
            Without<EquippedTooltipBackground>,
            Without<EquippedTooltipText>,
        ),
    >,
    mut equipped_background_query: Query<
        (&mut ScreenFixed, &mut Sprite, &mut Visibility),
        (
            With<EquippedTooltipBackground>,
            Without<EquippedTooltipText>,
            Without<ItemTooltipBackground>,
            Without<ItemTooltipText>,
        ),
    >,
    mut equipped_text_query: Query<
        (
            &mut ScreenFixed,
            &mut Text2d,
            &mut TextColor,
            &mut Visibility,
        ),
        (
            With<EquippedTooltipText>,
            Without<EquippedTooltipBackground>,
            Without<ItemTooltipBackground>,
            Without<ItemTooltipText>,
        ),
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
    let Ok((
        mut equipped_background_fixed,
        mut equipped_background_sprite,
        mut equipped_background_visibility,
    )) = equipped_background_query.single_mut()
    else {
        return;
    };
    let Ok((
        mut equipped_text_fixed,
        mut equipped_text,
        mut equipped_text_color,
        mut equipped_text_visibility,
    )) = equipped_text_query.single_mut()
    else {
        return;
    };

    if ui_state.active_panel != ActivePanel::Inventory || ui_state.dragged_item.is_some() {
        *background_visibility = Visibility::Hidden;
        *text_visibility = Visibility::Hidden;
        *equipped_background_visibility = Visibility::Hidden;
        *equipped_text_visibility = Visibility::Hidden;
        return;
    }

    let Ok(window) = window_query.single() else {
        *background_visibility = Visibility::Hidden;
        *text_visibility = Visibility::Hidden;
        *equipped_background_visibility = Visibility::Hidden;
        *equipped_text_visibility = Visibility::Hidden;
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        *background_visibility = Visibility::Hidden;
        *text_visibility = Visibility::Hidden;
        *equipped_background_visibility = Visibility::Hidden;
        *equipped_text_visibility = Visibility::Hidden;
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
            item_for_cell(cell, &profile).map(|item| (cell.source, item))
        } else {
            None
        }
    });

    let Some((source, item)) = hovered_item else {
        *background_visibility = Visibility::Hidden;
        *text_visibility = Visibility::Hidden;
        *equipped_background_visibility = Visibility::Hidden;
        *equipped_text_visibility = Visibility::Hidden;
        return;
    };

    tooltip_text.0 = item_tooltip_text(item, &database, None);
    let primary_tooltip_height = tooltip_height(&tooltip_text.0);
    let slot = database.items[item.def_id].slot;
    let equipped_item = if source == InventorySource::Equipment {
        None
    } else {
        profile.equipment[slot.index()].as_ref()
    };
    if let Some(equipped_item) = equipped_item {
        equipped_text.0 = item_tooltip_text(equipped_item, &database, Some("Equipped"));
    }
    let equipped_tooltip_height = equipped_item
        .map(|_| tooltip_height(&equipped_text.0))
        .unwrap_or(0.0);
    let max_tooltip_height = primary_tooltip_height.max(equipped_tooltip_height);
    let (top_left, equipped_top_left) =
        tooltip_positions(cursor_offset, max_tooltip_height, equipped_item.is_some());

    let rarity_tint = rarity_color(item.rarity);
    background_sprite.custom_size = Some(Vec2::new(TOOLTIP_WIDTH, primary_tooltip_height));
    background_sprite.color = rarity_tint.mix(&Color::srgba(0.03, 0.025, 0.025, 0.96), 0.82);
    background_fixed.offset = Vec3::new(
        top_left.x + TOOLTIP_WIDTH * 0.5,
        top_left.y - primary_tooltip_height * 0.5,
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

    if let (Some(equipped_item), Some(equipped_top_left)) = (equipped_item, equipped_top_left) {
        let equipped_rarity_tint = rarity_color(equipped_item.rarity);
        equipped_background_sprite.custom_size =
            Some(Vec2::new(TOOLTIP_WIDTH, equipped_tooltip_height));
        equipped_background_sprite.color =
            equipped_rarity_tint.mix(&Color::srgba(0.03, 0.025, 0.025, 0.96), 0.82);
        equipped_background_fixed.offset = Vec3::new(
            equipped_top_left.x + TOOLTIP_WIDTH * 0.5,
            equipped_top_left.y - equipped_tooltip_height * 0.5,
            equipped_background_fixed.offset.z,
        );
        equipped_text_fixed.offset = Vec3::new(
            equipped_top_left.x + TOOLTIP_PADDING,
            equipped_top_left.y - TOOLTIP_PADDING,
            equipped_text_fixed.offset.z,
        );
        equipped_text_color.0 = Color::srgb(0.95, 0.92, 0.84);
        *equipped_background_visibility = Visibility::Visible;
        *equipped_text_visibility = Visibility::Visible;
    } else {
        *equipped_background_visibility = Visibility::Hidden;
        *equipped_text_visibility = Visibility::Hidden;
    }
}

fn tooltip_height(text: &str) -> f32 {
    let line_count = text.lines().count() as f32;
    (line_count * TOOLTIP_LINE_HEIGHT + TOOLTIP_PADDING * 2.0).max(110.0)
}

fn tooltip_positions(
    cursor_offset: Vec2,
    max_tooltip_height: f32,
    has_comparison: bool,
) -> (Vec2, Option<Vec2>) {
    let right_edge = WINDOW_WIDTH as f32 * 0.5 - TOOLTIP_PADDING;
    let left_edge = -(WINDOW_WIDTH as f32) * 0.5 + TOOLTIP_PADDING;
    let top_edge = WINDOW_HEIGHT as f32 * 0.5 - TOOLTIP_PADDING;
    let bottom_edge = -(WINDOW_HEIGHT as f32) * 0.5 + TOOLTIP_PADDING;
    let total_width = if has_comparison {
        TOOLTIP_WIDTH * 2.0 + TOOLTIP_GAP
    } else {
        TOOLTIP_WIDTH
    };

    let mut group_left = cursor_offset.x + 18.0;
    if group_left + total_width > right_edge {
        group_left = cursor_offset.x - total_width - 18.0;
    }
    group_left = group_left.clamp(left_edge, right_edge - total_width);

    let mut group_top = cursor_offset.y - 18.0;
    if group_top > top_edge {
        group_top = top_edge;
    }
    if group_top - max_tooltip_height < bottom_edge {
        group_top = bottom_edge + max_tooltip_height;
    }

    let primary_top_left = Vec2::new(group_left, group_top);
    let comparison_top_left = has_comparison.then_some(Vec2::new(
        group_left + TOOLTIP_WIDTH + TOOLTIP_GAP,
        group_top,
    ));
    (primary_top_left, comparison_top_left)
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
    mut query: Query<(&HudText, &mut Text2d)>,
) {
    let map = &database.maps[run.map_index];
    let run_status = match run.status {
        RunStatus::Running => "Running",
        RunStatus::Dead => "Rebuilding",
        RunStatus::Cleared => "Cleared",
    };

    for (kind, mut text) in &mut query {
        text.0 = match kind {
            HudText::Header => format!("Gold {:>6}", profile.gold),
            HudText::Message => format!(
                "{}\nDifficulty   Normal\nArea Level   {}\nAtlas Tier   {}\nRoute Length {:.0}m\nPacks        {}/{}\nEnemies      {}/{}\nSpawned      {}\nStatus       {}\nLog\n{}",
                map.name,
                map.area_level,
                run.atlas_tier,
                map.finish_x,
                run.next_pack_index,
                map.packs.len(),
                run.enemies_defeated,
                run.enemies_total,
                run.enemies_spawned,
                run_status,
                portal_log_lines(&run.message),
            ),
        };
    }
}

fn portal_log_lines(message: &str) -> String {
    const MAX_LINE_CHARS: usize = 26;
    const MAX_LINES: usize = 2;

    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut truncated = false;

    for word in message.split_whitespace() {
        let word_length = word.chars().count();
        if current_line.is_empty() {
            current_line = word.chars().take(MAX_LINE_CHARS).collect();
            truncated |= word_length > MAX_LINE_CHARS;
        } else if current_line.chars().count() + 1 + word_length <= MAX_LINE_CHARS {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(std::mem::take(&mut current_line));
            if lines.len() == MAX_LINES {
                truncated = true;
                break;
            }
            current_line = word.chars().take(MAX_LINE_CHARS).collect();
            truncated |= word_length > MAX_LINE_CHARS;
        }
    }

    if lines.len() < MAX_LINES && !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push("-".to_string());
    }

    if truncated {
        if let Some(last_line) = lines.last_mut() {
            while last_line.chars().count() > MAX_LINE_CHARS - 3 {
                last_line.pop();
            }
            last_line.push_str("...");
        }
    }

    lines
        .into_iter()
        .take(MAX_LINES)
        .map(|line| format!("  {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn item_tooltip_text(
    item: &ItemInstance,
    database: &GameDatabase,
    heading: Option<&str>,
) -> String {
    let definition = &database.items[item.def_id];
    let damage = item_damage_bonus(item, definition);
    let armor = item_armor_bonus(item, definition);
    let life = item_life_bonus(item, definition);
    let move_speed = item_move_speed_bonus(item);
    let attack_speed = item_attack_speed_bonus(item);
    let crit_chance = item_crit_chance_bonus(item);
    let crit_damage = item_crit_damage_bonus(item);
    let health_regen = item_health_regen_bonus(item);
    let mut lines = Vec::new();
    if let Some(heading) = heading {
        lines.push(heading.to_string());
        lines.push(String::new());
    }
    lines.extend([
        definition.name.to_string(),
        format!("{} {}", item.rarity.name(), definition.slot.name()),
        format!("Item level {}  |  Power {}", item.item_level, item.power),
        String::new(),
        definition.description.to_string(),
        String::new(),
    ]);

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
    if crit_chance > 0.0 {
        lines.push(format!("Crit chance +{crit_chance:.1}%"));
    }
    if crit_damage > 0.0 {
        lines.push(format!("Crit damage +{crit_damage:.0}%"));
    }
    if health_regen > 0.0 {
        lines.push(format!("Health regen +{health_regen:.1}/s"));
    }
    lines.push(item_slot_effect(definition.slot).to_string());
    if let Some(extra_effect) = rarity_effect(item.rarity) {
        lines.push(extra_effect.to_string());
    }

    wrap_tooltip_lines(lines)
}

fn wrap_tooltip_lines(lines: Vec<String>) -> String {
    lines
        .into_iter()
        .flat_map(|line| wrap_tooltip_line(&line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn wrap_tooltip_line(line: &str) -> Vec<String> {
    if line.is_empty() || line.chars().count() <= TOOLTIP_WRAP_CHARS {
        return vec![line.to_string()];
    }

    let mut wrapped = Vec::new();
    let mut current_line = String::new();

    for word in line.split_whitespace() {
        let word_length = word.chars().count();
        if current_line.is_empty() {
            current_line = word.chars().take(TOOLTIP_WRAP_CHARS).collect();
        } else if current_line.chars().count() + 1 + word_length <= TOOLTIP_WRAP_CHARS {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            wrapped.push(current_line);
            current_line = format!(
                "  {}",
                word.chars()
                    .take(TOOLTIP_WRAP_CHARS - 2)
                    .collect::<String>()
            );
        }
    }

    if !current_line.is_empty() {
        wrapped.push(current_line);
    }
    wrapped
}

fn equipment_summary(profile: &PlayerProfile, database: &GameDatabase) -> String {
    let mut lines = vec!["Equipment".to_string()];
    for slot in ItemSlot::all() {
        let text = profile.equipment[slot.index()]
            .as_ref()
            .map(|item| {
                let definition = &database.items[item.def_id];
                format!("{} +{}", definition.name, item.power)
            })
            .unwrap_or_else(|| "Empty".to_string());
        lines.push(format!("{:>4}  {}", slot_abbreviation(slot), text));
    }
    lines.join("\n")
}

fn talent_summary(profile: &PlayerProfile, database: &GameDatabase) -> String {
    let mut lines = vec![format!("Talents   Points {}", profile.talent_points)];
    for (index, talent) in database.talents.iter().enumerate() {
        lines.push(format!(
            "{}  {}/{}",
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
        "Upgrades\nDamage talents +{damage_bonus:.0}%\nHealth talents +{health_bonus:.0}%\nLoot chance     +{loot_bonus:.0}%\n\nNext level\n+{} STR  +{} DEX\n+{} INT  +{} VIT",
        class.growth.strength,
        class.growth.dexterity,
        class.growth.intelligence,
        class.growth.vitality,
    )
}
