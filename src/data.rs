use bevy::prelude::*;

use crate::constants::{
    CRAFTING_SLOT_COUNT, EQUIPMENT_SLOT_COUNT, INVENTORY_SIZE, PLAYER_SPEED, STASH_SIZE,
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
            maps: vec![
                MapDefinition {
                    name: "Moss Gate",
                    area_level: 1,
                    finish_x: 2550.0,
                    background: Color::srgb(0.16, 0.32, 0.29),
                    packs: vec![
                        EnemyPack::new(150.0, EnemyKind::Risen, 1),
                        EnemyPack::new(460.0, EnemyKind::Risen, 2),
                        EnemyPack::new(820.0, EnemyKind::CarrionImp, 2),
                        EnemyPack::new(1210.0, EnemyKind::Risen, 2),
                        EnemyPack::new(1650.0, EnemyKind::CarrionImp, 2),
                        EnemyPack::new(2120.0, EnemyKind::MapRare, 1),
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
    const fn empty() -> Self {
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
    const fn new(min: f32, max: f32) -> Self {
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

#[derive(Clone)]
pub(crate) struct ItemInstance {
    pub(crate) def_id: usize,
    pub(crate) rarity: Rarity,
    pub(crate) item_level: u32,
    pub(crate) power: u32,
    pub(crate) rolls: ItemStatRolls,
}

#[derive(Clone, Copy, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq)]
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

#[derive(Clone, Copy)]
pub(crate) struct TalentNode {
    pub(crate) name: &'static str,
    pub(crate) flavor: &'static str,
    pub(crate) max_points: u8,
    pub(crate) grant: TalentGrant,
    pub(crate) requires: Option<usize>,
    pub(crate) position: Vec2,
}

#[derive(Clone, Copy)]
pub(crate) enum TalentGrant {
    Damage(f32),
    Life(f32),
    Armor(f32),
    AttackSpeed(f32),
    CritChance(f32),
    CritDamage(f32),
    MoveSpeed(f32),
    LifeRegen(f32),
    LootChance(f32),
    Strength(f32),
    Dexterity(f32),
    Intelligence(f32),
    Vitality(f32),
}

impl TalentGrant {
    fn per_point(self) -> f32 {
        match self {
            TalentGrant::Damage(v)
            | TalentGrant::Life(v)
            | TalentGrant::Armor(v)
            | TalentGrant::AttackSpeed(v)
            | TalentGrant::CritChance(v)
            | TalentGrant::CritDamage(v)
            | TalentGrant::MoveSpeed(v)
            | TalentGrant::LifeRegen(v)
            | TalentGrant::LootChance(v)
            | TalentGrant::Strength(v)
            | TalentGrant::Dexterity(v)
            | TalentGrant::Intelligence(v)
            | TalentGrant::Vitality(v) => v,
        }
    }

    fn apply(self, effects: &mut TalentEffects, points: f32) {
        let total = self.per_point() * points;
        match self {
            TalentGrant::Damage(_) => effects.damage_percent += total,
            TalentGrant::Life(_) => effects.life_percent += total,
            TalentGrant::Armor(_) => effects.armor_percent += total,
            TalentGrant::AttackSpeed(_) => effects.attack_speed_percent += total,
            TalentGrant::CritChance(_) => effects.crit_chance += total,
            TalentGrant::CritDamage(_) => effects.crit_damage += total,
            TalentGrant::MoveSpeed(_) => effects.move_speed_percent += total,
            TalentGrant::LifeRegen(_) => effects.life_regen += total,
            TalentGrant::LootChance(_) => effects.loot_chance += total,
            TalentGrant::Strength(_) => effects.strength += total,
            TalentGrant::Dexterity(_) => effects.dexterity += total,
            TalentGrant::Intelligence(_) => effects.intelligence += total,
            TalentGrant::Vitality(_) => effects.vitality += total,
        }
    }

    pub(crate) fn effect_line(self, points: u8) -> String {
        let total = self.per_point() * points as f32;
        match self {
            TalentGrant::Damage(_) => format!("+{total:.0}% increased Damage"),
            TalentGrant::Life(_) => format!("+{total:.0}% increased Life"),
            TalentGrant::Armor(_) => format!("+{total:.0}% increased Armor"),
            TalentGrant::AttackSpeed(_) => format!("+{total:.0}% increased Attack Speed"),
            TalentGrant::CritChance(_) => format!("+{total:.1}% Critical Chance"),
            TalentGrant::CritDamage(_) => format!("+{total:.0}% Critical Damage"),
            TalentGrant::MoveSpeed(_) => format!("+{total:.0}% increased Move Speed"),
            TalentGrant::LifeRegen(_) => format!("+{total:.1} Life Regen / s"),
            TalentGrant::LootChance(_) => format!("+{total:.0}% Loot Chance"),
            TalentGrant::Strength(_) => format!("+{total:.0} Strength"),
            TalentGrant::Dexterity(_) => format!("+{total:.0} Dexterity"),
            TalentGrant::Intelligence(_) => format!("+{total:.0} Intelligence"),
            TalentGrant::Vitality(_) => format!("+{total:.0} Vitality"),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct TalentEffects {
    pub(crate) damage_percent: f32,
    pub(crate) life_percent: f32,
    pub(crate) armor_percent: f32,
    pub(crate) attack_speed_percent: f32,
    pub(crate) crit_chance: f32,
    pub(crate) crit_damage: f32,
    pub(crate) move_speed_percent: f32,
    pub(crate) life_regen: f32,
    pub(crate) loot_chance: f32,
    pub(crate) strength: f32,
    pub(crate) dexterity: f32,
    pub(crate) intelligence: f32,
    pub(crate) vitality: f32,
}

fn tn(
    name: &'static str,
    flavor: &'static str,
    max_points: u8,
    grant: TalentGrant,
    requires: Option<usize>,
    x: f32,
    y: f32,
) -> TalentNode {
    TalentNode {
        name,
        flavor,
        max_points,
        grant,
        requires,
        position: Vec2::new(x, y),
    }
}

fn knight_talents() -> Vec<TalentNode> {
    vec![
        tn(
            "Bulwark Stance",
            "Raise your shield and harden your guard.",
            3,
            TalentGrant::Armor(12.0),
            None,
            -250.0,
            250.0,
        ),
        tn(
            "Iron Discipline",
            "Years of drills temper raw might.",
            3,
            TalentGrant::Strength(5.0),
            Some(0),
            -370.0,
            175.0,
        ),
        tn(
            "Crushing Blows",
            "Every swing lands with crushing force.",
            5,
            TalentGrant::Damage(9.0),
            Some(1),
            -445.0,
            95.0,
        ),
        tn(
            "Bloodlust",
            "The thrill of battle quickens your strikes.",
            3,
            TalentGrant::AttackSpeed(6.0),
            Some(1),
            -300.0,
            95.0,
        ),
        tn(
            "Executioner's Edge",
            "Strike the weak point and end it.",
            3,
            TalentGrant::CritDamage(22.0),
            Some(2),
            -445.0,
            5.0,
        ),
        tn(
            "Warlord's Wrath",
            "Lead the charge with unstoppable fury.",
            1,
            TalentGrant::Damage(18.0),
            Some(4),
            -445.0,
            -78.0,
        ),
        tn(
            "Stalwart Heart",
            "A warrior's resolve fortifies the body.",
            5,
            TalentGrant::Life(9.0),
            None,
            -130.0,
            175.0,
        ),
        tn(
            "Plated Resolve",
            "Layered plate turns aside the deadliest blows.",
            3,
            TalentGrant::Armor(16.0),
            Some(6),
            -55.0,
            95.0,
        ),
        tn(
            "Second Wind",
            "Recover swiftly between clashes.",
            3,
            TalentGrant::LifeRegen(2.2),
            Some(6),
            -200.0,
            95.0,
        ),
        tn(
            "Unyielding",
            "Nothing short of death will stop you.",
            3,
            TalentGrant::Vitality(6.0),
            Some(7),
            -55.0,
            5.0,
        ),
        tn(
            "Aegis Eternal",
            "Become an unbreakable wall.",
            1,
            TalentGrant::Life(20.0),
            Some(9),
            -55.0,
            -78.0,
        ),
    ]
}

fn ranger_talents() -> Vec<TalentNode> {
    vec![
        tn(
            "Hunter's Focus",
            "Sharpen your senses for the hunt.",
            3,
            TalentGrant::Dexterity(5.0),
            None,
            -250.0,
            250.0,
        ),
        tn(
            "Fleet Footed",
            "Move like the wind through the wilds.",
            3,
            TalentGrant::MoveSpeed(8.0),
            Some(0),
            -370.0,
            175.0,
        ),
        tn(
            "Quickdraw",
            "Loose arrows faster than the eye can follow.",
            5,
            TalentGrant::AttackSpeed(7.0),
            Some(1),
            -445.0,
            95.0,
        ),
        tn(
            "Deadeye",
            "Never miss the mark.",
            5,
            TalentGrant::CritChance(5.0),
            Some(1),
            -300.0,
            95.0,
        ),
        tn(
            "Lethal Precision",
            "Find the gap in any armor.",
            3,
            TalentGrant::CritDamage(20.0),
            Some(3),
            -300.0,
            5.0,
        ),
        tn(
            "Assassinate",
            "One shot, one kill.",
            1,
            TalentGrant::CritChance(6.0),
            Some(4),
            -300.0,
            -78.0,
        ),
        tn(
            "Survivalist",
            "Endure the harshest terrain.",
            5,
            TalentGrant::Life(7.0),
            None,
            -130.0,
            175.0,
        ),
        tn(
            "Evasion",
            "Slip away before the blow lands.",
            3,
            TalentGrant::Armor(14.0),
            Some(6),
            -55.0,
            95.0,
        ),
        tn(
            "Cartographer's Eye",
            "Spot treasure others overlook.",
            3,
            TalentGrant::LootChance(5.0),
            Some(6),
            -200.0,
            95.0,
        ),
        tn(
            "Windrunner",
            "Outpace every foe on the field.",
            3,
            TalentGrant::MoveSpeed(10.0),
            Some(7),
            -55.0,
            5.0,
        ),
        tn(
            "Storm of Arrows",
            "Unleash a relentless volley.",
            1,
            TalentGrant::AttackSpeed(12.0),
            Some(9),
            -55.0,
            -78.0,
        ),
    ]
}

fn acolyte_talents() -> Vec<TalentNode> {
    vec![
        tn(
            "Arcane Spark",
            "Awaken the latent power within.",
            3,
            TalentGrant::Intelligence(5.0),
            None,
            -250.0,
            250.0,
        ),
        tn(
            "Kindled Mind",
            "Focus your will into raw destruction.",
            5,
            TalentGrant::Damage(10.0),
            Some(0),
            -370.0,
            175.0,
        ),
        tn(
            "Searing Focus",
            "Channel power into precise, burning strikes.",
            5,
            TalentGrant::CritChance(4.0),
            Some(1),
            -445.0,
            95.0,
        ),
        tn(
            "Empowered Strikes",
            "Amplify each cast with surging energy.",
            3,
            TalentGrant::Damage(12.0),
            Some(1),
            -300.0,
            95.0,
        ),
        tn(
            "Cataclysm",
            "Let devastation follow your every spell.",
            3,
            TalentGrant::CritDamage(25.0),
            Some(3),
            -300.0,
            5.0,
        ),
        tn(
            "Annihilation",
            "Reduce your enemies to ash.",
            1,
            TalentGrant::Damage(20.0),
            Some(4),
            -300.0,
            -78.0,
        ),
        tn(
            "Mana Ward",
            "Weave protective wards into your flesh.",
            5,
            TalentGrant::Life(8.0),
            None,
            -130.0,
            175.0,
        ),
        tn(
            "Runic Armor",
            "Etch runes that deflect harm.",
            3,
            TalentGrant::Armor(15.0),
            Some(6),
            -55.0,
            95.0,
        ),
        tn(
            "Lifeweave",
            "Knit your wounds with arcane threads.",
            3,
            TalentGrant::LifeRegen(2.5),
            Some(6),
            -200.0,
            95.0,
        ),
        tn(
            "Intellect Mastery",
            "Master the deepest arcane truths.",
            3,
            TalentGrant::Intelligence(8.0),
            Some(7),
            -55.0,
            5.0,
        ),
        tn(
            "Eternal Font",
            "Tap an endless wellspring of vitality.",
            1,
            TalentGrant::Life(18.0),
            Some(9),
            -55.0,
            -78.0,
        ),
    ]
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
    pub(crate) allocated_talents: Vec<u8>,
    pub(crate) inventory: Vec<Option<ItemInstance>>,
    pub(crate) stash: Vec<Option<ItemInstance>>,
    pub(crate) crafting: Vec<Option<ItemInstance>>,
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
            allocated_talents: Vec::new(),
            inventory: vec![None; INVENTORY_SIZE],
            stash: vec![None; STASH_SIZE],
            crafting: vec![None; CRAFTING_SLOT_COUNT],
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

    pub(crate) fn crafting_upgrade_preview(&self) -> CraftingPreview {
        let Some(first_item) = self.crafting.first().and_then(Option::as_ref) else {
            return CraftingPreview::NeedsItems;
        };
        let rarity = first_item.rarity;
        let mut count = 0;

        for slot in &self.crafting {
            let Some(item) = slot else {
                return CraftingPreview::NeedsItems;
            };
            count += 1;
            if item.rarity != rarity {
                return CraftingPreview::RarityMismatch;
            }
        }

        if count != CRAFTING_SLOT_COUNT {
            return CraftingPreview::NeedsItems;
        }

        let Some(next_rarity) = rarity.next() else {
            return CraftingPreview::MaxRarity;
        };

        CraftingPreview::Ready {
            from: rarity,
            to: next_rarity,
        }
    }

    pub(crate) fn craft_rarity_upgrade(
        &mut self,
        database: &GameDatabase,
        rng: &mut LootRng,
    ) -> CraftingResult {
        let CraftingPreview::Ready { to, .. } = self.crafting_upgrade_preview() else {
            return match self.crafting_upgrade_preview() {
                CraftingPreview::NeedsItems => CraftingResult::NeedsItems,
                CraftingPreview::RarityMismatch => CraftingResult::RarityMismatch,
                CraftingPreview::MaxRarity => CraftingResult::MaxRarity,
                CraftingPreview::Ready { .. } => unreachable!(),
            };
        };

        let item_level = self
            .crafting
            .iter()
            .filter_map(Option::as_ref)
            .map(|item| item.item_level)
            .max()
            .unwrap_or(1);

        for slot in &mut self.crafting {
            *slot = None;
        }

        let item = roll_item_of_rarity(rng, database, item_level, to);
        let crafted_item = item.clone();
        let destination = match self.add_item(item) {
            ItemDestination::Inventory => CraftingDestination::Inventory,
            ItemDestination::Stash => CraftingDestination::Stash,
            ItemDestination::Lost => {
                if let Some(slot) = self.crafting.iter_mut().find(|slot| slot.is_none()) {
                    *slot = Some(crafted_item.clone());
                    CraftingDestination::Cube
                } else {
                    CraftingDestination::Lost
                }
            }
        };

        CraftingResult::Crafted {
            item: crafted_item,
            destination,
        }
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

pub(crate) enum CraftingPreview {
    NeedsItems,
    RarityMismatch,
    MaxRarity,
    Ready { from: Rarity, to: Rarity },
}

pub(crate) enum CraftingDestination {
    Inventory,
    Stash,
    Cube,
    Lost,
}

pub(crate) enum CraftingResult {
    Crafted {
        item: ItemInstance,
        destination: CraftingDestination,
    },
    NeedsItems,
    RarityMismatch,
    MaxRarity,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ItemLocation {
    Inventory(usize),
    Stash(usize),
    Crafting(usize),
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

    fn unit(&mut self) -> f32 {
        self.percent() / 100.0
    }
}

pub(crate) fn seed_starting_equipment(profile: &mut PlayerProfile, database: &GameDatabase) {
    if profile.equipment.iter().any(Option::is_some) {
        return;
    }

    let weapon = starter_item(0, database);
    let shield = starter_item(1, database);
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

pub(crate) fn damage_after_armor(raw_damage: f32, armor: f32) -> f32 {
    (raw_damage - armor * 0.16).max(1.0)
}
