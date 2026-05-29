use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::components::{
    ActivePanel, CraftingButton, CraftingButtonLabel, CraftingInfoText, CraftingPanelPiece,
    InventorySource, ScreenFixed, UiState,
};
use crate::constants::CRAFTING_SLOT_COUNT;
use crate::data::{
    CraftingDestination, CraftingPreview, CraftingResult, GameDatabase, ItemLocation, LootRng,
    PlayerProfile, describe_item,
};

use super::{cursor_offset, item_location, spawn_inventory_cells};

fn spawn_crafting_button(commands: &mut Commands, offset: Vec3) {
    let size = Vec2::new(92.0, 34.0);
    commands.spawn((
        Sprite::from_color(Color::srgba(0.30, 0.11, 0.04, 0.98), size),
        Transform::from_translation(offset),
        Visibility::Hidden,
        ScreenFixed { offset },
        CraftingPanelPiece,
        CraftingButton { size },
    ));

    let text_offset = offset + Vec3::new(0.0, 5.0, 1.0);
    commands.spawn((
        Text2d::new("CRAFT"),
        TextFont {
            font_size: 12.0,
            ..default()
        },
        TextColor(Color::srgb(0.96, 0.70, 0.32)),
        TextLayout::new_with_justify(Justify::Center),
        Anchor::CENTER,
        Transform::from_translation(text_offset),
        Visibility::Hidden,
        ScreenFixed {
            offset: text_offset,
        },
        CraftingPanelPiece,
        CraftingButtonLabel,
    ));
}

fn spawn_crafting_info_text(commands: &mut Commands, offset: Vec3) {
    commands.spawn((
        Text2d::new(""),
        TextFont {
            font_size: 10.5,
            ..default()
        },
        TextColor(Color::srgb(0.78, 0.72, 0.62)),
        TextLayout::new_with_justify(Justify::Left),
        Anchor::TOP_LEFT,
        Transform::from_translation(offset),
        Visibility::Hidden,
        ScreenFixed { offset },
        CraftingPanelPiece,
        CraftingInfoText,
    ));
}

pub(super) fn spawn_crafting_panel(commands: &mut Commands) {
    spawn_crafting_panel_rect(
        commands,
        Vec3::new(-150.0, 90.0, 42.0),
        Vec2::new(724.0, 540.0),
        Color::srgba(0.025, 0.025, 0.025, 0.98),
    );
    spawn_crafting_panel_rect(
        commands,
        Vec3::new(-150.0, 90.0, 43.0),
        Vec2::new(712.0, 528.0),
        Color::srgba(0.18, 0.17, 0.16, 0.97),
    );
    spawn_crafting_panel_rect(
        commands,
        Vec3::new(-150.0, 70.0, 44.0),
        Vec2::new(696.0, 466.0),
        Color::srgba(0.09, 0.085, 0.08, 0.97),
    );
    spawn_crafting_panel_rect(
        commands,
        Vec3::new(-150.0, 328.0, 45.0),
        Vec2::new(710.0, 40.0),
        Color::srgba(0.56, 0.10, 0.07, 0.98),
    );
    spawn_crafting_panel_rect(
        commands,
        Vec3::new(-150.0, 305.0, 46.0),
        Vec2::new(710.0, 4.0),
        Color::srgba(0.98, 0.56, 0.12, 0.92),
    );
    spawn_crafting_panel_rect(
        commands,
        Vec3::new(-142.0, 70.0, 46.0),
        Vec2::new(3.0, 430.0),
        Color::srgba(0.70, 0.50, 0.24, 0.62),
    );
    spawn_crafting_panel_label(
        commands,
        "CRAFTING",
        Vec3::new(-490.0, 342.0, 48.0),
        22.0,
        Color::srgb(1.0, 0.72, 0.20),
    );
    spawn_crafting_panel_label(
        commands,
        "STASH",
        Vec3::new(-500.0, 280.0, 48.0),
        14.0,
        Color::srgb(0.96, 0.70, 0.32),
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
        Color::srgb(0.96, 0.70, 0.32),
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
        Color::srgb(0.96, 0.70, 0.32),
    );
    spawn_crafting_panel_rect(
        commands,
        Vec3::new(34.0, 160.0, 46.0),
        Vec2::new(312.0, 128.0),
        Color::srgba(0.13, 0.115, 0.10, 0.96),
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
    spawn_crafting_button(commands, Vec3::new(34.0, 120.0, 48.0));
    spawn_crafting_info_text(commands, Vec3::new(-108.0, 82.0, 48.0));
}

fn spawn_crafting_panel_rect(commands: &mut Commands, offset: Vec3, size: Vec2, color: Color) {
    commands.spawn((
        Sprite::from_color(color, size),
        Transform::from_translation(offset),
        Visibility::Hidden,
        ScreenFixed { offset },
        CraftingPanelPiece,
    ));
}

fn spawn_crafting_panel_label(
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
        Visibility::Hidden,
        ScreenFixed { offset },
        CraftingPanelPiece,
    ));
}

pub(crate) fn handle_crafting_input(
    mut ui_state: ResMut<UiState>,
    database: Res<GameDatabase>,
    mut profile: ResMut<PlayerProfile>,
    mut rng: ResMut<LootRng>,
    mouse: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window>,
    button_query: Query<(&CraftingButton, &ScreenFixed)>,
) {
    if ui_state.active_panel != ActivePanel::Crafting || ui_state.dragged_item.is_some() {
        return;
    }

    let Some(cursor_offset) = cursor_offset(&window_query) else {
        return;
    };

    let button_hovered = button_query.iter().any(|(button, fixed)| {
        let half_size = button.size * 0.5;
        (cursor_offset.x - fixed.offset.x).abs() <= half_size.x
            && (cursor_offset.y - fixed.offset.y).abs() <= half_size.y
    });

    if !button_hovered || !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    ui_state.crafting_message = match profile.craft_rarity_upgrade(&database, &mut rng) {
        CraftingResult::Crafted { item, destination } => format!(
            "Crafted {}\nSent to {}",
            describe_item(&item, &database),
            crafting_destination_name(destination)
        ),
        CraftingResult::NeedsItems => "Add 5 matching items".to_string(),
        CraftingResult::RarityMismatch => "Rarities must match".to_string(),
        CraftingResult::MaxRarity => "Legendary cannot upgrade".to_string(),
    };
}

pub(crate) fn sync_crafting_panel(
    ui_state: Res<UiState>,
    profile: Res<PlayerProfile>,
    window_query: Query<&Window>,
    mut visibility_query: Query<&mut Visibility, With<CraftingPanelPiece>>,
    mut button_query: Query<(&CraftingButton, &ScreenFixed, &mut Sprite)>,
    mut label_query: Query<&mut TextColor, With<CraftingButtonLabel>>,
    mut info_query: Query<&mut Text2d, With<CraftingInfoText>>,
) {
    let is_visible = ui_state.active_panel == ActivePanel::Crafting;
    for mut visibility in &mut visibility_query {
        *visibility = if is_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    let is_ready = matches!(
        profile.crafting_upgrade_preview(),
        CraftingPreview::Ready { .. }
    );
    let cursor_offset = cursor_offset(&window_query);

    for (button, fixed, mut sprite) in &mut button_query {
        let hovered = is_visible
            && cursor_offset.is_some_and(|cursor| {
                let half_size = button.size * 0.5;
                (cursor.x - fixed.offset.x).abs() <= half_size.x
                    && (cursor.y - fixed.offset.y).abs() <= half_size.y
            });

        sprite.color = if is_ready && hovered {
            Color::srgba(0.92, 0.50, 0.08, 0.98)
        } else if is_ready {
            Color::srgba(0.56, 0.18, 0.05, 0.98)
        } else if hovered {
            Color::srgba(0.34, 0.19, 0.12, 0.98)
        } else {
            Color::srgba(0.18, 0.13, 0.10, 0.98)
        };
    }

    for mut text_color in &mut label_query {
        text_color.0 = if is_ready {
            Color::srgb(0.96, 0.70, 0.32)
        } else {
            Color::srgba(0.64, 0.58, 0.48, 0.78)
        };
    }

    for mut text in &mut info_query {
        text.0 = crafting_status_text(&profile, &ui_state);
    }
}

fn crafting_status_text(profile: &PlayerProfile, ui_state: &UiState) -> String {
    if !ui_state.crafting_message.is_empty() {
        return ui_state.crafting_message.clone();
    }

    let filled = profile
        .crafting
        .iter()
        .filter(|slot| slot.is_some())
        .count();
    match profile.crafting_upgrade_preview() {
        CraftingPreview::Ready { from, to } => {
            format!("Ready\n{} -> {}", from.name(), to.name())
        }
        CraftingPreview::NeedsItems => {
            format!("{filled}/{CRAFTING_SLOT_COUNT} items\n5 matching rarity")
        }
        CraftingPreview::RarityMismatch => "Rarity mismatch\nUse one rarity".to_string(),
        CraftingPreview::MaxRarity => "Legendary items\nCannot upgrade".to_string(),
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
