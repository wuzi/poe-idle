use bevy::prelude::*;
use bevy::sprite::Anchor;

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
    ACTION_BUTTON_SIZE, UiColors, UiFontSize, action_button_color, bounded_lines,
    spawn_panel_label, spawn_panel_rect, spawn_wide_panel_chrome,
};
use super::{item_location, spawn_inventory_cells, ui_position_from_screen_center};

fn spawn_crafting_button(
    commands: &mut Commands,
    action: CraftingAction,
    label: &'static str,
    offset: Vec3,
) {
    let size = ACTION_BUTTON_SIZE;
    let (left, top) = ui_position_from_screen_center(offset.truncate(), size);
    commands
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                left: px(left),
                top: px(top),
                width: px(size.x),
                height: px(size.y),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(px(2)),
                ..default()
            },
            BorderColor::all(UiColors::accent()),
            BackgroundColor(action_button_color(false, false)),
            Visibility::Hidden,
            ZIndex(22),
            CraftingPanelPiece,
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
                CraftingPanelPiece,
                CraftingButtonLabel { action },
                Label,
            ));
        });
}

fn spawn_crafting_info_text(commands: &mut Commands, offset: Vec3) {
    let (left, top) = ui_position_from_screen_center(offset.truncate(), Vec2::new(230.0, 74.0));
    commands.spawn((
        Text::new(""),
        TextFont {
            font_size: UiFontSize::BODY_SMALL,
            ..default()
        },
        TextColor(UiColors::text_muted()),
        TextLayout::new_with_justify(Justify::Left),
        Node {
            position_type: PositionType::Absolute,
            left: px(left),
            top: px(top),
            width: px(230.0),
            height: px(74.0),
            overflow: Overflow::clip_y(),
            ..default()
        },
        Visibility::Hidden,
        ZIndex(22),
        CraftingPanelPiece,
        CraftingInfoText,
        Label,
    ));
}

pub(super) fn spawn_crafting_panel(commands: &mut Commands) {
    let center = Vec3::new(-150.0, 90.0, 42.0);
    spawn_wide_panel_chrome(
        commands,
        || CraftingPanelPiece,
        center,
        "CRAFTING",
        UiColors::frame_body(),
        Visibility::Hidden,
    );
    spawn_crafting_panel_rect(
        commands,
        Vec3::new(-142.0, 70.0, 46.0),
        Vec2::new(3.0, 430.0),
        UiColors::divider(),
    );
    spawn_crafting_panel_label(
        commands,
        "STASH",
        Vec3::new(-500.0, 280.0, 48.0),
        14.0,
        UiColors::text_section(),
    );
    spawn_inventory_cells(
        commands,
        ActivePanel::Crafting,
        InventorySource::Stash,
        -496.0,
        240.0,
        6,
        5,
        46.0,
    );
    spawn_crafting_panel_label(
        commands,
        "INVENTORY",
        Vec3::new(-500.0, 20.0, 48.0),
        14.0,
        UiColors::text_section(),
    );
    spawn_inventory_cells(
        commands,
        ActivePanel::Crafting,
        InventorySource::Inventory,
        -496.0,
        -18.0,
        6,
        4,
        46.0,
    );
    spawn_crafting_panel_label(
        commands,
        "RARITY UPGRADE",
        Vec3::new(-110.0, 240.0, 48.0),
        16.0,
        UiColors::text_section(),
    );
    spawn_crafting_panel_rect(
        commands,
        Vec3::new(34.0, 160.0, 46.0),
        Vec2::new(312.0, 128.0),
        UiColors::section(),
    );
    spawn_inventory_cells(
        commands,
        ActivePanel::Crafting,
        InventorySource::Crafting,
        -92.0,
        182.0,
        CRAFTING_SLOT_COUNT,
        1,
        46.0,
    );
    spawn_crafting_button(
        commands,
        CraftingAction::RarityUpgrade,
        "CRAFT",
        Vec3::new(-36.0, 120.0, 48.0),
    );
    spawn_crafting_button(
        commands,
        CraftingAction::Liquidate,
        "LIQUIDATE",
        Vec3::new(106.0, 120.0, 48.0),
    );
    spawn_crafting_info_text(commands, Vec3::new(-108.0, 82.0, 48.0));
}

fn spawn_crafting_panel_rect(commands: &mut Commands, offset: Vec3, size: Vec2, color: Color) {
    spawn_panel_rect(
        commands,
        CraftingPanelPiece,
        offset,
        size,
        color,
        Visibility::Hidden,
    );
}

fn spawn_crafting_panel_label(
    commands: &mut Commands,
    label: &'static str,
    offset: Vec3,
    font_size: f32,
    color: Color,
) {
    spawn_panel_label(
        commands,
        CraftingPanelPiece,
        label,
        offset,
        font_size,
        color,
        Visibility::Hidden,
        Justify::Left,
        Anchor::TOP_LEFT,
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
    mut button_query: Query<
        (&CraftingButton, &Interaction, &mut BackgroundColor),
        (With<Button>, Without<Sprite>),
    >,
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
