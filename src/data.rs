use bevy::prelude::*;

use crate::constants::{EQUIPMENT_SLOT_COUNT, INVENTORY_SIZE, STASH_SIZE};

#[derive(Resource)]
pub(crate) struct GameDatabase {
    pub(crate) classes: Vec<ClassDefinition>,
    pub(crate) maps: Vec<MapDefinition>,
    pub(crate) items: Vec<ItemDefinition>,
    pub(crate) talents: Vec<TalentDefinition>,
}

impl Default for GameDatabase {
    fn default() -> Self {
        Self {
            classes: vec![
                ClassDefinition {
                    id: ClassId::Knight,
                    name: "Knight",
                    base_attributes: Attributes {
                        strength: 13,
                        dexterity: 8,
                        intelligence: 5,
                        vitality: 12,
                    },
                    growth: Attributes {
                        strength: 3,
                        dexterity: 1,
                        intelligence: 1,
                        vitality: 3,
                    },
                    base_damage: 8.0,
                    base_armor: 8.0,
                    attacks_per_second: 0.92,
                    visual: VisualProfile {
                        asset_key: "characters/knight.png",
                        color: Color::srgb(0.56, 0.63, 0.68),
                        size: Vec2::new(46.0, 64.0),
                    },
                },
                ClassDefinition {
                    id: ClassId::Ranger,
                    name: "Ranger",
                    base_attributes: Attributes {
                        strength: 8,
                        dexterity: 14,
                        intelligence: 7,
                        vitality: 9,
                    },
                    growth: Attributes {
                        strength: 1,
                        dexterity: 4,
                        intelligence: 1,
                        vitality: 2,
                    },
                    base_damage: 7.0,
                    base_armor: 4.0,
                    attacks_per_second: 1.18,
                    visual: VisualProfile {
                        asset_key: "characters/ranger.png",
                        color: Color::srgb(0.43, 0.72, 0.42),
                        size: Vec2::new(40.0, 60.0),
                    },
                },
                ClassDefinition {
                    id: ClassId::Acolyte,
                    name: "Acolyte",
                    base_attributes: Attributes {
                        strength: 5,
                        dexterity: 7,
                        intelligence: 15,
                        vitality: 8,
                    },
                    growth: Attributes {
                        strength: 1,
                        dexterity: 1,
                        intelligence: 4,
                        vitality: 2,
                    },
                    base_damage: 9.0,
                    base_armor: 2.0,
                    attacks_per_second: 0.98,
                    visual: VisualProfile {
                        asset_key: "characters/acolyte.png",
                        color: Color::srgb(0.55, 0.50, 0.86),
                        size: Vec2::new(42.0, 60.0),
                    },
                },
            ],
            maps: vec![
                MapDefinition {
                    name: "Moss Gate",
                    area_level: 1,
                    finish_x: 3200.0,
                    background: Color::srgb(0.16, 0.32, 0.29),
                    packs: vec![
                        EnemyPack::new(150.0, EnemyKind::Risen, 2),
                        EnemyPack::new(620.0, EnemyKind::Risen, 3),
                        EnemyPack::new(1120.0, EnemyKind::CarrionImp, 2),
                        EnemyPack::new(1680.0, EnemyKind::Risen, 3),
                        EnemyPack::new(2250.0, EnemyKind::CarrionImp, 3),
                        EnemyPack::new(2880.0, EnemyKind::MapRare, 1),
                    ],
                },
                MapDefinition {
                    name: "Copper Hollow",
                    area_level: 2,
                    finish_x: 3600.0,
                    background: Color::srgb(0.29, 0.24, 0.18),
                    packs: vec![
                        EnemyPack::new(160.0, EnemyKind::CarrionImp, 3),
                        EnemyPack::new(700.0, EnemyKind::Risen, 4),
                        EnemyPack::new(1300.0, EnemyKind::Stonebound, 2),
                        EnemyPack::new(1910.0, EnemyKind::CarrionImp, 4),
                        EnemyPack::new(2560.0, EnemyKind::Stonebound, 3),
                        EnemyPack::new(3260.0, EnemyKind::MapRare, 1),
                    ],
                },
                MapDefinition {
                    name: "Vaal Orchard",
                    area_level: 3,
                    finish_x: 4100.0,
                    background: Color::srgb(0.20, 0.23, 0.35),
                    packs: vec![
                        EnemyPack::new(180.0, EnemyKind::Risen, 4),
                        EnemyPack::new(760.0, EnemyKind::Stonebound, 2),
                        EnemyPack::new(1420.0, EnemyKind::CarrionImp, 4),
                        EnemyPack::new(2120.0, EnemyKind::Stonebound, 3),
                        EnemyPack::new(2870.0, EnemyKind::Risen, 5),
                        EnemyPack::new(3700.0, EnemyKind::MapRare, 1),
                    ],
                },
            ],
            items: vec![
                ItemDefinition {
                    name: "Iron Splitter",
                    slot: ItemSlot::Weapon,
                    base_power: 6,
                    description: "A heavy starter blade built for steady map clearing.",
                    tint: Color::srgb(0.72, 0.74, 0.78),
                    asset_key: "items/iron_splitter.png",
                },
                ItemDefinition {
                    name: "Buckler",
                    slot: ItemSlot::Shield,
                    base_power: 4,
                    description: "A worn round shield that takes the edge off incoming hits.",
                    tint: Color::srgb(0.45, 0.58, 0.73),
                    asset_key: "items/buckler.png",
                },
                ItemDefinition {
                    name: "Scale Vest",
                    slot: ItemSlot::Armor,
                    base_power: 5,
                    description: "Overlapping metal scales that add protection and endurance.",
                    tint: Color::srgb(0.66, 0.48, 0.24),
                    asset_key: "items/scale_vest.png",
                },
                ItemDefinition {
                    name: "Verdant Band",
                    slot: ItemSlot::Trinket,
                    base_power: 3,
                    description: "A small ring carrying a pulse of green battlefield luck.",
                    tint: Color::srgb(0.34, 0.73, 0.36),
                    asset_key: "items/verdant_band.png",
                },
                ItemDefinition {
                    name: "Runed Focus",
                    slot: ItemSlot::Trinket,
                    base_power: 7,
                    description: "A carved focus that sharpens damage and keeps the wearer alive.",
                    tint: Color::srgb(0.48, 0.56, 0.88),
                    asset_key: "items/runed_focus.png",
                },
            ],
            talents: vec![
                TalentDefinition {
                    name: "Brutal Momentum",
                    max_points: 5,
                    grant: TalentGrant::DamagePercent(8.0),
                },
                TalentDefinition {
                    name: "Enduring Guard",
                    max_points: 5,
                    grant: TalentGrant::HealthPercent(7.0),
                },
                TalentDefinition {
                    name: "Cartographer's Luck",
                    max_points: 5,
                    grant: TalentGrant::LootChance(4.0),
                },
            ],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ClassId {
    Knight,
    Ranger,
    Acolyte,
}

#[derive(Clone)]
pub(crate) struct ClassDefinition {
    pub(crate) id: ClassId,
    pub(crate) name: &'static str,
    pub(crate) base_attributes: Attributes,
    pub(crate) growth: Attributes,
    pub(crate) base_damage: f32,
    pub(crate) base_armor: f32,
    pub(crate) attacks_per_second: f32,
    pub(crate) visual: VisualProfile,
}

#[derive(Clone)]
pub(crate) struct MapDefinition {
    pub(crate) name: &'static str,
    pub(crate) area_level: u32,
    pub(crate) finish_x: f32,
    pub(crate) background: Color,
    pub(crate) packs: Vec<EnemyPack>,
}

impl MapDefinition {
    pub(crate) fn total_enemies(&self) -> usize {
        self.packs.iter().map(|pack| pack.count).sum()
    }
}

#[derive(Clone)]
pub(crate) struct EnemyPack {
    pub(crate) spawn_x: f32,
    pub(crate) kind: EnemyKind,
    pub(crate) count: usize,
}

impl EnemyPack {
    fn new(spawn_x: f32, kind: EnemyKind, count: usize) -> Self {
        Self {
            spawn_x,
            kind,
            count,
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum EnemyKind {
    Risen,
    CarrionImp,
    Stonebound,
    MapRare,
}

impl EnemyKind {
    pub(crate) fn archetype(self) -> EnemyArchetype {
        match self {
            EnemyKind::Risen => EnemyArchetype {
                name: "Risen",
                max_health: 92.0,
                damage: 13.0,
                armor: 3.0,
                attacks_per_second: 0.72,
                move_speed: 46.0,
                gold_reward: 8,
                xp_reward: 8,
                item_chance: 10.0,
                visual: VisualProfile {
                    asset_key: "enemies/risen.png",
                    color: Color::srgb(0.58, 0.62, 0.66),
                    size: Vec2::new(38.0, 48.0),
                },
            },
            EnemyKind::CarrionImp => EnemyArchetype {
                name: "Carrion Imp",
                max_health: 72.0,
                damage: 11.0,
                armor: 1.0,
                attacks_per_second: 1.05,
                move_speed: 64.0,
                gold_reward: 6,
                xp_reward: 7,
                item_chance: 9.0,
                visual: VisualProfile {
                    asset_key: "enemies/carrion_imp.png",
                    color: Color::srgb(0.78, 0.35, 0.34),
                    size: Vec2::new(34.0, 38.0),
                },
            },
            EnemyKind::Stonebound => EnemyArchetype {
                name: "Stonebound",
                max_health: 155.0,
                damage: 20.0,
                armor: 10.0,
                attacks_per_second: 0.58,
                move_speed: 38.0,
                gold_reward: 13,
                xp_reward: 12,
                item_chance: 14.0,
                visual: VisualProfile {
                    asset_key: "enemies/stonebound.png",
                    color: Color::srgb(0.46, 0.42, 0.36),
                    size: Vec2::new(50.0, 58.0),
                },
            },
            EnemyKind::MapRare => EnemyArchetype {
                name: "Map Rare",
                max_health: 360.0,
                damage: 30.0,
                armor: 14.0,
                attacks_per_second: 0.68,
                move_speed: 42.0,
                gold_reward: 38,
                xp_reward: 38,
                item_chance: 48.0,
                visual: VisualProfile {
                    asset_key: "enemies/map_rare.png",
                    color: Color::srgb(0.86, 0.62, 0.22),
                    size: Vec2::new(62.0, 72.0),
                },
            },
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct EnemyArchetype {
    pub(crate) name: &'static str,
    pub(crate) max_health: f32,
    pub(crate) damage: f32,
    pub(crate) armor: f32,
    pub(crate) attacks_per_second: f32,
    pub(crate) move_speed: f32,
    pub(crate) gold_reward: u32,
    pub(crate) xp_reward: u32,
    pub(crate) item_chance: f32,
    pub(crate) visual: VisualProfile,
}

#[derive(Clone, Copy)]
pub(crate) struct VisualProfile {
    pub(crate) asset_key: &'static str,
    pub(crate) color: Color,
    pub(crate) size: Vec2,
}

#[derive(Clone)]
pub(crate) struct ItemDefinition {
    pub(crate) name: &'static str,
    pub(crate) slot: ItemSlot,
    pub(crate) base_power: u32,
    pub(crate) description: &'static str,
    pub(crate) tint: Color,
    pub(crate) asset_key: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ItemSlot {
    Weapon,
    Shield,
    Armor,
    Trinket,
}

impl ItemSlot {
    pub(crate) fn index(self) -> usize {
        match self {
            ItemSlot::Weapon => 0,
            ItemSlot::Shield => 1,
            ItemSlot::Armor => 2,
            ItemSlot::Trinket => 3,
        }
    }

    pub(crate) fn name(self) -> &'static str {
        match self {
            ItemSlot::Weapon => "Weapon",
            ItemSlot::Shield => "Shield",
            ItemSlot::Armor => "Armor",
            ItemSlot::Trinket => "Trinket",
        }
    }
}

#[derive(Clone)]
pub(crate) struct ItemInstance {
    pub(crate) def_id: usize,
    pub(crate) rarity: Rarity,
    pub(crate) item_level: u32,
    pub(crate) power: u32,
}

#[derive(Clone, Copy)]
pub(crate) enum Rarity {
    Normal,
    Magic,
    Rare,
}

impl Rarity {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Rarity::Normal => "Normal",
            Rarity::Magic => "Magic",
            Rarity::Rare => "Rare",
        }
    }

    pub(crate) fn multiplier(self) -> u32 {
        match self {
            Rarity::Normal => 1,
            Rarity::Magic => 2,
            Rarity::Rare => 3,
        }
    }
}

#[derive(Clone)]
pub(crate) struct TalentDefinition {
    pub(crate) name: &'static str,
    pub(crate) max_points: u8,
    pub(crate) grant: TalentGrant,
}

#[derive(Clone, Copy)]
pub(crate) enum TalentGrant {
    DamagePercent(f32),
    HealthPercent(f32),
    LootChance(f32),
}

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

#[derive(Resource)]
pub(crate) struct PlayerProfile {
    pub(crate) class_id: ClassId,
    pub(crate) level: u32,
    pub(crate) xp: u32,
    pub(crate) gold: u32,
    pub(crate) talent_points: u8,
    pub(crate) allocated_talents: Vec<u8>,
    pub(crate) inventory: Vec<Option<ItemInstance>>,
    pub(crate) stash: Vec<Option<ItemInstance>>,
    pub(crate) equipment: Vec<Option<ItemInstance>>,
    pub(crate) respawns: u32,
}

impl Default for PlayerProfile {
    fn default() -> Self {
        Self {
            class_id: ClassId::Knight,
            level: 1,
            xp: 0,
            gold: 0,
            talent_points: 0,
            allocated_talents: vec![0, 0, 0],
            inventory: vec![None; INVENTORY_SIZE],
            stash: vec![None; STASH_SIZE],
            equipment: vec![None; EQUIPMENT_SLOT_COUNT],
            respawns: 0,
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
        class
            .base_attributes
            .scaled_add(class.growth, self.level - 1)
    }

    pub(crate) fn derived_stats(&self, database: &GameDatabase) -> DerivedStats {
        let class = self.class(database);
        let attributes = self.attributes(database);
        let mut item_damage = 0.0;
        let mut item_armor = 0.0;
        let mut item_health = 0.0;

        for item in self.equipment.iter().flatten() {
            let definition = &database.items[item.def_id];
            let power = item.power as f32;
            match definition.slot {
                ItemSlot::Weapon => item_damage += power * 0.9,
                ItemSlot::Shield => item_armor += power * 0.65,
                ItemSlot::Armor => {
                    item_armor += power * 1.1;
                    item_health += power * 1.8;
                }
                ItemSlot::Trinket => {
                    item_damage += power * 0.3;
                    item_health += power * 0.9;
                }
            }
        }

        let mut damage_multiplier = 1.0;
        let mut health_multiplier = 1.0;
        let mut loot_bonus = 0.0;
        for (index, points) in self.allocated_talents.iter().enumerate() {
            if *points == 0 {
                continue;
            }
            match database.talents[index].grant {
                TalentGrant::DamagePercent(percent) => {
                    damage_multiplier += percent * *points as f32 / 100.0;
                }
                TalentGrant::HealthPercent(percent) => {
                    health_multiplier += percent * *points as f32 / 100.0;
                }
                TalentGrant::LootChance(percent) => {
                    loot_bonus += percent * *points as f32;
                }
            }
        }

        let max_health = (65.0
            + attributes.vitality as f32 * 5.8
            + attributes.strength as f32 * 1.4
            + item_health)
            * health_multiplier;
        let damage = (class.base_damage
            + attributes.strength as f32 * 0.55
            + attributes.dexterity as f32 * 0.25
            + attributes.intelligence as f32 * 0.18
            + item_damage)
            * damage_multiplier;
        let armor = class.base_armor + attributes.strength as f32 * 0.30 + item_armor;
        let attacks_per_second =
            (class.attacks_per_second + attributes.dexterity as f32 * 0.006).clamp(0.45, 1.8);

        DerivedStats {
            max_health,
            damage,
            armor,
            attacks_per_second,
            loot_bonus,
        }
    }

    pub(crate) fn xp_to_next_level(&self) -> u32 {
        260 + self.level.pow(2) * 95
    }

    pub(crate) fn gain_xp(&mut self, xp: u32, database: &GameDatabase) -> bool {
        let mut leveled = false;
        self.xp += xp;
        while self.xp >= self.xp_to_next_level() {
            self.xp -= self.xp_to_next_level();
            self.level += 1;
            self.talent_points += 1;
            leveled = true;
        }
        if leveled {
            self.auto_allocate_talents(database);
        }
        leveled
    }

    fn auto_allocate_talents(&mut self, database: &GameDatabase) {
        while self.talent_points > 0 {
            let Some(index) = self
                .allocated_talents
                .iter()
                .enumerate()
                .find(|(index, points)| **points < database.talents[*index].max_points)
                .map(|(index, _)| index)
            else {
                break;
            };
            self.allocated_talents[index] += 1;
            self.talent_points -= 1;
        }
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

    pub(crate) fn item_at(&self, location: ItemLocation) -> Option<&ItemInstance> {
        match location {
            ItemLocation::Inventory(index) => self.inventory.get(index),
            ItemLocation::Stash(index) => self.stash.get(index),
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
            ItemLocation::Equipment(_) => {
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
            ItemLocation::Inventory(_) | ItemLocation::Stash(_) => self.location_exists(location),
            ItemLocation::Equipment(index) => {
                self.location_exists(location) && database.items[item.def_id].slot.index() == index
            }
        }
    }

    fn take_item(&mut self, location: ItemLocation) -> Option<ItemInstance> {
        match location {
            ItemLocation::Inventory(index) => self.inventory.get_mut(index),
            ItemLocation::Stash(index) => self.stash.get_mut(index),
            ItemLocation::Equipment(index) => self.equipment.get_mut(index),
        }
        .and_then(Option::take)
    }

    fn place_item_unchecked(&mut self, location: ItemLocation, item: ItemInstance) {
        let slot = match location {
            ItemLocation::Inventory(index) => self.inventory.get_mut(index),
            ItemLocation::Stash(index) => self.stash.get_mut(index),
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
    Equipment(usize),
}

#[derive(Resource)]
pub(crate) struct RunState {
    pub(crate) status: RunStatus,
    pub(crate) map_index: usize,
    pub(crate) atlas_tier: u32,
    pub(crate) next_pack_index: usize,
    pub(crate) enemies_spawned: usize,
    pub(crate) enemies_defeated: usize,
    pub(crate) enemies_total: usize,
    pub(crate) next_enemy_id: u64,
    pub(crate) transition_remaining: f32,
    pub(crate) message: String,
}

impl Default for RunState {
    fn default() -> Self {
        Self {
            status: RunStatus::Running,
            map_index: 0,
            atlas_tier: 1,
            next_pack_index: 0,
            enemies_spawned: 0,
            enemies_defeated: 0,
            enemies_total: 0,
            next_enemy_id: 0,
            transition_remaining: 0.0,
            message: "Entering map".into(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum RunStatus {
    Running,
    Dead,
    Cleared,
}

#[derive(Resource)]
pub(crate) struct LootRng {
    state: u64,
}

impl Default for LootRng {
    fn default() -> Self {
        Self {
            state: 0xace5_2026_0519,
        }
    }
}

impl LootRng {
    fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.state >> 32) as u32
    }

    pub(crate) fn range(&mut self, max: usize) -> usize {
        (self.next_u32() as usize) % max
    }

    pub(crate) fn percent(&mut self) -> f32 {
        (self.next_u32() % 10_000) as f32 / 100.0
    }
}

pub(crate) fn seed_starting_equipment(profile: &mut PlayerProfile, database: &GameDatabase) {
    if profile.equipment.iter().any(Option::is_some) {
        return;
    }

    let weapon = ItemInstance {
        def_id: 0,
        rarity: Rarity::Normal,
        item_level: 1,
        power: database.items[0].base_power,
    };
    let shield = ItemInstance {
        def_id: 1,
        rarity: Rarity::Normal,
        item_level: 1,
        power: database.items[1].base_power,
    };
    let weapon_slot = database.items[weapon.def_id].slot.index();
    let shield_slot = database.items[shield.def_id].slot.index();
    profile.equipment[weapon_slot] = Some(weapon);
    profile.equipment[shield_slot] = Some(shield);
}

pub(crate) fn roll_item(
    rng: &mut LootRng,
    database: &GameDatabase,
    item_level: u32,
) -> ItemInstance {
    let def_id = rng.range(database.items.len());
    let rarity = match rng.percent() {
        roll if roll >= 92.0 => Rarity::Rare,
        roll if roll >= 68.0 => Rarity::Magic,
        _ => Rarity::Normal,
    };
    let definition = &database.items[def_id];
    let power =
        definition.base_power + item_level + rarity.multiplier() * (1 + rng.range(3) as u32);
    ItemInstance {
        def_id,
        rarity,
        item_level,
        power,
    }
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

pub(crate) fn item_damage_bonus(item: &ItemInstance, definition: &ItemDefinition) -> f32 {
    let power = item.power as f32;
    match definition.slot {
        ItemSlot::Weapon => power * 0.9,
        ItemSlot::Trinket => power * 0.3,
        ItemSlot::Shield | ItemSlot::Armor => 0.0,
    }
}

pub(crate) fn item_armor_bonus(item: &ItemInstance, definition: &ItemDefinition) -> f32 {
    let power = item.power as f32;
    match definition.slot {
        ItemSlot::Shield => power * 0.65,
        ItemSlot::Armor => power * 1.1,
        ItemSlot::Weapon | ItemSlot::Trinket => 0.0,
    }
}

pub(crate) fn item_life_bonus(item: &ItemInstance, definition: &ItemDefinition) -> f32 {
    let power = item.power as f32;
    match definition.slot {
        ItemSlot::Armor => power * 1.8,
        ItemSlot::Trinket => power * 0.9,
        ItemSlot::Weapon | ItemSlot::Shield => 0.0,
    }
}

pub(crate) fn item_slot_effect(slot: ItemSlot) -> &'static str {
    match slot {
        ItemSlot::Weapon => "Effect: increases auto-attack hit damage.",
        ItemSlot::Shield => "Effect: reduces incoming hit damage through armor.",
        ItemSlot::Armor => "Effect: grants armor and maximum life.",
        ItemSlot::Trinket => "Effect: adds a mix of damage and survival stats.",
    }
}

pub(crate) fn rarity_effect(rarity: Rarity) -> Option<&'static str> {
    match rarity {
        Rarity::Normal => None,
        Rarity::Magic => Some("Magic effect: stronger single modifier roll."),
        Rarity::Rare => Some("Rare effect: multiple stronger modifier rolls."),
    }
}

pub(crate) fn rarity_color(rarity: Rarity) -> Color {
    match rarity {
        Rarity::Normal => Color::srgb(0.72, 0.72, 0.68),
        Rarity::Magic => Color::srgb(0.32, 0.49, 0.95),
        Rarity::Rare => Color::srgb(0.96, 0.70, 0.26),
    }
}

pub(crate) fn damage_after_armor(raw_damage: f32, armor: f32) -> f32 {
    (raw_damage - armor * 0.16).max(1.0)
}
