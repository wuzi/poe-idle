use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::constants::{
    CRAFTING_SLOT_COUNT, EQUIPMENT_SLOT_COUNT, INVENTORY_SIZE, PLAYER_SPEED, STASH_SIZE,
};

mod crafting;
mod database;
mod items;
mod run;
mod talents;

pub(crate) use crafting::*;
pub(crate) use database::*;
pub(crate) use items::*;
pub(crate) use run::*;
pub(crate) use talents::*;

#[derive(Clone, Copy, Default)]
pub(crate) struct Attributes {
    pub(crate) strength: u32,
    pub(crate) dexterity: u32,
    pub(crate) intelligence: u32,
    pub(crate) vitality: u32,
}

impl Attributes {
    fn scaled_add(self, growth: Attributes, levels: u32) -> Self {
        Self {
            strength: self.strength + growth.strength * levels,
            dexterity: self.dexterity + growth.dexterity * levels,
            intelligence: self.intelligence + growth.intelligence * levels,
            vitality: self.vitality + growth.vitality * levels,
        }
    }
}

#[derive(Resource, Serialize, Deserialize)]
#[serde(default)]
pub(crate) struct PlayerProfile {
    pub(crate) class_id: ClassId,
    pub(crate) level: u32,
    pub(crate) xp: u32,
    pub(crate) gold: u32,
    pub(crate) allocated_talents: Vec<u8>,
    pub(crate) inventory: Vec<Option<ItemInstance>>,
    pub(crate) stash: Vec<Option<ItemInstance>>,
    pub(crate) crafting: Vec<Option<ItemInstance>>,
    pub(crate) equipment: Vec<Option<ItemInstance>>,
    pub(crate) highest_unlocked_map_index: usize,
    pub(crate) respawns: u32,
    pub(crate) starter_items_seeded: bool,
}

impl Default for PlayerProfile {
    fn default() -> Self {
        Self {
            class_id: ClassId::Knight,
            level: 1,
            xp: 0,
            gold: 0,
            allocated_talents: Vec::new(),
            inventory: vec![None; INVENTORY_SIZE],
            stash: vec![None; STASH_SIZE],
            crafting: vec![None; CRAFTING_SLOT_COUNT],
            equipment: vec![None; EQUIPMENT_SLOT_COUNT],
            highest_unlocked_map_index: 0,
            respawns: 0,
            starter_items_seeded: false,
        }
    }
}

impl PlayerProfile {
    pub(crate) fn class<'a>(&self, database: &'a GameDatabase) -> &'a ClassDefinition {
        database
            .classes
            .iter()
            .find(|class| class.id == self.class_id)
            .expect("player class should exist")
    }

    pub(crate) fn attributes(&self, database: &GameDatabase) -> Attributes {
        let class = self.class(database);
        let mut attributes = class
            .base_attributes
            .scaled_add(class.growth, self.level - 1);
        let effects = self.talent_effects(database);
        attributes.strength += effects.strength as u32;
        attributes.dexterity += effects.dexterity as u32;
        attributes.intelligence += effects.intelligence as u32;
        attributes.vitality += effects.vitality as u32;
        attributes
    }

    pub(crate) fn talent_tree<'a>(&self, database: &'a GameDatabase) -> &'a [TalentNode] {
        &self.class(database).talents
    }

    pub(crate) fn ensure_talent_slots(&mut self, database: &GameDatabase) {
        let len = self.class(database).talents.len();
        if self.allocated_talents.len() != len {
            self.allocated_talents.resize(len, 0);
        }
    }

    pub(crate) fn total_talent_points(&self) -> u32 {
        self.level.saturating_sub(1)
    }

    pub(crate) fn spent_talent_points(&self) -> u32 {
        self.allocated_talents
            .iter()
            .map(|points| *points as u32)
            .sum()
    }

    pub(crate) fn available_talent_points(&self) -> u32 {
        self.total_talent_points()
            .saturating_sub(self.spent_talent_points())
    }

    pub(crate) fn talent_points_in(&self, index: usize) -> u8 {
        self.allocated_talents.get(index).copied().unwrap_or(0)
    }

    pub(crate) fn talent_unlocked(&self, database: &GameDatabase, index: usize) -> bool {
        let Some(node) = self.talent_tree(database).get(index) else {
            return false;
        };
        match node.requires {
            Some(req) => self.talent_points_in(req) > 0,
            None => true,
        }
    }

    pub(crate) fn can_allocate_talent(&self, database: &GameDatabase, index: usize) -> bool {
        let Some(node) = self.talent_tree(database).get(index) else {
            return false;
        };
        self.available_talent_points() > 0
            && self.talent_points_in(index) < node.max_points
            && self.talent_unlocked(database, index)
    }

    pub(crate) fn allocate_talent(&mut self, database: &GameDatabase, index: usize) -> bool {
        if !self.can_allocate_talent(database, index) {
            return false;
        }
        self.allocated_talents[index] += 1;
        true
    }

    pub(crate) fn can_deallocate_talent(&self, database: &GameDatabase, index: usize) -> bool {
        if self.talent_points_in(index) == 0 {
            return false;
        }
        if self.talent_points_in(index) == 1 {
            for (other, node) in self.talent_tree(database).iter().enumerate() {
                if node.requires == Some(index) && self.talent_points_in(other) > 0 {
                    return false;
                }
            }
        }
        true
    }

    pub(crate) fn deallocate_talent(&mut self, database: &GameDatabase, index: usize) -> bool {
        if !self.can_deallocate_talent(database, index) {
            return false;
        }
        self.allocated_talents[index] -= 1;
        true
    }

    pub(crate) fn reset_talents(&mut self) {
        for points in self.allocated_talents.iter_mut() {
            *points = 0;
        }
    }

    pub(crate) fn talent_effects(&self, database: &GameDatabase) -> TalentEffects {
        let mut effects = TalentEffects::default();
        for (index, node) in self.class(database).talents.iter().enumerate() {
            let points = self.talent_points_in(index);
            if points == 0 {
                continue;
            }
            node.grant.apply(&mut effects, points as f32);
        }
        effects
    }

    pub(crate) fn derived_stats(&self, database: &GameDatabase) -> DerivedStats {
        let class = self.class(database);
        let attributes = self.attributes(database);
        let mut item_damage = 0.0;
        let mut item_armor = 0.0;
        let mut item_health = 0.0;
        let mut item_move_speed = 0.0;
        let mut item_attack_speed = 0.0;
        let mut item_crit_chance = 0.0;
        let mut item_crit_damage = 0.0;
        let mut item_health_regen = 0.0;

        for item in self.equipment.iter().flatten() {
            item_damage += item.rolls.damage;
            item_armor += item.rolls.armor;
            item_health += item.rolls.max_health;
            item_move_speed += item.rolls.move_speed;
            item_attack_speed += item.rolls.attack_speed;
            item_crit_chance += item.rolls.crit_chance;
            item_crit_damage += item.rolls.crit_damage;
            item_health_regen += item.rolls.health_regen;
        }

        let talents = self.talent_effects(database);
        let damage_multiplier = 1.0 + talents.damage_percent / 100.0;
        let health_multiplier = 1.0 + talents.life_percent / 100.0;
        let armor_multiplier = 1.0 + talents.armor_percent / 100.0;
        let move_multiplier = 1.0 + talents.move_speed_percent / 100.0;
        let loot_bonus = talents.loot_chance;

        let max_health = (82.0
            + attributes.vitality as f32 * 6.3
            + attributes.strength as f32 * 1.55
            + item_health)
            * health_multiplier;
        let damage = (class.base_damage
            + attributes.strength as f32 * 0.55
            + attributes.dexterity as f32 * 0.25
            + attributes.intelligence as f32 * 0.18
            + item_damage)
            * damage_multiplier;
        let armor =
            (class.base_armor + attributes.strength as f32 * 0.30 + item_armor) * armor_multiplier;
        let attacks_per_second = ((class.attacks_per_second + attributes.dexterity as f32 * 0.004)
            * (1.0 + (item_attack_speed + talents.attack_speed_percent) / 100.0))
            .clamp(0.45, 5.0);
        let crit_chance =
            (5.0 + attributes.dexterity as f32 * 0.08 + item_crit_chance + talents.crit_chance)
                .clamp(0.0, 75.0);
        let crit_damage =
            (50.0 + attributes.strength as f32 * 0.20 + item_crit_damage + talents.crit_damage)
                .clamp(0.0, 350.0);
        let move_speed = ((PLAYER_SPEED + attributes.dexterity as f32 * 0.28 + item_move_speed)
            * move_multiplier)
            .clamp(40.0, 260.0);
        let health_regeneration =
            (1.2 + attributes.vitality as f32 * 0.06 + item_health_regen + talents.life_regen)
                .clamp(0.0, 40.0);

        DerivedStats {
            max_health,
            damage,
            armor,
            attacks_per_second,
            crit_chance,
            crit_damage,
            move_speed,
            health_regeneration,
            loot_bonus,
        }
    }

    pub(crate) fn xp_to_next_level(&self) -> u32 {
        210 + self.level.pow(2) * 80
    }

    pub(crate) fn gain_xp(&mut self, xp: u32, _database: &GameDatabase) -> bool {
        let mut leveled = false;
        self.xp += xp;
        while self.xp >= self.xp_to_next_level() {
            self.xp -= self.xp_to_next_level();
            self.level += 1;
            leveled = true;
        }
        leveled
    }

    pub(crate) fn add_item(&mut self, item: ItemInstance) -> ItemDestination {
        if let Some(slot) = self.inventory.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(item);
            return ItemDestination::Inventory;
        }

        if let Some(slot) = self.stash.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(item);
            return ItemDestination::Stash;
        }

        ItemDestination::Lost
    }

    pub(crate) fn highest_unlocked_map_index(&self, database: &GameDatabase) -> usize {
        self.highest_unlocked_map_index
            .min(database.maps.len().saturating_sub(1))
    }

    pub(crate) fn map_unlocked(&self, database: &GameDatabase, map_index: usize) -> bool {
        map_index <= self.highest_unlocked_map_index(database)
    }

    pub(crate) fn unlock_next_map(
        &mut self,
        database: &GameDatabase,
        cleared_map_index: usize,
    ) -> Option<usize> {
        let next_map_index = cleared_map_index + 1;
        if next_map_index >= database.maps.len()
            || next_map_index <= self.highest_unlocked_map_index(database)
        {
            return None;
        }

        self.highest_unlocked_map_index = next_map_index;
        Some(next_map_index)
    }

    pub(crate) fn item_at(&self, location: ItemLocation) -> Option<&ItemInstance> {
        match location {
            ItemLocation::Inventory(index) => self.inventory.get(index),
            ItemLocation::Stash(index) => self.stash.get(index),
            ItemLocation::Crafting(index) => self.crafting.get(index),
            ItemLocation::Equipment(index) => self.equipment.get(index),
        }
        .and_then(Option::as_ref)
    }

    pub(crate) fn move_item(
        &mut self,
        from: ItemLocation,
        to: ItemLocation,
        database: &GameDatabase,
    ) -> bool {
        if from == to || !self.location_exists(to) {
            return false;
        }

        let Some(from_item) = self.take_item(from) else {
            return false;
        };
        if !self.can_place_item(to, &from_item, database) {
            self.place_item_unchecked(from, from_item);
            return false;
        }

        let to_item = self.take_item(to);
        if let Some(item) = &to_item {
            if !self.can_place_item(from, item, database) {
                self.place_item_unchecked(to, to_item.expect("checked item should exist"));
                self.place_item_unchecked(from, from_item);
                return false;
            }
        }

        self.place_item_unchecked(to, from_item);
        if let Some(item) = to_item {
            self.place_item_unchecked(from, item);
        }
        true
    }

    pub(crate) fn use_item_at(&mut self, location: ItemLocation, database: &GameDatabase) -> bool {
        let Some(item) = self.item_at(location) else {
            return false;
        };

        match location {
            ItemLocation::Inventory(_) | ItemLocation::Stash(_) => {
                let equipment_slot = database.items[item.def_id].slot.index();
                self.move_item(location, ItemLocation::Equipment(equipment_slot), database)
            }
            ItemLocation::Crafting(_) | ItemLocation::Equipment(_) => {
                if let Some(index) = self.inventory.iter().position(Option::is_none) {
                    self.move_item(location, ItemLocation::Inventory(index), database)
                } else if let Some(index) = self.stash.iter().position(Option::is_none) {
                    self.move_item(location, ItemLocation::Stash(index), database)
                } else {
                    false
                }
            }
        }
    }

    fn location_exists(&self, location: ItemLocation) -> bool {
        match location {
            ItemLocation::Inventory(index) => index < self.inventory.len(),
            ItemLocation::Stash(index) => index < self.stash.len(),
            ItemLocation::Crafting(index) => index < self.crafting.len(),
            ItemLocation::Equipment(index) => index < self.equipment.len(),
        }
    }

    fn can_place_item(
        &self,
        location: ItemLocation,
        item: &ItemInstance,
        database: &GameDatabase,
    ) -> bool {
        match location {
            ItemLocation::Inventory(_) | ItemLocation::Stash(_) | ItemLocation::Crafting(_) => {
                self.location_exists(location)
            }
            ItemLocation::Equipment(index) => {
                self.location_exists(location) && database.items[item.def_id].slot.index() == index
            }
        }
    }

    fn take_item(&mut self, location: ItemLocation) -> Option<ItemInstance> {
        match location {
            ItemLocation::Inventory(index) => self.inventory.get_mut(index),
            ItemLocation::Stash(index) => self.stash.get_mut(index),
            ItemLocation::Crafting(index) => self.crafting.get_mut(index),
            ItemLocation::Equipment(index) => self.equipment.get_mut(index),
        }
        .and_then(Option::take)
    }

    fn place_item_unchecked(&mut self, location: ItemLocation, item: ItemInstance) {
        let slot = match location {
            ItemLocation::Inventory(index) => self.inventory.get_mut(index),
            ItemLocation::Stash(index) => self.stash.get_mut(index),
            ItemLocation::Crafting(index) => self.crafting.get_mut(index),
            ItemLocation::Equipment(index) => self.equipment.get_mut(index),
        };

        if let Some(slot) = slot {
            *slot = Some(item);
        }
    }
}

pub(crate) struct DerivedStats {
    pub(crate) max_health: f32,
    pub(crate) damage: f32,
    pub(crate) armor: f32,
    pub(crate) attacks_per_second: f32,
    pub(crate) crit_chance: f32,
    pub(crate) crit_damage: f32,
    pub(crate) move_speed: f32,
    pub(crate) health_regeneration: f32,
    pub(crate) loot_bonus: f32,
}

pub(crate) enum ItemDestination {
    Inventory,
    Stash,
    Lost,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ItemLocation {
    Inventory(usize),
    Stash(usize),
    Crafting(usize),
    Equipment(usize),
}
