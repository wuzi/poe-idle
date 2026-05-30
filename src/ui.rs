use bevy::prelude::*;
use bevy::sprite::Anchor;

mod crafting;
mod talents;
mod theme;

pub(crate) use crafting::{handle_crafting_input, sync_crafting_panel};
pub(crate) use talents::{handle_talent_panel, sync_talent_panel};

use crafting::{spawn_crafting_panel, use_item_on_crafting_panel};
use talents::spawn_talent_panel;
use theme::{
    UiColors, bounded_lines, navigation_button_color, spawn_panel_label,
    spawn_standard_panel_chrome, truncate_chars,
};

use crate::components::{
    ActivePanel, BottomButton, BottomButtonLabel, CharacterPanelPiece, CharacterPanelText,
    CraftingPanelPiece, DraggedItem, DraggedItemVisual, EquippedTooltipBackground,
    EquippedTooltipText, Health, HudText, InventoryCell, InventoryCellLabel, InventoryPanelPiece,
    InventorySource, ItemTooltipBackground, ItemTooltipText, Player, PortalMapButton,
    PortalMapButtonLabel, PortalMapRouteSlot, PortalPanelPiece, PortalToggleButton,
    PortalToggleButtonLabel, ProgressBarFill, ScreenFixed, UiState,
};
use crate::constants::{
    BOTTOM_BUTTON_SIZE, INVENTORY_CELL_SIZE, TOOLTIP_PADDING, TOOLTIP_WIDTH, WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

const TOOLTIP_LINE_HEIGHT: f32 = 17.0;
const TOOLTIP_GAP: f32 = 10.0;
const TOOLTIP_WRAP_CHARS: usize = 32;
const PORTAL_ROUTE_VISIBLE_COUNT: usize = 7;
use crate::data::{
    GameDatabase, ItemInstance, ItemLocation, ItemSlot, PlayerProfile, Rarity, RunState, RunStatus,
    TalentNode, item_armor_bonus, item_attack_speed_bonus, item_crit_chance_bonus,
    item_crit_damage_bonus, item_damage_bonus, item_gold_value, item_health_regen_bonus,
    item_life_bonus, item_move_speed_bonus, item_slot_effect, rarity_color, rarity_effect,
};

pub(crate) fn spawn_screen_layout(
    commands: &mut Commands,
    talents: &[TalentNode],
    map_count: usize,
) {
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

    spawn_portal_ui_panel(commands, map_count);

    spawn_fixed_text(
        commands,
        HudText::Header,
        Vec3::new(-62.0, 378.0, 35.0),
        14.0,
    );
    spawn_inventory_panel_frame(commands, Vec3::new(-394.0, 112.0, 30.0), "STASH");
    spawn_inventory_panel_frame(commands, Vec3::new(0.0, 112.0, 30.0), "HERO");
    spawn_inventory_panel_label(commands, "Inventory", Vec3::new(-134.0, 54.0, 35.0), 15.0);
    spawn_inventory_panel_label(commands, "Equipped", Vec3::new(-134.0, 286.0, 35.0), 15.0);

    spawn_inventory_cells(
        commands,
        ActivePanel::Inventory,
        InventorySource::Stash,
        -532.0,
        270.0,
        6,
        5,
        46.0,
    );
    spawn_inventory_cells(
        commands,
        ActivePanel::Inventory,
        InventorySource::Equipment,
        -134.0,
        250.0,
        4,
        2,
        46.0,
    );
    spawn_inventory_cells(
        commands,
        ActivePanel::Inventory,
        InventorySource::Inventory,
        -134.0,
        18.0,
        6,
        4,
        46.0,
    );
    spawn_bottom_button(
        commands,
        ActivePanel::Character,
        "HERO",
        Vec3::new(-220.0, -338.0, 35.0),
    );
    spawn_bottom_button(
        commands,
        ActivePanel::Inventory,
        "STASH",
        Vec3::new(-110.0, -338.0, 35.0),
    );
    spawn_bottom_button(
        commands,
        ActivePanel::Crafting,
        "CRAFT",
        Vec3::new(0.0, -338.0, 35.0),
    );
    spawn_bottom_button(
        commands,
        ActivePanel::Talents,
        "TALENTS",
        Vec3::new(110.0, -338.0, 35.0),
    );
    spawn_portal_toggle_button(commands, "PORTAL", Vec3::new(220.0, -338.0, 35.0));
    spawn_character_panel(commands);
    spawn_crafting_panel(commands);
    spawn_talent_panel(commands, talents);
    spawn_item_tooltip(commands);
    spawn_dragged_item_visual(commands);
}

fn spawn_inventory_panel_frame(commands: &mut Commands, center: Vec3, title: &'static str) {
    spawn_standard_panel_chrome(
        commands,
        || InventoryPanelPiece,
        center,
        title,
        Visibility::Hidden,
    );
}

fn spawn_portal_ui_panel(commands: &mut Commands, map_count: usize) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(800.0),
                top: px(17.0),
                width: px(368.0),
                height: px(502.0),
                padding: UiRect::all(px(6)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(UiColors::frame_shadow()),
            Visibility::Visible,
            ZIndex(12),
            PortalPanelPiece,
        ))
        .with_children(|panel| {
            panel
                .spawn((
                    Node {
                        width: percent(100),
                        height: percent(100),
                        padding: UiRect::all(px(6)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(8),
                        ..default()
                    },
                    BackgroundColor(UiColors::frame_shell()),
                ))
                .with_children(|shell| {
                    shell
                        .spawn((
                            Node {
                                width: percent(100),
                                height: px(40.0),
                                padding: UiRect::axes(px(18), px(0)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(UiColors::header()),
                        ))
                        .with_children(|header| {
                            header.spawn((
                                Text::new("PORTAL"),
                                TextFont {
                                    font_size: 22.0,
                                    ..default()
                                },
                                TextColor(UiColors::text_header()),
                                Label,
                            ));
                        });

                    shell.spawn((
                        Node {
                            width: percent(100),
                            height: px(4.0),
                            ..default()
                        },
                        BackgroundColor(UiColors::accent()),
                    ));

                    shell
                        .spawn((
                            Node {
                                width: percent(100),
                                flex_grow: 1.0,
                                padding: UiRect::all(px(10)),
                                flex_direction: FlexDirection::Column,
                                row_gap: px(8),
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            BackgroundColor(UiColors::frame_body()),
                        ))
                        .with_children(|body| {
                            body.spawn((
                                Node {
                                    width: percent(100),
                                    padding: UiRect::all(px(10)),
                                    flex_direction: FlexDirection::Column,
                                    row_gap: px(8),
                                    ..default()
                                },
                                BackgroundColor(UiColors::section()),
                            ))
                            .with_children(|details| {
                                details.spawn((
                                    Text::new("MAP DETAILS"),
                                    TextFont {
                                        font_size: 15.0,
                                        ..default()
                                    },
                                    TextColor(UiColors::text_section()),
                                    Label,
                                ));
                                details.spawn((
                                    Text::new(""),
                                    TextFont {
                                        font_size: 13.0,
                                        ..default()
                                    },
                                    TextColor(UiColors::text_primary()),
                                    TextLayout::new_with_justify(Justify::Left),
                                    Node {
                                        width: percent(100),
                                        ..default()
                                    },
                                    HudText::Message,
                                    Label,
                                ));
                                details
                                    .spawn((
                                        Node {
                                            width: percent(100),
                                            height: px(12.0),
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgba(0.05, 0.04, 0.035, 0.95)),
                                    ))
                                    .with_children(|bar| {
                                        bar.spawn((
                                            Node {
                                                width: px(1.0),
                                                height: percent(100),
                                                ..default()
                                            },
                                            BackgroundColor(Color::srgb(0.94, 0.66, 0.22)),
                                            ProgressBarFill,
                                        ));
                                    });
                            });

                            body.spawn((
                                Node {
                                    width: percent(100),
                                    flex_grow: 1.0,
                                    min_height: px(0),
                                    padding: UiRect::all(px(10)),
                                    flex_direction: FlexDirection::Column,
                                    row_gap: px(6),
                                    overflow: Overflow::clip_y(),
                                    ..default()
                                },
                                BackgroundColor(UiColors::section()),
                            ))
                            .with_children(|route| {
                                for slot_index in
                                    0..PORTAL_ROUTE_VISIBLE_COUNT.min(map_count.max(1))
                                {
                                    spawn_portal_ui_route_button(route, slot_index);
                                }
                            });
                        });
                });
        });
}

fn spawn_portal_ui_route_button(parent: &mut ChildSpawnerCommands, slot_index: usize) {
    parent
        .spawn((
            Button,
            Node {
                width: percent(100),
                height: px(32.0),
                padding: UiRect::axes(px(10), px(0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(px(1)),
                ..default()
            },
            BorderColor::all(UiColors::accent().with_alpha(0.55)),
            BackgroundColor(Color::srgba(0.28, 0.18, 0.09, 0.95)),
            PortalMapRouteSlot { slot_index },
            PortalMapButton {
                map_index: slot_index,
            },
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(""),
                TextFont {
                    font_size: 10.5,
                    ..default()
                },
                TextColor(UiColors::text_primary()),
                TextLayout::new_with_justify(Justify::Center),
                PortalMapRouteSlot { slot_index },
                PortalMapButtonLabel {
                    map_index: slot_index,
                },
                Label,
            ));
        });
}

fn spawn_inventory_cells(
    commands: &mut Commands,
    panel: ActivePanel,
    source: InventorySource,
    start_x: f32,
    start_y: f32,
    columns: usize,
    rows: usize,
    step: f32,
) {
    let cell_z = match panel {
        ActivePanel::Crafting => 48.0,
        _ => 34.0,
    };
    for row in 0..rows {
        for column in 0..columns {
            let index = row * columns + column;
            let offset = Vec3::new(
                start_x + column as f32 * step,
                start_y - row as f32 * step,
                cell_z,
            );
            let mut cell = commands.spawn((
                Sprite::from_color(
                    Color::srgba(0.10, 0.10, 0.11, 0.98),
                    Vec2::splat(INVENTORY_CELL_SIZE),
                ),
                Transform::from_translation(offset),
                Visibility::Hidden,
                ScreenFixed { offset },
                InventoryCell {
                    panel,
                    source,
                    index,
                },
            ));
            tag_inventory_cell_panel(&mut cell, panel);

            let mut label = commands.spawn((
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
                InventoryCellLabel {
                    panel,
                    source,
                    index,
                },
            ));
            tag_inventory_cell_panel(&mut label, panel);
        }
    }
}

fn tag_inventory_cell_panel(entity: &mut EntityCommands, panel: ActivePanel) {
    match panel {
        ActivePanel::Inventory => {
            entity.insert(InventoryPanelPiece);
        }
        ActivePanel::Crafting => {
            entity.insert(CraftingPanelPiece);
        }
        _ => {}
    }
}

fn spawn_fixed_rect(commands: &mut Commands, offset: Vec3, size: Vec2, color: Color) {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(offset),
        ScreenFixed { offset },
    ));
}

fn spawn_fixed_text(commands: &mut Commands, kind: HudText, offset: Vec3, font_size: f32) {
    commands.spawn((
        Text2d::new(""),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(UiColors::text_primary()),
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
    spawn_panel_label(
        commands,
        InventoryPanelPiece,
        label,
        offset,
        font_size,
        UiColors::text_section(),
        Visibility::Hidden,
        Justify::Left,
        Anchor::TOP_LEFT,
    );
}

fn spawn_bottom_button(
    commands: &mut Commands,
    panel: ActivePanel,
    label: &'static str,
    offset: Vec3,
) {
    let left = offset.x + WINDOW_WIDTH as f32 * 0.5 - BOTTOM_BUTTON_SIZE.x * 0.5;
    let bottom = WINDOW_HEIGHT as f32 * 0.5 + offset.y - BOTTOM_BUTTON_SIZE.y * 0.5;
    commands
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                left: px(left),
                bottom: px(bottom),
                width: px(BOTTOM_BUTTON_SIZE.x),
                height: px(BOTTOM_BUTTON_SIZE.y),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(px(2)),
                ..default()
            },
            BorderColor::all(UiColors::accent()),
            BackgroundColor(navigation_button_color(false, false)),
            ZIndex(20),
            BottomButton {
                panel,
                size: BOTTOM_BUTTON_SIZE,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(UiColors::text_section()),
                BottomButtonLabel { panel },
                Label,
            ));
        });
}

fn spawn_portal_toggle_button(commands: &mut Commands, label: &'static str, offset: Vec3) {
    let left = offset.x + WINDOW_WIDTH as f32 * 0.5 - BOTTOM_BUTTON_SIZE.x * 0.5;
    let bottom = WINDOW_HEIGHT as f32 * 0.5 + offset.y - BOTTOM_BUTTON_SIZE.y * 0.5;
    commands
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                left: px(left),
                bottom: px(bottom),
                width: px(BOTTOM_BUTTON_SIZE.x),
                height: px(BOTTOM_BUTTON_SIZE.y),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(px(2)),
                ..default()
            },
            BorderColor::all(UiColors::accent()),
            BackgroundColor(navigation_button_color(false, false)),
            ZIndex(20),
            PortalToggleButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: 13.0,
                    ..default()
                },
                TextColor(UiColors::text_section()),
                PortalToggleButtonLabel,
                Label,
            ));
        });
}

fn spawn_character_panel(commands: &mut Commands) {
    spawn_character_ui_window(commands, "STATUS", 12.0, 17.0, |parent| {
        spawn_character_ui_section(parent, CharacterPanelText::Status, 1.05);
        spawn_character_ui_section(parent, CharacterPanelText::Combat, 1.15);
        spawn_character_ui_section(parent, CharacterPanelText::Attributes, 0.55);
        spawn_character_ui_section(parent, CharacterPanelText::Talents, 0.8);
    });
    spawn_character_ui_window(commands, "HERO", 406.0, 17.0, |parent| {
        spawn_character_ui_section(parent, CharacterPanelText::Header, 0.38);
        spawn_character_ui_section(parent, CharacterPanelText::Equipment, 1.35);
        spawn_character_ui_section(parent, CharacterPanelText::Upgrades, 1.45);
    });
}

fn spawn_character_ui_window(
    commands: &mut Commands,
    title: &'static str,
    left: f32,
    top: f32,
    spawn_body: impl FnOnce(&mut ChildSpawnerCommands),
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(left),
                top: px(top),
                width: px(368.0),
                height: px(502.0),
                padding: UiRect::all(px(6)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(UiColors::frame_shadow()),
            Visibility::Hidden,
            ZIndex(12),
            CharacterPanelPiece,
        ))
        .with_children(|panel| {
            panel
                .spawn((
                    Node {
                        width: percent(100),
                        height: percent(100),
                        padding: UiRect::all(px(6)),
                        flex_direction: FlexDirection::Column,
                        row_gap: px(8),
                        ..default()
                    },
                    BackgroundColor(UiColors::frame_shell()),
                ))
                .with_children(|shell| {
                    shell
                        .spawn((
                            Node {
                                width: percent(100),
                                height: px(40.0),
                                padding: UiRect::axes(px(18), px(0)),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(UiColors::header()),
                        ))
                        .with_children(|header| {
                            header.spawn((
                                Text::new(title),
                                TextFont {
                                    font_size: 22.0,
                                    ..default()
                                },
                                TextColor(UiColors::text_header()),
                                Label,
                            ));
                        });

                    shell.spawn((
                        Node {
                            width: percent(100),
                            height: px(4.0),
                            ..default()
                        },
                        BackgroundColor(UiColors::accent()),
                    ));

                    shell
                        .spawn((
                            Node {
                                width: percent(100),
                                flex_grow: 1.0,
                                padding: UiRect::all(px(10)),
                                flex_direction: FlexDirection::Column,
                                row_gap: px(8),
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            BackgroundColor(UiColors::frame_body()),
                        ))
                        .with_children(spawn_body);
                });
        });
}

fn spawn_character_ui_section(
    parent: &mut ChildSpawnerCommands,
    kind: CharacterPanelText,
    flex_grow: f32,
) {
    parent
        .spawn((
            Node {
                width: percent(100),
                flex_grow,
                min_height: px(0),
                padding: UiRect::all(px(8)),
                overflow: Overflow::clip_y(),
                ..default()
            },
            BackgroundColor(UiColors::section()),
        ))
        .with_children(|section| {
            section.spawn((
                Text::new(""),
                TextFont {
                    font_size: character_ui_font_size(kind),
                    ..default()
                },
                TextColor(character_ui_text_color(kind)),
                TextLayout::new_with_justify(Justify::Left),
                Node {
                    width: percent(100),
                    ..default()
                },
                kind,
                Label,
            ));
        });
}

fn character_ui_font_size(kind: CharacterPanelText) -> f32 {
    match kind {
        CharacterPanelText::Header => 15.0,
        CharacterPanelText::Status => 13.0,
        _ => 12.0,
    }
}

fn character_ui_text_color(kind: CharacterPanelText) -> Color {
    match kind {
        CharacterPanelText::Header => UiColors::text_section(),
        _ => UiColors::text_primary(),
    }
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
    mut ui_button_query: Query<
        (&BottomButton, &Interaction, &mut BackgroundColor),
        (With<Button>, Without<Sprite>),
    >,
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
        sprite.color = navigation_button_color(active, hovered);
    }

    for (button, interaction, mut background) in &mut ui_button_query {
        let hovered = matches!(*interaction, Interaction::Hovered | Interaction::Pressed);
        if *interaction == Interaction::Pressed && mouse.just_pressed(MouseButton::Left) {
            next_panel = if ui_state.active_panel == button.panel {
                ActivePanel::None
            } else {
                button.panel
            };
        }

        let active = next_panel == button.panel;
        background.0 = navigation_button_color(active, hovered);
    }

    ui_state.active_panel = next_panel;

    for (label, mut text_color) in &mut label_query {
        text_color.0 = if ui_state.active_panel == label.panel {
            UiColors::text_dark()
        } else {
            UiColors::text_section()
        };
    }
}

pub(crate) fn handle_portal_button(
    mut ui_state: ResMut<UiState>,
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    run: Res<RunState>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut ui_toggle_query: Query<
        (&PortalToggleButton, &Interaction, &mut BackgroundColor),
        (With<Button>, Without<Sprite>),
    >,
    ui_map_button_query: Query<
        (&PortalMapButton, &Interaction),
        (
            With<Button>,
            Without<PortalToggleButton>,
            Without<ScreenFixed>,
        ),
    >,
    mut label_query: Query<&mut TextColor, With<PortalToggleButtonLabel>>,
) {
    for (_button, interaction, mut background) in &mut ui_toggle_query {
        let hovered = matches!(*interaction, Interaction::Hovered | Interaction::Pressed);
        if *interaction == Interaction::Pressed && mouse.just_pressed(MouseButton::Left) {
            ui_state.portal_visible = !ui_state.portal_visible;
        }

        background.0 = navigation_button_color(ui_state.portal_visible, hovered);
    }

    for mut text_color in &mut label_query {
        text_color.0 = if ui_state.portal_visible {
            UiColors::text_dark()
        } else {
            UiColors::text_section()
        };
    }

    if !ui_state.portal_visible || !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let visible_range = portal_visible_map_range(run.map_index, database.maps.len());

    for (button, interaction) in &ui_map_button_query {
        if !visible_range.contains(&button.map_index) {
            continue;
        }
        if *interaction == Interaction::Pressed && profile.map_unlocked(&database, button.map_index)
        {
            ui_state.requested_map_index = Some(button.map_index);
            break;
        }
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

pub(crate) fn sync_portal_map_buttons(
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    run: Res<RunState>,
    ui_state: Res<UiState>,
    mut ui_button_query: Query<
        (
            &PortalMapRouteSlot,
            &mut PortalMapButton,
            &Interaction,
            &mut Visibility,
            &mut BackgroundColor,
        ),
        (With<Button>, Without<Sprite>),
    >,
    mut ui_label_query: Query<
        (
            &PortalMapRouteSlot,
            &mut PortalMapButtonLabel,
            &mut Text,
            &mut TextColor,
        ),
        (With<Text>, Without<Text2d>),
    >,
) {
    let highest_unlocked = profile.highest_unlocked_map_index(&database);
    let visible_range = portal_visible_map_range(run.map_index, database.maps.len());

    for (slot, mut button, interaction, mut visibility, mut background) in &mut ui_button_query {
        if !ui_state.portal_visible || slot.slot_index >= visible_range.len() {
            *visibility = Visibility::Hidden;
            continue;
        }

        button.map_index = visible_range.start + slot.slot_index;
        *visibility = Visibility::Visible;

        let unlocked = profile.map_unlocked(&database, button.map_index);
        let current = run.map_index == button.map_index;
        let conquered = button.map_index < highest_unlocked;
        let hovered =
            unlocked && matches!(*interaction, Interaction::Hovered | Interaction::Pressed);
        background.0 = portal_map_button_color(current, conquered, unlocked, hovered);
    }

    for (slot, mut label, mut text, mut text_color) in &mut ui_label_query {
        if slot.slot_index >= visible_range.len() || !ui_state.portal_visible {
            text.0.clear();
            continue;
        }

        label.map_index = visible_range.start + slot.slot_index;
        let Some(map) = database.maps.get(label.map_index) else {
            text.0.clear();
            continue;
        };
        let unlocked = profile.map_unlocked(&database, label.map_index);
        let current = run.map_index == label.map_index;
        let state = if current {
            "CURRENT"
        } else if label.map_index < highest_unlocked {
            "CLEARED"
        } else if unlocked {
            "AVAILABLE"
        } else {
            "LOCKED"
        };

        text.0 = portal_map_label_text(label.map_index, map, state);
        text_color.0 = portal_map_label_color(current, unlocked);
    }
}

fn portal_map_button_color(current: bool, conquered: bool, unlocked: bool, hovered: bool) -> Color {
    if current && hovered {
        Color::srgba(0.94, 0.58, 0.10, 0.98)
    } else if current {
        Color::srgba(0.78, 0.38, 0.08, 0.98)
    } else if conquered && hovered {
        Color::srgba(0.46, 0.55, 0.22, 0.98)
    } else if conquered {
        Color::srgba(0.28, 0.40, 0.18, 0.96)
    } else if unlocked && hovered {
        Color::srgba(0.68, 0.42, 0.12, 0.98)
    } else if unlocked {
        Color::srgba(0.42, 0.27, 0.10, 0.96)
    } else {
        Color::srgba(0.12, 0.10, 0.09, 0.88)
    }
}

fn portal_map_label_text(
    map_index: usize,
    map: &crate::data::MapDefinition,
    state: &str,
) -> String {
    format!(
        "{:02}. {}   Lv {}   {}",
        map_index + 1,
        truncate_chars(map.name, 20),
        map.recommended_enemy_level(),
        state
    )
}

fn portal_map_label_color(current: bool, unlocked: bool) -> Color {
    if current {
        UiColors::text_dark()
    } else if unlocked {
        Color::srgb(0.95, 0.84, 0.58)
    } else {
        Color::srgba(0.58, 0.54, 0.48, 0.72)
    }
}

fn portal_visible_map_range(anchor: usize, total_maps: usize) -> std::ops::Range<usize> {
    if total_maps == 0 {
        return 0..0;
    }

    let visible_count = PORTAL_ROUTE_VISIBLE_COUNT.min(total_maps);
    let anchor = anchor.min(total_maps - 1);
    let mut start = anchor.saturating_sub(visible_count / 2);
    if start + visible_count > total_maps {
        start = total_maps - visible_count;
    }
    start..start + visible_count
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
    if !matches!(
        ui_state.active_panel,
        ActivePanel::Inventory | ActivePanel::Crafting
    ) {
        ui_state.dragged_item = None;
        return;
    }
    let active_panel = ui_state.active_panel;

    let Some(cursor_offset) = cursor_offset(&window_query) else {
        if mouse.just_released(MouseButton::Left) {
            ui_state.dragged_item = None;
        }
        return;
    };
    let hovered_cell = hovered_inventory_cell(cursor_offset, &cell_query, active_panel);

    if mouse.just_pressed(MouseButton::Right) {
        if let Some((source, index)) = hovered_cell {
            ui_state.dragged_item = None;
            let moved = if active_panel == ActivePanel::Crafting {
                use_item_on_crafting_panel(&mut profile, source, index, &database)
            } else {
                profile.use_item_at(item_location(source, index), &database)
            };
            if moved {
                ui_state.crafting_message.clear();
            }
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
                if profile.move_item(
                    item_location(dragged_item.source, dragged_item.index),
                    item_location(target_source, target_index),
                    &database,
                ) {
                    ui_state.crafting_message.clear();
                }
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
    if !matches!(
        ui_state.active_panel,
        ActivePanel::Inventory | ActivePanel::Crafting
    ) {
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
    mut text_2d_query: Query<(&CharacterPanelText, &mut Text2d)>,
    mut ui_text_query: Query<(&CharacterPanelText, &mut Text), Without<Text2d>>,
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

    let panel_text = |kind: &CharacterPanelText| -> String {
        match kind {
            CharacterPanelText::Header => format!("{}  Lv.{}", class.name, profile.level),
            CharacterPanelText::Status => bounded_lines(
                vec![
                    format!("Level        {}", profile.level),
                    format!("EXP          {}/{}", profile.xp, profile.xp_to_next_level()),
                    format!("HP           {}", health_text),
                    format!("Gold         {}", profile.gold),
                    format!("Map          {}", truncate_chars(map.name, 14)),
                    format!(
                        "Enemies      {}/{}",
                        run.enemies_defeated, run.enemies_total
                    ),
                    format!("Respawns     {}", profile.respawns),
                ],
                28,
                7,
            ),
            CharacterPanelText::Combat => bounded_lines(
                vec![
                    "Combat".to_string(),
                    format!("Damage       {:.0}", stats.damage),
                    format!("Armor        {:.0}", stats.armor),
                    format!("Attack speed {:.2}/s", stats.attacks_per_second),
                    format!(
                        "Crit         {:.1}% / +{:.0}%",
                        stats.crit_chance, stats.crit_damage
                    ),
                    format!("Move speed   {:.0}", stats.move_speed),
                    format!("Regen        {:.1}/s", stats.health_regeneration),
                    format!("Loot bonus   +{:.0}%", stats.loot_bonus),
                ],
                28,
                8,
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
        }
    };

    for (kind, mut text) in &mut text_2d_query {
        text.0 = panel_text(kind);
    }

    for (kind, mut text) in &mut ui_text_query {
        text.0 = panel_text(kind);
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

    if !matches!(
        ui_state.active_panel,
        ActivePanel::Inventory | ActivePanel::Crafting
    ) || ui_state.dragged_item.is_some()
    {
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
    let active_panel = ui_state.active_panel;
    let hovered_item = cell_query.iter().find_map(|(cell, fixed)| {
        if cell.panel != active_panel {
            return None;
        }
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
            panel: label.panel,
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
        } else if label.source == InventorySource::Crafting {
            text.0 = format!("{}", label.index + 1);
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
    active_panel: ActivePanel,
) -> Option<(InventorySource, usize)> {
    cell_query.iter().find_map(|(cell, fixed)| {
        if cell.panel != active_panel {
            return None;
        }
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
        InventorySource::Crafting => ItemLocation::Crafting(index),
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
        InventorySource::Crafting => profile.crafting.get(cell.index).and_then(Option::as_ref),
        InventorySource::Equipment => profile.equipment.get(cell.index).and_then(Option::as_ref),
    }
}

pub(crate) fn sync_hud_text(
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    run: Res<RunState>,
    mut text_2d_query: Query<(&HudText, &mut Text2d)>,
    mut ui_text_query: Query<(&HudText, &mut Text), Without<Text2d>>,
) {
    let map = &database.maps[run.map_index];
    let unlocked_count = profile.highest_unlocked_map_index(&database) + 1;
    let total_maps = database.maps.len();
    let run_status = match run.status {
        RunStatus::Running => "Running",
        RunStatus::Dead => "Rebuilding",
        RunStatus::Cleared => "Cleared",
    };

    let hud_text = |kind: &HudText| -> String {
        match kind {
            HudText::Header => format!("Gold {:>6}", profile.gold),
            HudText::Message => format!(
                "{}\nStage {}/{}  Enemy Lv {}\nUnlocked {}/{}  {}\n{}",
                truncate_chars(map.name, 24),
                run.map_index + 1,
                total_maps,
                map.recommended_enemy_level(),
                unlocked_count,
                total_maps,
                run_status,
                portal_log_lines(&run.message),
            ),
        }
    };

    for (kind, mut text) in &mut text_2d_query {
        text.0 = hud_text(kind);
    }

    for (kind, mut text) in &mut ui_text_query {
        text.0 = hud_text(kind);
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
        format!("Value {} gold", item_gold_value(item)),
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
                truncate_chars(&format!("{} +{}", definition.name, item.power), 22)
            })
            .unwrap_or_else(|| "Empty".to_string());
        lines.push(format!("{:>4}  {}", slot_abbreviation(slot), text));
    }
    bounded_lines(lines, 30, 9)
}

fn talent_summary(profile: &PlayerProfile, database: &GameDatabase) -> String {
    let mut lines = vec![
        "Talents".to_string(),
        format!("Available {}", profile.available_talent_points()),
    ];
    let tree = profile.talent_tree(database);
    let mut shown = 0;
    let mut total = 0;
    for (index, node) in tree.iter().enumerate() {
        let points = profile.talent_points_in(index);
        if points == 0 {
            continue;
        }
        total += 1;
        if shown < 4 {
            lines.push(format!(
                "{}  {}/{}",
                truncate_chars(node.name, 19),
                points,
                node.max_points
            ));
            shown += 1;
        }
    }
    if total == 0 {
        lines.push("Open TALENTS to spend".to_string());
    } else if total > shown {
        lines.push(format!("(+{} more)", total - shown));
    }
    bounded_lines(lines, 30, 7)
}

fn upgrade_summary(profile: &PlayerProfile, database: &GameDatabase) -> String {
    let class = profile.class(database);
    let effects = profile.talent_effects(database);
    let mut lines = vec!["Talent Bonuses".to_string()];
    if effects.damage_percent > 0.0 {
        lines.push(format!("Damage      +{:.0}%", effects.damage_percent));
    }
    if effects.life_percent > 0.0 {
        lines.push(format!("Life        +{:.0}%", effects.life_percent));
    }
    if effects.armor_percent > 0.0 {
        lines.push(format!("Armor       +{:.0}%", effects.armor_percent));
    }
    if effects.attack_speed_percent > 0.0 {
        lines.push(format!("Atk speed   +{:.0}%", effects.attack_speed_percent));
    }
    if effects.crit_chance > 0.0 {
        lines.push(format!("Crit chance +{:.1}%", effects.crit_chance));
    }
    if effects.crit_damage > 0.0 {
        lines.push(format!("Crit dmg    +{:.0}%", effects.crit_damage));
    }
    if effects.move_speed_percent > 0.0 {
        lines.push(format!("Move speed  +{:.0}%", effects.move_speed_percent));
    }
    if effects.life_regen > 0.0 {
        lines.push(format!("Regen       +{:.1}/s", effects.life_regen));
    }
    if effects.loot_chance > 0.0 {
        lines.push(format!("Loot        +{:.0}%", effects.loot_chance));
    }
    if effects.strength > 0.0 {
        lines.push(format!("Strength    +{:.0}", effects.strength));
    }
    if effects.dexterity > 0.0 {
        lines.push(format!("Dexterity   +{:.0}", effects.dexterity));
    }
    if effects.intelligence > 0.0 {
        lines.push(format!("Intel       +{:.0}", effects.intelligence));
    }
    if effects.vitality > 0.0 {
        lines.push(format!("Vitality    +{:.0}", effects.vitality));
    }
    if lines.len() == 1 {
        lines.push("None yet".to_string());
    }
    lines.push(String::new());
    lines.push(format!(
        "Next lv +{} STR +{} DEX",
        class.growth.strength, class.growth.dexterity
    ));
    lines.push(format!(
        "        +{} INT +{} VIT",
        class.growth.intelligence, class.growth.vitality
    ));
    bounded_lines(lines, 30, 12)
}
