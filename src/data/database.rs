use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    Attributes, ItemDefinition, ItemRollProfile, ItemSlot, RollRange, TalentNode, acolyte_talents,
    knight_talents, ranger_talents,
};

#[derive(Resource)]
pub(crate) struct GameDatabase {
    pub(crate) classes: Vec<ClassDefinition>,
    pub(crate) maps: Vec<MapDefinition>,
    pub(crate) items: Vec<ItemDefinition>,
}

impl Default for GameDatabase {
    fn default() -> Self {
        Self {
            classes: vec![
                ClassDefinition {
                    id: ClassId::Knight,
                    name: "Knight",
                    base_attributes: Attributes {
                        strength: 14,
                        dexterity: 10,
                        intelligence: 5,
                        vitality: 13,
                    },
                    growth: Attributes {
                        strength: 3,
                        dexterity: 1,
                        intelligence: 1,
                        vitality: 3,
                    },
                    base_damage: 10.0,
                    base_armor: 9.0,
                    attacks_per_second: 0.98,
                    visual: VisualProfile {
                        asset_key: "characters/knight.png",
                        color: Color::srgb(0.56, 0.63, 0.68),
                        size: Vec2::new(46.0, 64.0),
                    },
                    talents: knight_talents(),
                },
                ClassDefinition {
                    id: ClassId::Ranger,
                    name: "Ranger",
                    base_attributes: Attributes {
                        strength: 8,
                        dexterity: 16,
                        intelligence: 7,
                        vitality: 10,
                    },
                    growth: Attributes {
                        strength: 1,
                        dexterity: 4,
                        intelligence: 1,
                        vitality: 2,
                    },
                    base_damage: 8.5,
                    base_armor: 5.0,
                    attacks_per_second: 1.24,
                    visual: VisualProfile {
                        asset_key: "characters/ranger.png",
                        color: Color::srgb(0.43, 0.72, 0.42),
                        size: Vec2::new(40.0, 60.0),
                    },
                    talents: ranger_talents(),
                },
                ClassDefinition {
                    id: ClassId::Acolyte,
                    name: "Acolyte",
                    base_attributes: Attributes {
                        strength: 5,
                        dexterity: 7,
                        intelligence: 17,
                        vitality: 9,
                    },
                    growth: Attributes {
                        strength: 1,
                        dexterity: 1,
                        intelligence: 4,
                        vitality: 2,
                    },
                    base_damage: 10.5,
                    base_armor: 3.0,
                    attacks_per_second: 1.03,
                    visual: VisualProfile {
                        asset_key: "characters/acolyte.png",
                        color: Color::srgb(0.55, 0.50, 0.86),
                        size: Vec2::new(42.0, 60.0),
                    },
                    talents: acolyte_talents(),
                },
            ],
            maps: build_map_progression(),
            items: vec![
                ItemDefinition {
                    name: "Iron Splitter",
                    slot: ItemSlot::Weapon,
                    base_power: 6,
                    description: "A heavy starter blade with damage and attack speed rolls.",
                    tint: Color::srgb(0.72, 0.74, 0.78),
                    asset_key: "items/iron_splitter.png",
                    rolls: ItemRollProfile {
                        damage: Some(RollRange::new(5.0, 9.0)),
                        attack_speed: Some(RollRange::new(1.0, 3.0)),
                        crit_chance: Some(RollRange::new(1.0, 3.0)),
                        crit_damage: Some(RollRange::new(5.0, 12.0)),
                        ..ItemRollProfile::empty()
                    },
                },
                ItemDefinition {
                    name: "Buckler",
                    slot: ItemSlot::Shield,
                    base_power: 4,
                    description: "A worn round shield that rolls armor and life.",
                    tint: Color::srgb(0.45, 0.58, 0.73),
                    asset_key: "items/buckler.png",
                    rolls: ItemRollProfile {
                        armor: Some(RollRange::new(4.0, 8.0)),
                        max_health: Some(RollRange::new(4.0, 10.0)),
                        ..ItemRollProfile::empty()
                    },
                },
                ItemDefinition {
                    name: "Leather Cap",
                    slot: ItemSlot::Head,
                    base_power: 4,
                    description: "Light head armour with life and regeneration rolls.",
                    tint: Color::srgb(0.49, 0.36, 0.24),
                    asset_key: "items/leather_cap.png",
                    rolls: ItemRollProfile {
                        armor: Some(RollRange::new(2.0, 5.0)),
                        max_health: Some(RollRange::new(6.0, 14.0)),
                        health_regen: Some(RollRange::new(0.4, 1.2)),
                        ..ItemRollProfile::empty()
                    },
                },
                ItemDefinition {
                    name: "Scale Vest",
                    slot: ItemSlot::Chest,
                    base_power: 7,
                    description: "Overlapping metal scales with strong life and regen rolls.",
                    tint: Color::srgb(0.66, 0.48, 0.24),
                    asset_key: "items/scale_vest.png",
                    rolls: ItemRollProfile {
                        armor: Some(RollRange::new(7.0, 14.0)),
                        max_health: Some(RollRange::new(18.0, 35.0)),
                        health_regen: Some(RollRange::new(0.8, 2.0)),
                        ..ItemRollProfile::empty()
                    },
                },
                ItemDefinition {
                    name: "Iron Grips",
                    slot: ItemSlot::Gloves,
                    base_power: 5,
                    description: "Combat gloves that can roll attack speed and damage.",
                    tint: Color::srgb(0.58, 0.58, 0.61),
                    asset_key: "items/iron_grips.png",
                    rolls: ItemRollProfile {
                        damage: Some(RollRange::new(2.0, 5.0)),
                        attack_speed: Some(RollRange::new(1.0, 3.0)),
                        crit_chance: Some(RollRange::new(1.0, 3.0)),
                        crit_damage: Some(RollRange::new(4.0, 10.0)),
                        ..ItemRollProfile::empty()
                    },
                },
                ItemDefinition {
                    name: "Plate Greaves",
                    slot: ItemSlot::Legs,
                    base_power: 6,
                    description: "Leg armour with defensive and regeneration rolls.",
                    tint: Color::srgb(0.54, 0.50, 0.45),
                    asset_key: "items/plate_greaves.png",
                    rolls: ItemRollProfile {
                        armor: Some(RollRange::new(5.0, 11.0)),
                        max_health: Some(RollRange::new(12.0, 26.0)),
                        health_regen: Some(RollRange::new(0.6, 1.6)),
                        ..ItemRollProfile::empty()
                    },
                },
                ItemDefinition {
                    name: "Trail Boots",
                    slot: ItemSlot::Boots,
                    base_power: 5,
                    description: "Boots that roll movement speed for faster map travel.",
                    tint: Color::srgb(0.36, 0.45, 0.30),
                    asset_key: "items/trail_boots.png",
                    rolls: ItemRollProfile {
                        armor: Some(RollRange::new(2.0, 5.0)),
                        move_speed: Some(RollRange::new(4.0, 8.0)),
                        ..ItemRollProfile::empty()
                    },
                },
                ItemDefinition {
                    name: "Verdant Band",
                    slot: ItemSlot::Trinket,
                    base_power: 3,
                    description: "A small ring carrying a pulse of damage and survival.",
                    tint: Color::srgb(0.34, 0.73, 0.36),
                    asset_key: "items/verdant_band.png",
                    rolls: ItemRollProfile {
                        damage: Some(RollRange::new(1.0, 3.0)),
                        max_health: Some(RollRange::new(5.0, 12.0)),
                        ..ItemRollProfile::empty()
                    },
                },
                ItemDefinition {
                    name: "Runed Focus",
                    slot: ItemSlot::Trinket,
                    base_power: 7,
                    description: "A carved focus with higher damage and life rolls.",
                    tint: Color::srgb(0.48, 0.56, 0.88),
                    asset_key: "items/runed_focus.png",
                    rolls: ItemRollProfile {
                        damage: Some(RollRange::new(3.0, 7.0)),
                        max_health: Some(RollRange::new(8.0, 18.0)),
                        ..ItemRollProfile::empty()
                    },
                },
            ],
        }
    }
}

fn build_map_progression() -> Vec<MapDefinition> {
    const MAP_NAMES: [&str; 50] = [
        "Moss Gate",
        "Copper Hollow",
        "Vaal Orchard",
        "Ashen Causeway",
        "Glimmering Fen",
        "Sable Quarry",
        "Frostpine Watch",
        "Rusted Aqueduct",
        "Sunken Reliquary",
        "Emberfall Rise",
        "Graveglass Marsh",
        "Ivory Bastion",
        "Witchlight Thicket",
        "Crimson Foundry",
        "Saltwind Cliffs",
        "Moonlit Barrows",
        "Shattered Viaduct",
        "Obsidian Steppe",
        "Hollow Menagerie",
        "Stormbreak Strand",
        "Dreadroot Grove",
        "Gilded Ossuary",
        "Nightfall Terrace",
        "Cinder Monastery",
        "Bitterglass Tundra",
        "Bloodpetal Garden",
        "Forgotten Reservoir",
        "Thornwound Pass",
        "Marrowdeep Mine",
        "Starless Archives",
        "Duskvein Crossing",
        "Searing Basilica",
        "Rotcrown Mire",
        "Ironspine Rampart",
        "Whispering Necropolis",
        "Tempest Crucible",
        "Pale Serpent Road",
        "Sunfire Caldera",
        "Ebonwater Docks",
        "Mirrorbone Sanctum",
        "Wraithglass Citadel",
        "Howling Crown",
        "Feverdream Palace",
        "Blackstar Labyrinth",
        "Doomroot Expanse",
        "Celestial Furnace",
        "Voidscar Summit",
        "Ravaged Atlas",
        "Eternal Threshold",
        "Apex of Hunger",
    ];

    MAP_NAMES
        .iter()
        .enumerate()
        .map(|(index, name)| {
            let area_level = index as u32 + 1;
            let finish_x = 2450.0 + index as f32 * 135.0;
            MapDefinition {
                name,
                area_level,
                finish_x,
                background: map_background(index),
                packs: map_packs(index, finish_x),
            }
        })
        .collect()
}

fn map_background(index: usize) -> Color {
    match index % 10 {
        0 => Color::srgb(0.16, 0.32, 0.29),
        1 => Color::srgb(0.29, 0.24, 0.18),
        2 => Color::srgb(0.20, 0.23, 0.35),
        3 => Color::srgb(0.32, 0.18, 0.14),
        4 => Color::srgb(0.18, 0.28, 0.22),
        5 => Color::srgb(0.23, 0.21, 0.25),
        6 => Color::srgb(0.18, 0.26, 0.33),
        7 => Color::srgb(0.30, 0.25, 0.20),
        8 => Color::srgb(0.24, 0.18, 0.27),
        _ => Color::srgb(0.28, 0.17, 0.12),
    }
}

fn map_packs(index: usize, finish_x: f32) -> Vec<EnemyPack> {
    let pack_count = 5 + (index / 7).min(3);
    let last_pack_x = (finish_x - 360.0).max(900.0);
    let spacing = (last_pack_x - 180.0) / pack_count as f32;
    let mut packs = Vec::with_capacity(pack_count + 1);

    for pack_index in 0..pack_count {
        let kind = match (index + pack_index) % 4 {
            0 => EnemyKind::Risen,
            1 => EnemyKind::CarrionImp,
            2 => EnemyKind::Stonebound,
            _ => EnemyKind::Risen,
        };
        let count = 1 + (index / 6).min(4) + (pack_index % 3);
        packs.push(EnemyPack::new(
            180.0 + spacing * pack_index as f32,
            kind,
            count,
        ));
    }

    packs.push(EnemyPack::new(last_pack_x, EnemyKind::MapRare, 1));
    packs
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ClassId {
    #[default]
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
    pub(crate) talents: Vec<TalentNode>,
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
    pub(crate) fn recommended_enemy_level(&self) -> u32 {
        self.area_level
    }

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
                max_health: 76.0,
                damage: 9.0,
                armor: 3.0,
                attacks_per_second: 0.66,
                move_speed: 46.0,
                gold_reward: 10,
                xp_reward: 18,
                item_chance: 26.0,
                visual: VisualProfile {
                    asset_key: "enemies/risen.png",
                    color: Color::srgb(0.58, 0.62, 0.66),
                    size: Vec2::new(38.0, 48.0),
                },
            },
            EnemyKind::CarrionImp => EnemyArchetype {
                name: "Carrion Imp",
                max_health: 64.0,
                damage: 8.0,
                armor: 1.0,
                attacks_per_second: 0.92,
                move_speed: 64.0,
                gold_reward: 9,
                xp_reward: 16,
                item_chance: 24.0,
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
                max_health: 320.0,
                damage: 26.0,
                armor: 14.0,
                attacks_per_second: 0.68,
                move_speed: 42.0,
                gold_reward: 48,
                xp_reward: 52,
                item_chance: 70.0,
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
