use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::constants::EQUIPMENT_SLOT_COUNT;

use super::{GameDatabase, LootRng, PlayerProfile};

#[derive(Clone)]
pub(crate) struct ItemDefinition {
    pub(crate) name: &'static str,
    pub(crate) slot: ItemSlot,
    pub(crate) base_power: u32,
    pub(crate) description: &'static str,
    pub(crate) tint: Color,
    pub(crate) asset_key: &'static str,
    pub(crate) rolls: ItemRollProfile,
}

#[derive(Clone, Copy)]
pub(crate) struct ItemRollProfile {
    pub(crate) damage: Option<RollRange>,
    pub(crate) armor: Option<RollRange>,
    pub(crate) max_health: Option<RollRange>,
    pub(crate) move_speed: Option<RollRange>,
    pub(crate) attack_speed: Option<RollRange>,
    pub(crate) crit_chance: Option<RollRange>,
    pub(crate) crit_damage: Option<RollRange>,
    pub(crate) health_regen: Option<RollRange>,
}

impl ItemRollProfile {
    pub(super) const fn empty() -> Self {
        Self {
            damage: None,
            armor: None,
            max_health: None,
            move_speed: None,
            attack_speed: None,
            crit_chance: None,
            crit_damage: None,
            health_regen: None,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct RollRange {
    min: f32,
    max: f32,
}

impl RollRange {
    pub(super) const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ItemSlot {
    Weapon,
    Shield,
    Head,
    Chest,
    Gloves,
    Legs,
    Boots,
    Trinket,
}

impl ItemSlot {
    pub(crate) fn index(self) -> usize {
        match self {
            ItemSlot::Weapon => 0,
            ItemSlot::Shield => 1,
            ItemSlot::Head => 2,
            ItemSlot::Chest => 3,
            ItemSlot::Gloves => 4,
            ItemSlot::Legs => 5,
            ItemSlot::Boots => 6,
            ItemSlot::Trinket => 7,
        }
    }

    pub(crate) fn name(self) -> &'static str {
        match self {
            ItemSlot::Weapon => "Weapon",
            ItemSlot::Shield => "Shield",
            ItemSlot::Head => "Head",
            ItemSlot::Chest => "Chest",
            ItemSlot::Gloves => "Gloves",
            ItemSlot::Legs => "Legs",
            ItemSlot::Boots => "Boots",
            ItemSlot::Trinket => "Trinket",
        }
    }

    pub(crate) const fn all() -> [Self; EQUIPMENT_SLOT_COUNT] {
        [
            Self::Weapon,
            Self::Shield,
            Self::Head,
            Self::Chest,
            Self::Gloves,
            Self::Legs,
            Self::Boots,
            Self::Trinket,
        ]
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct ItemInstance {
    pub(crate) def_id: usize,
    pub(crate) rarity: Rarity,
    pub(crate) item_level: u32,
    pub(crate) power: u32,
    pub(crate) rolls: ItemStatRolls,
}

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub(crate) struct ItemStatRolls {
    pub(crate) damage: f32,
    pub(crate) armor: f32,
    pub(crate) max_health: f32,
    pub(crate) move_speed: f32,
    pub(crate) attack_speed: f32,
    pub(crate) crit_chance: f32,
    pub(crate) crit_damage: f32,
    pub(crate) health_regen: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Rarity {
    Common,
    Uncommon,
    Magic,
    Rare,
    Epic,
    Legendary,
}

impl Rarity {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Rarity::Common => "Common",
            Rarity::Uncommon => "Uncommon",
            Rarity::Magic => "Magic",
            Rarity::Rare => "Rare",
            Rarity::Epic => "Epic",
            Rarity::Legendary => "Legendary",
        }
    }

    pub(crate) fn multiplier(self) -> u32 {
        match self {
            Rarity::Common => 1,
            Rarity::Uncommon => 2,
            Rarity::Magic => 3,
            Rarity::Rare => 4,
            Rarity::Epic => 5,
            Rarity::Legendary => 7,
        }
    }

    pub(crate) fn next(self) -> Option<Self> {
        match self {
            Rarity::Common => Some(Rarity::Uncommon),
            Rarity::Uncommon => Some(Rarity::Magic),
            Rarity::Magic => Some(Rarity::Rare),
            Rarity::Rare => Some(Rarity::Epic),
            Rarity::Epic => Some(Rarity::Legendary),
            Rarity::Legendary => None,
        }
    }

    fn roll_multiplier(self) -> f32 {
        match self {
            Rarity::Common => 1.0,
            Rarity::Uncommon => 1.15,
            Rarity::Magic => 1.4,
            Rarity::Rare => 1.85,
            Rarity::Epic => 2.45,
            Rarity::Legendary => 3.2,
        }
    }
}

pub(crate) fn seed_starting_equipment(profile: &mut PlayerProfile, database: &GameDatabase) {
    if profile.starter_items_seeded {
        return;
    }
    if profile.equipment.iter().any(Option::is_some) {
        profile.starter_items_seeded = true;
        return;
    }

    let weapon = starter_item(0, database);
    let shield = starter_item(1, database);
    let weapon_slot = database.items[weapon.def_id].slot.index();
    let shield_slot = database.items[shield.def_id].slot.index();
    profile.equipment[weapon_slot] = Some(weapon);
    profile.equipment[shield_slot] = Some(shield);
    profile.starter_items_seeded = true;
}

pub(crate) fn roll_item(
    rng: &mut LootRng,
    database: &GameDatabase,
    item_level: u32,
) -> ItemInstance {
    let rarity = match rng.percent() {
        roll if roll >= 99.6 => Rarity::Legendary,
        roll if roll >= 98.0 => Rarity::Epic,
        roll if roll >= 92.0 => Rarity::Rare,
        roll if roll >= 72.0 => Rarity::Magic,
        roll if roll >= 45.0 => Rarity::Uncommon,
        _ => Rarity::Common,
    };
    roll_item_of_rarity(rng, database, item_level, rarity)
}

pub(crate) fn roll_item_of_rarity(
    rng: &mut LootRng,
    database: &GameDatabase,
    item_level: u32,
    rarity: Rarity,
) -> ItemInstance {
    let def_id = rng.range(database.items.len());
    let definition = &database.items[def_id];
    let rolls = roll_item_stats(rng, definition, item_level, rarity);
    let power = item_power_score(definition, item_level, rarity, rolls);
    ItemInstance {
        def_id,
        rarity,
        item_level,
        power,
        rolls,
    }
}

fn starter_item(def_id: usize, database: &GameDatabase) -> ItemInstance {
    let rarity = Rarity::Common;
    let item_level = 1;
    let definition = &database.items[def_id];
    let rolls = minimum_item_stats(definition, item_level, rarity);
    let power = item_power_score(definition, item_level, rarity, rolls);
    ItemInstance {
        def_id,
        rarity,
        item_level,
        power,
        rolls,
    }
}

fn roll_item_stats(
    rng: &mut LootRng,
    definition: &ItemDefinition,
    item_level: u32,
    rarity: Rarity,
) -> ItemStatRolls {
    ItemStatRolls {
        damage: roll_stat(rng, definition.rolls.damage, item_level, rarity),
        armor: roll_stat(rng, definition.rolls.armor, item_level, rarity),
        max_health: roll_stat(rng, definition.rolls.max_health, item_level, rarity),
        move_speed: roll_stat(rng, definition.rolls.move_speed, item_level, rarity),
        attack_speed: roll_stat(rng, definition.rolls.attack_speed, item_level, rarity),
        crit_chance: roll_stat(rng, definition.rolls.crit_chance, item_level, rarity),
        crit_damage: roll_stat(rng, definition.rolls.crit_damage, item_level, rarity),
        health_regen: roll_stat(rng, definition.rolls.health_regen, item_level, rarity),
    }
}

fn minimum_item_stats(
    definition: &ItemDefinition,
    item_level: u32,
    rarity: Rarity,
) -> ItemStatRolls {
    ItemStatRolls {
        damage: minimum_stat(definition.rolls.damage, item_level, rarity),
        armor: minimum_stat(definition.rolls.armor, item_level, rarity),
        max_health: minimum_stat(definition.rolls.max_health, item_level, rarity),
        move_speed: minimum_stat(definition.rolls.move_speed, item_level, rarity),
        attack_speed: minimum_stat(definition.rolls.attack_speed, item_level, rarity),
        crit_chance: minimum_stat(definition.rolls.crit_chance, item_level, rarity),
        crit_damage: minimum_stat(definition.rolls.crit_damage, item_level, rarity),
        health_regen: minimum_stat(definition.rolls.health_regen, item_level, rarity),
    }
}

fn roll_stat(rng: &mut LootRng, range: Option<RollRange>, item_level: u32, rarity: Rarity) -> f32 {
    let Some(range) = range else {
        return 0.0;
    };
    let (min, max) = scaled_range(range, item_level, rarity);
    rounded_roll(min + (max - min) * rng.unit())
}

fn minimum_stat(range: Option<RollRange>, item_level: u32, rarity: Rarity) -> f32 {
    let Some(range) = range else {
        return 0.0;
    };
    let (min, _) = scaled_range(range, item_level, rarity);
    rounded_roll(min)
}

fn scaled_range(range: RollRange, item_level: u32, rarity: Rarity) -> (f32, f32) {
    let level_scale = 1.0 + item_level.saturating_sub(1) as f32 * 0.12;
    let rarity_scale = rarity.roll_multiplier();
    (
        range.min * level_scale * rarity_scale,
        range.max * level_scale * rarity_scale,
    )
}

fn rounded_roll(value: f32) -> f32 {
    (value * 10.0).round() / 10.0
}

fn item_power_score(
    definition: &ItemDefinition,
    item_level: u32,
    rarity: Rarity,
    rolls: ItemStatRolls,
) -> u32 {
    let weighted_stats = rolls.damage
        + rolls.armor * 0.65
        + rolls.max_health * 0.18
        + rolls.move_speed * 0.85
        + rolls.attack_speed * 1.4
        + rolls.crit_chance * 2.2
        + rolls.crit_damage * 0.45
        + rolls.health_regen * 4.0;
    definition.base_power + item_level + rarity.multiplier() * 2 + weighted_stats.round() as u32
}

pub(crate) fn describe_item(item: &ItemInstance, database: &GameDatabase) -> String {
    let definition = &database.items[item.def_id];
    format!(
        "{} {} ilvl {}",
        item.rarity.name(),
        definition.name,
        item.item_level
    )
}

pub(crate) fn item_damage_bonus(item: &ItemInstance, _definition: &ItemDefinition) -> f32 {
    item.rolls.damage
}

pub(crate) fn item_armor_bonus(item: &ItemInstance, _definition: &ItemDefinition) -> f32 {
    item.rolls.armor
}

pub(crate) fn item_life_bonus(item: &ItemInstance, _definition: &ItemDefinition) -> f32 {
    item.rolls.max_health
}

pub(crate) fn item_move_speed_bonus(item: &ItemInstance) -> f32 {
    item.rolls.move_speed
}

pub(crate) fn item_attack_speed_bonus(item: &ItemInstance) -> f32 {
    item.rolls.attack_speed
}

pub(crate) fn item_crit_chance_bonus(item: &ItemInstance) -> f32 {
    item.rolls.crit_chance
}

pub(crate) fn item_crit_damage_bonus(item: &ItemInstance) -> f32 {
    item.rolls.crit_damage
}

pub(crate) fn item_health_regen_bonus(item: &ItemInstance) -> f32 {
    item.rolls.health_regen
}

pub(crate) fn item_slot_effect(slot: ItemSlot) -> &'static str {
    match slot {
        ItemSlot::Weapon => "Effect: rolls damage, attack speed, and critical stats.",
        ItemSlot::Shield => "Effect: reduces incoming hit damage through armor.",
        ItemSlot::Head => "Effect: rolls armor, maximum life, and health regeneration.",
        ItemSlot::Chest => "Effect: rolls strong armor, life, and health regeneration.",
        ItemSlot::Gloves => "Effect: rolls attack speed, damage, and critical stats.",
        ItemSlot::Legs => "Effect: rolls armor, life, and health regeneration.",
        ItemSlot::Boots => "Effect: rolls movement speed for faster map travel.",
        ItemSlot::Trinket => "Effect: adds flexible damage and survival stats.",
    }
}

pub(crate) fn rarity_effect(rarity: Rarity) -> Option<&'static str> {
    match rarity {
        Rarity::Common => None,
        Rarity::Uncommon => Some("Uncommon effect: slightly stronger roll ranges."),
        Rarity::Magic => Some("Magic effect: stronger roll ranges."),
        Rarity::Rare => Some("Rare effect: several strong roll ranges."),
        Rarity::Epic => Some("Epic effect: exceptional roll ranges."),
        Rarity::Legendary => Some("Legendary effect: extreme roll ranges."),
    }
}

pub(crate) fn rarity_color(rarity: Rarity) -> Color {
    match rarity {
        Rarity::Common => Color::srgb(0.92, 0.92, 0.86),
        Rarity::Uncommon => Color::srgb(0.20, 0.78, 0.28),
        Rarity::Magic => Color::srgb(0.26, 0.45, 0.95),
        Rarity::Rare => Color::srgb(0.96, 0.84, 0.18),
        Rarity::Epic => Color::srgb(0.68, 0.36, 0.95),
        Rarity::Legendary => Color::srgb(1.0, 0.55, 0.08),
    }
}
