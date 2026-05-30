use bevy::prelude::*;

use crate::components::{
    ActivePanel, CraftingAction, CraftingButton, CraftingButtonLabel, CraftingInfoText,
    CraftingPanelPiece, InventorySource, UiState,
};
use crate::constants::CRAFTING_SLOT_COUNT;
use crate::data::{
    CraftingDestination, CraftingPreview, CraftingResult, GameDatabase, ItemLocation,
    LiquidationResult, LootRng, PlayerProfile, describe_item,
};

use super::theme::{
    ACTION_BUTTON_SIZE, GRID_GAP, UiColors, UiFontSize, UiPanelSpec, WORKSPACE_PANEL_SIZE,
    action_button_color, action_button_node, bounded_lines, spawn_panel_section,
    spawn_panel_window,
};
use super::{item_location, spawn_inventory_grid};

fn spawn_crafting_button(
    parent: &mut ChildSpawnerCommands,
    action: CraftingAction,
    label: &'static str,
) {
    let size = ACTION_BUTTON_SIZE;
    parent
        .spawn((
            Button,
            action_button_node(size),
            BorderColor::all(UiColors::accent()),
            BackgroundColor(action_button_color(false, false)),
            CraftingButton { action },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font_size: UiFontSize::BUTTON,
                    ..default()
                },
                TextColor(UiColors::text_section()),
                CraftingButtonLabel { action },
                Label,
            ));
        });
}

fn spawn_crafting_info_text(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Text::new(""),
        TextFont {
            font_size: UiFontSize::BODY_SMALL,
            ..default()
        },
        TextColor(UiColors::text_muted()),
        TextLayout::new_with_justify(Justify::Left),
        Node {
            width: percent(100),
            min_height: px(74),
            overflow: Overflow::clip_y(),
            ..default()
        },
        CraftingInfoText,
        Label,
    ));
}

pub(super) fn spawn_crafting_panel(commands: &mut Commands) {
    spawn_panel_window(
        commands,
        CraftingPanelPiece,
        UiPanelSpec {
            left: 12.0,
            top: 17.0,
            size: WORKSPACE_PANEL_SIZE,
            title: "CRAFTING",
            body_color: UiColors::frame_body(),
            visibility: Visibility::Hidden,
            z_index: 12,
        },
        |body| {
            body.spawn((Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Row,
                column_gap: px(10),
                ..default()
            },))
                .with_children(|layout| {
                    layout
                        .spawn((Node {
                            width: px(292),
                            height: percent(100),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(8),
                            ..default()
                        },))
                        .with_children(|sources| {
                            spawn_panel_section(sources, "Stash", 1.12, |section| {
                                spawn_inventory_grid(
                                    section,
                                    ActivePanel::Crafting,
                                    InventorySource::Stash,
                                    6,
                                    5,
                                    4.0,
                                );
                            });
                            spawn_panel_section(sources, "Inventory", 0.9, |section| {
                                spawn_inventory_grid(
                                    section,
                                    ActivePanel::Crafting,
                                    InventorySource::Inventory,
                                    6,
                                    4,
                                    4.0,
                                );
                            });
                        });

                    layout
                        .spawn((Node {
                            flex_grow: 1.0,
                            height: percent(100),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(8),
                            ..default()
                        },))
                        .with_children(|workbench| {
                            spawn_panel_section(workbench, "Rarity Forge", 0.7, |section| {
                                spawn_inventory_grid(
                                    section,
                                    ActivePanel::Crafting,
                                    InventorySource::Crafting,
                                    CRAFTING_SLOT_COUNT,
                                    1,
                                    GRID_GAP,
                                );
                            });

                            spawn_panel_section(workbench, "Actions", 0.55, |section| {
                                section
                                    .spawn((Node {
                                        width: percent(100),
                                        flex_direction: FlexDirection::Row,
                                        column_gap: px(8),
                                        ..default()
                                    },))
                                    .with_children(|actions| {
                                        spawn_crafting_button(
                                            actions,
                                            CraftingAction::RarityUpgrade,
                                            "CRAFT",
                                        );
                                        spawn_crafting_button(
                                            actions,
                                            CraftingAction::Liquidate,
                                            "LIQUIDATE",
                                        );
                                    });
                                spawn_crafting_info_text(section);
                            });
                        });
                });
        },
    );
}

pub(crate) fn handle_crafting_input(
    mut ui_state: ResMut<UiState>,
    database: Res<GameDatabase>,
    mut profile: ResMut<PlayerProfile>,
    mut rng: ResMut<LootRng>,
    mouse: Res<ButtonInput<MouseButton>>,
    button_query: Query<(&CraftingButton, &Interaction), With<Button>>,
) {
    if ui_state.active_panel != ActivePanel::Crafting || ui_state.dragged_item.is_some() {
        return;
    }

    let clicked_action = button_query.iter().find_map(|(button, interaction)| {
        (*interaction == Interaction::Pressed).then_some(button.action)
    });

    let Some(clicked_action) = clicked_action else {
        return;
    };

    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    ui_state.crafting_message = match clicked_action {
        CraftingAction::RarityUpgrade => match profile.craft_rarity_upgrade(&database, &mut rng) {
            CraftingResult::Crafted { item, destination } => format!(
                "Crafted {}\nSent to {}",
                describe_item(&item, &database),
                crafting_destination_name(destination)
            ),
            CraftingResult::NeedsItems => "Add 5 matching items".to_string(),
            CraftingResult::RarityMismatch => "Rarities must match".to_string(),
            CraftingResult::MaxRarity => "Legendary cannot upgrade".to_string(),
        },
        CraftingAction::Liquidate => match profile.liquidate_crafting_items() {
            LiquidationResult::Liquidated { items, gold } => {
                format!("Liquidated {} {}\n+{} gold", items, item_word(items), gold)
            }
            LiquidationResult::Empty => "Add items to liquidate".to_string(),
        },
    };
}

pub(crate) fn sync_crafting_panel(
    ui_state: Res<UiState>,
    profile: Res<PlayerProfile>,
    mut visibility_query: Query<&mut Visibility, With<CraftingPanelPiece>>,
    mut button_query: Query<(&CraftingButton, &Interaction, &mut BackgroundColor), With<Button>>,
    mut label_query: Query<(&CraftingButtonLabel, &mut TextColor)>,
    mut info_query: Query<&mut Text, With<CraftingInfoText>>,
) {
    let is_visible = ui_state.active_panel == ActivePanel::Crafting;
    for mut visibility in &mut visibility_query {
        *visibility = if is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    let upgrade_ready = matches!(
        profile.crafting_upgrade_preview(),
        CraftingPreview::Ready { .. }
    );
    let liquidation_ready = profile.crafting_liquidation_count() > 0;

    for (button, interaction, mut background) in &mut button_query {
        let action_ready = crafting_action_ready(button.action, upgrade_ready, liquidation_ready);
        let hovered =
            is_visible && matches!(*interaction, Interaction::Hovered | Interaction::Pressed);

        background.0 = action_button_color(action_ready, hovered);
    }

    for (label, mut text_color) in &mut label_query {
        text_color.0 = if crafting_action_ready(label.action, upgrade_ready, liquidation_ready) {
            UiColors::text_section()
        } else {
            Color::srgba(0.64, 0.58, 0.48, 0.78)
        };
    }

    for mut text in &mut info_query {
        let status = crafting_status_text(&profile, &ui_state);
        text.0 = bounded_lines(status.lines().map(|line| line.to_string()), 31, 4);
    }
}

fn crafting_action_ready(
    action: CraftingAction,
    upgrade_ready: bool,
    liquidation_ready: bool,
) -> bool {
    match action {
        CraftingAction::RarityUpgrade => upgrade_ready,
        CraftingAction::Liquidate => liquidation_ready,
    }
}

fn crafting_status_text(profile: &PlayerProfile, ui_state: &UiState) -> String {
    if !ui_state.crafting_message.is_empty() {
        return ui_state.crafting_message.clone();
    }

    let filled = profile.crafting_liquidation_count();
    let liquidation_value = profile.crafting_liquidation_value();
    let liquidation_line = if liquidation_value > 0 {
        format!("\nLiquidate: +{liquidation_value}g")
    } else {
        String::new()
    };

    match profile.crafting_upgrade_preview() {
        CraftingPreview::Ready { from, to } => {
            format!("Ready\n{} -> {}{liquidation_line}", from.name(), to.name())
        }
        CraftingPreview::NeedsItems => {
            format!("{filled}/{CRAFTING_SLOT_COUNT} items\n5 matching rarity{liquidation_line}")
        }
        CraftingPreview::RarityMismatch => {
            format!("Rarity mismatch\nUse one rarity{liquidation_line}")
        }
        CraftingPreview::MaxRarity => {
            format!("Legendary items\nCannot upgrade{liquidation_line}")
        }
    }
}

pub(super) fn use_item_on_crafting_panel(
    profile: &mut PlayerProfile,
    source: InventorySource,
    index: usize,
    database: &GameDatabase,
) -> bool {
    match source {
        InventorySource::Inventory | InventorySource::Stash => {
            let Some(crafting_index) = profile.crafting.iter().position(Option::is_none) else {
                return false;
            };
            profile.move_item(
                item_location(source, index),
                ItemLocation::Crafting(crafting_index),
                database,
            )
        }
        InventorySource::Crafting => profile.use_item_at(ItemLocation::Crafting(index), database),
        InventorySource::Equipment => profile.use_item_at(ItemLocation::Equipment(index), database),
    }
}

fn crafting_destination_name(destination: CraftingDestination) -> &'static str {
    match destination {
        CraftingDestination::Inventory => "Inventory",
        CraftingDestination::Stash => "Stash",
        CraftingDestination::Cube => "Cube",
        CraftingDestination::Lost => "Lost",
    }
}

fn item_word(count: usize) -> &'static str {
    if count == 1 { "item" } else { "items" }
}
