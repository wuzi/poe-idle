#![allow(
    clippy::cargo,
    clippy::complexity,
    clippy::expect_used,
    clippy::nursery,
    clippy::pedantic,
    clippy::style
)]

use bevy::prelude::*;
use bevy::sprite::Anchor;

const WINDOW_WIDTH: u32 = 720;
const WINDOW_HEIGHT: u32 = 960;
const PLAYER_START_X: f32 = -260.0;
const PLAYER_Y: f32 = -250.0;
const PLAYER_SPEED: f32 = 88.0;
const SPAWN_AHEAD_DISTANCE: f32 = 560.0;
const PLAYER_ATTACK_RANGE: f32 = 58.0;
const ENEMY_ATTACK_RANGE: f32 = 52.0;
const INVENTORY_SIZE: usize = 24;
const STASH_SIZE: usize = 30;
const EQUIPMENT_SLOT_COUNT: usize = 4;
const INVENTORY_CELL_SIZE: f32 = 34.0;
const TOOLTIP_WIDTH: f32 = 278.0;
const TOOLTIP_PADDING: f32 = 12.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.07, 0.08)))
        .insert_resource(GameDatabase::default())
        .insert_resource(PlayerProfile::default())
        .insert_resource(RunState::default())
        .insert_resource(LootRng::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PoE Idle Prototype".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_map_transitions,
                spawn_enemy_packs,
                move_player,
                move_enemies,
                player_attack,
                enemies_attack,
                resolve_combat_outcomes,
                tick_timed_entities,
                sync_health_bars,
                sync_character_visuals,
                camera_follow,
                update_item_tooltip,
                sync_screen_fixed_entities,
                sync_progress_bar,
                sync_inventory_grid,
                sync_hud_text,
            )
                .chain(),
        )
        .run();
}

#[derive(Resource)]
struct GameDatabase {
    classes: Vec<ClassDefinition>,
    maps: Vec<MapDefinition>,
    items: Vec<ItemDefinition>,
    talents: Vec<TalentDefinition>,
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
                    finish_x: 2150.0,
                    background: Color::srgb(0.16, 0.32, 0.29),
                    packs: vec![
                        EnemyPack::new(150.0, EnemyKind::Risen, 2),
                        EnemyPack::new(430.0, EnemyKind::Risen, 3),
                        EnemyPack::new(770.0, EnemyKind::CarrionImp, 2),
                        EnemyPack::new(1120.0, EnemyKind::Risen, 3),
                        EnemyPack::new(1510.0, EnemyKind::CarrionImp, 3),
                        EnemyPack::new(1900.0, EnemyKind::MapRare, 1),
                    ],
                },
                MapDefinition {
                    name: "Copper Hollow",
                    area_level: 2,
                    finish_x: 2450.0,
                    background: Color::srgb(0.29, 0.24, 0.18),
                    packs: vec![
                        EnemyPack::new(160.0, EnemyKind::CarrionImp, 3),
                        EnemyPack::new(520.0, EnemyKind::Risen, 4),
                        EnemyPack::new(900.0, EnemyKind::Stonebound, 2),
                        EnemyPack::new(1290.0, EnemyKind::CarrionImp, 4),
                        EnemyPack::new(1710.0, EnemyKind::Stonebound, 3),
                        EnemyPack::new(2180.0, EnemyKind::MapRare, 1),
                    ],
                },
                MapDefinition {
                    name: "Vaal Orchard",
                    area_level: 3,
                    finish_x: 2700.0,
                    background: Color::srgb(0.20, 0.23, 0.35),
                    packs: vec![
                        EnemyPack::new(180.0, EnemyKind::Risen, 4),
                        EnemyPack::new(560.0, EnemyKind::Stonebound, 2),
                        EnemyPack::new(960.0, EnemyKind::CarrionImp, 4),
                        EnemyPack::new(1390.0, EnemyKind::Stonebound, 3),
                        EnemyPack::new(1840.0, EnemyKind::Risen, 5),
                        EnemyPack::new(2350.0, EnemyKind::MapRare, 1),
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
enum ClassId {
    Knight,
    Ranger,
    Acolyte,
}

#[derive(Clone)]
struct ClassDefinition {
    id: ClassId,
    name: &'static str,
    base_attributes: Attributes,
    growth: Attributes,
    base_damage: f32,
    base_armor: f32,
    attacks_per_second: f32,
    visual: VisualProfile,
}

#[derive(Clone)]
struct MapDefinition {
    name: &'static str,
    area_level: u32,
    finish_x: f32,
    background: Color,
    packs: Vec<EnemyPack>,
}

impl MapDefinition {
    fn total_enemies(&self) -> usize {
        self.packs.iter().map(|pack| pack.count).sum()
    }
}

#[derive(Clone)]
struct EnemyPack {
    spawn_x: f32,
    kind: EnemyKind,
    count: usize,
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
enum EnemyKind {
    Risen,
    CarrionImp,
    Stonebound,
    MapRare,
}

impl EnemyKind {
    fn archetype(self) -> EnemyArchetype {
        match self {
            EnemyKind::Risen => EnemyArchetype {
                name: "Risen",
                max_health: 34.0,
                damage: 5.0,
                armor: 1.0,
                attacks_per_second: 0.75,
                move_speed: 55.0,
                gold_reward: 8,
                xp_reward: 16,
                item_chance: 18.0,
                visual: VisualProfile {
                    asset_key: "enemies/risen.png",
                    color: Color::srgb(0.58, 0.62, 0.66),
                    size: Vec2::new(38.0, 48.0),
                },
            },
            EnemyKind::CarrionImp => EnemyArchetype {
                name: "Carrion Imp",
                max_health: 24.0,
                damage: 4.0,
                armor: 0.0,
                attacks_per_second: 1.12,
                move_speed: 78.0,
                gold_reward: 6,
                xp_reward: 14,
                item_chance: 16.0,
                visual: VisualProfile {
                    asset_key: "enemies/carrion_imp.png",
                    color: Color::srgb(0.78, 0.35, 0.34),
                    size: Vec2::new(34.0, 38.0),
                },
            },
            EnemyKind::Stonebound => EnemyArchetype {
                name: "Stonebound",
                max_health: 58.0,
                damage: 8.0,
                armor: 5.0,
                attacks_per_second: 0.55,
                move_speed: 42.0,
                gold_reward: 13,
                xp_reward: 24,
                item_chance: 24.0,
                visual: VisualProfile {
                    asset_key: "enemies/stonebound.png",
                    color: Color::srgb(0.46, 0.42, 0.36),
                    size: Vec2::new(50.0, 58.0),
                },
            },
            EnemyKind::MapRare => EnemyArchetype {
                name: "Map Rare",
                max_health: 128.0,
                damage: 12.0,
                armor: 6.0,
                attacks_per_second: 0.72,
                move_speed: 48.0,
                gold_reward: 38,
                xp_reward: 72,
                item_chance: 78.0,
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
struct EnemyArchetype {
    name: &'static str,
    max_health: f32,
    damage: f32,
    armor: f32,
    attacks_per_second: f32,
    move_speed: f32,
    gold_reward: u32,
    xp_reward: u32,
    item_chance: f32,
    visual: VisualProfile,
}

#[derive(Clone, Copy)]
struct VisualProfile {
    asset_key: &'static str,
    color: Color,
    size: Vec2,
}

#[derive(Clone)]
struct ItemDefinition {
    name: &'static str,
    slot: ItemSlot,
    base_power: u32,
    description: &'static str,
    tint: Color,
    asset_key: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ItemSlot {
    Weapon,
    Shield,
    Armor,
    Trinket,
}

impl ItemSlot {
    fn index(self) -> usize {
        match self {
            ItemSlot::Weapon => 0,
            ItemSlot::Shield => 1,
            ItemSlot::Armor => 2,
            ItemSlot::Trinket => 3,
        }
    }

    fn name(self) -> &'static str {
        match self {
            ItemSlot::Weapon => "Weapon",
            ItemSlot::Shield => "Shield",
            ItemSlot::Armor => "Armor",
            ItemSlot::Trinket => "Trinket",
        }
    }
}

#[derive(Clone)]
struct ItemInstance {
    def_id: usize,
    rarity: Rarity,
    item_level: u32,
    power: u32,
}

#[derive(Clone, Copy)]
enum Rarity {
    Normal,
    Magic,
    Rare,
}

impl Rarity {
    fn name(self) -> &'static str {
        match self {
            Rarity::Normal => "Normal",
            Rarity::Magic => "Magic",
            Rarity::Rare => "Rare",
        }
    }

    fn multiplier(self) -> u32 {
        match self {
            Rarity::Normal => 1,
            Rarity::Magic => 2,
            Rarity::Rare => 3,
        }
    }
}

#[derive(Clone)]
struct TalentDefinition {
    name: &'static str,
    max_points: u8,
    grant: TalentGrant,
}

#[derive(Clone, Copy)]
enum TalentGrant {
    DamagePercent(f32),
    HealthPercent(f32),
    LootChance(f32),
}

#[derive(Clone, Copy, Default)]
struct Attributes {
    strength: u32,
    dexterity: u32,
    intelligence: u32,
    vitality: u32,
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
struct PlayerProfile {
    class_id: ClassId,
    level: u32,
    xp: u32,
    gold: u32,
    talent_points: u8,
    allocated_talents: Vec<u8>,
    inventory: Vec<Option<ItemInstance>>,
    stash: Vec<Option<ItemInstance>>,
    equipment: Vec<Option<ItemInstance>>,
    respawns: u32,
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
    fn class<'a>(&self, database: &'a GameDatabase) -> &'a ClassDefinition {
        database
            .classes
            .iter()
            .find(|class| class.id == self.class_id)
            .expect("player class should exist")
    }

    fn attributes(&self, database: &GameDatabase) -> Attributes {
        let class = self.class(database);
        class
            .base_attributes
            .scaled_add(class.growth, self.level - 1)
    }

    fn derived_stats(&self, database: &GameDatabase) -> DerivedStats {
        let class = self.class(database);
        let attributes = self.attributes(database);
        let mut item_damage = 0.0;
        let mut item_armor = 0.0;
        let mut item_health = 0.0;

        for item in self.equipment.iter().flatten() {
            let definition = &database.items[item.def_id];
            let power = item.power as f32;
            match definition.slot {
                ItemSlot::Weapon => item_damage += power * 1.4,
                ItemSlot::Shield => item_armor += power * 1.2,
                ItemSlot::Armor => {
                    item_armor += power * 1.7;
                    item_health += power * 2.5;
                }
                ItemSlot::Trinket => {
                    item_damage += power * 0.45;
                    item_health += power * 1.2;
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

        let max_health = (78.0
            + attributes.vitality as f32 * 9.0
            + attributes.strength as f32 * 2.4
            + item_health)
            * health_multiplier;
        let damage = (class.base_damage
            + attributes.strength as f32 * 1.15
            + attributes.dexterity as f32 * 0.55
            + attributes.intelligence as f32 * 0.35
            + item_damage)
            * damage_multiplier;
        let armor = class.base_armor + attributes.strength as f32 * 0.62 + item_armor;
        let attacks_per_second =
            (class.attacks_per_second + attributes.dexterity as f32 * 0.012).clamp(0.45, 2.2);

        DerivedStats {
            max_health,
            damage,
            armor,
            attacks_per_second,
            loot_bonus,
        }
    }

    fn xp_to_next_level(&self) -> u32 {
        90 + self.level * 45
    }

    fn gain_xp(&mut self, xp: u32, database: &GameDatabase) -> bool {
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

    fn add_item(&mut self, item: ItemInstance, database: &GameDatabase) -> ItemDestination {
        if self.try_auto_equip(item.clone(), database) {
            return ItemDestination::Equipped;
        }

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

    fn try_auto_equip(&mut self, item: ItemInstance, database: &GameDatabase) -> bool {
        let slot_index = database.items[item.def_id].slot.index();
        let is_upgrade = self.equipment[slot_index]
            .as_ref()
            .map(|equipped| item.power > equipped.power)
            .unwrap_or(true);

        if !is_upgrade {
            return false;
        }

        if let Some(old_item) = self.equipment[slot_index].replace(item) {
            if let Some(inventory_slot) = self.inventory.iter_mut().find(|slot| slot.is_none()) {
                *inventory_slot = Some(old_item);
            } else if let Some(stash_slot) = self.stash.iter_mut().find(|slot| slot.is_none()) {
                *stash_slot = Some(old_item);
            }
        }

        true
    }
}

struct DerivedStats {
    max_health: f32,
    damage: f32,
    armor: f32,
    attacks_per_second: f32,
    loot_bonus: f32,
}

enum ItemDestination {
    Equipped,
    Inventory,
    Stash,
    Lost,
}

#[derive(Resource)]
struct RunState {
    status: RunStatus,
    map_index: usize,
    atlas_tier: u32,
    next_pack_index: usize,
    enemies_spawned: usize,
    enemies_defeated: usize,
    enemies_total: usize,
    next_enemy_id: u64,
    transition_remaining: f32,
    message: String,
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
enum RunStatus {
    Running,
    Dead,
    Cleared,
}

#[derive(Resource)]
struct LootRng {
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

    fn range(&mut self, max: usize) -> usize {
        (self.next_u32() as usize) % max
    }

    fn percent(&mut self) -> f32 {
        (self.next_u32() % 10_000) as f32 / 100.0
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy {
    id: u64,
    name: &'static str,
    gold_reward: u32,
    xp_reward: u32,
    item_chance: f32,
    damage: f32,
    armor: f32,
    attacks_per_second: f32,
    move_speed: f32,
}

#[derive(Component)]
struct Health {
    current: f32,
    max: f32,
}

#[derive(Component)]
struct AttackClock {
    remaining: f32,
}

#[derive(Component)]
struct MapEntity;

#[derive(Component)]
struct CharacterVisual {
    base_color: Color,
}

#[derive(Component)]
struct ScreenFixed {
    offset: Vec3,
}

#[derive(Component)]
enum HudText {
    Header,
    Stats,
    Equipment,
    Talents,
    Message,
}

#[derive(Component)]
struct InventoryCell {
    source: InventorySource,
    index: usize,
}

#[derive(Clone, Copy)]
enum InventorySource {
    Inventory,
    Stash,
    Equipment,
}

#[derive(Component)]
struct ProgressBarFill;

#[derive(Component)]
struct ItemTooltipBackground;

#[derive(Component)]
struct ItemTooltipText;

#[derive(Component)]
struct HealthBar {
    target: Entity,
    width: f32,
    y_offset: f32,
    is_fill: bool,
}

#[derive(Component)]
struct TimedDespawn {
    remaining: f32,
    total: f32,
    drift_y: f32,
}

fn setup(
    mut commands: Commands,
    database: Res<GameDatabase>,
    mut profile: ResMut<PlayerProfile>,
    mut run: ResMut<RunState>,
) {
    commands.spawn((Camera2d, MainCamera));
    spawn_background(&mut commands, &database.maps[run.map_index]);

    let stats = profile.derived_stats(&database);
    let class = profile.class(&database);
    let player = spawn_placeholder_actor(
        &mut commands,
        class.visual,
        Vec3::new(PLAYER_START_X, PLAYER_Y, 4.0),
        (
            Player,
            Health {
                current: stats.max_health,
                max: stats.max_health,
            },
            AttackClock { remaining: 0.0 },
            CharacterVisual {
                base_color: class.visual.color,
            },
        ),
    );
    spawn_health_bar(&mut commands, player, 68.0, 50.0, false);
    spawn_screen_layout(&mut commands);
    seed_starting_equipment(&mut profile, &database);
    begin_current_map(&mut run, &database);
}

fn seed_starting_equipment(profile: &mut PlayerProfile, database: &GameDatabase) {
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
    profile.try_auto_equip(weapon, database);
    profile.try_auto_equip(shield, database);
}

fn spawn_background(commands: &mut Commands, map: &MapDefinition) {
    spawn_rect(
        commands,
        map.background,
        Vec2::new(3800.0, 960.0),
        Vec3::new(1150.0, 0.0, -10.0),
    );
    spawn_rect(
        commands,
        Color::srgba(0.09, 0.13, 0.13, 0.85),
        Vec2::new(3900.0, 190.0),
        Vec3::new(1150.0, -355.0, -4.0),
    );

    for index in 0..9 {
        let x = -360.0 + index as f32 * 430.0;
        let height = 150.0 + (index % 3) as f32 * 46.0;
        spawn_rect(
            commands,
            Color::srgba(0.08, 0.17, 0.18, 0.9),
            Vec2::new(250.0, height),
            Vec3::new(x, -145.0 + height * 0.25, -8.0),
        );
    }

    for index in 0..18 {
        let x = -360.0 + index as f32 * 210.0;
        spawn_rect(
            commands,
            Color::srgb(0.14, 0.29, 0.20),
            Vec2::new(160.0, 24.0),
            Vec3::new(x, PLAYER_Y - 48.0, 0.0),
        );
        spawn_rect(
            commands,
            Color::srgb(0.08, 0.12, 0.10),
            Vec2::new(160.0, 28.0),
            Vec3::new(x, PLAYER_Y - 72.0, -1.0),
        );
    }
}

fn spawn_screen_layout(commands: &mut Commands) {
    spawn_fixed_rect(
        commands,
        Vec3::new(0.0, 420.0, 30.0),
        Vec2::new(668.0, 92.0),
        Color::srgba(0.08, 0.08, 0.09, 0.93),
    );
    spawn_fixed_rect(
        commands,
        Vec3::new(0.0, 350.0, 30.0),
        Vec2::new(668.0, 34.0),
        Color::srgba(0.25, 0.13, 0.08, 0.92),
    );
    spawn_fixed_rect(
        commands,
        Vec3::new(0.0, -342.0, 30.0),
        Vec2::new(668.0, 242.0),
        Color::srgba(0.07, 0.07, 0.08, 0.94),
    );
    spawn_fixed_rect(
        commands,
        Vec3::new(-168.0, 322.0, 31.0),
        Vec2::new(320.0, 12.0),
        Color::srgba(0.02, 0.02, 0.02, 0.90),
    );

    commands.spawn((
        Sprite::from_color(Color::srgb(0.94, 0.66, 0.22), Vec2::new(1.0, 12.0)),
        Transform::from_xyz(-328.0, 322.0, 32.0),
        ScreenFixed {
            offset: Vec3::new(-328.0, 322.0, 32.0),
        },
        ProgressBarFill,
    ));

    spawn_fixed_text(
        commands,
        HudText::Header,
        Vec3::new(-315.0, 448.0, 35.0),
        18.0,
    );
    spawn_fixed_text(
        commands,
        HudText::Stats,
        Vec3::new(-315.0, 405.0, 35.0),
        15.0,
    );
    spawn_fixed_text(
        commands,
        HudText::Equipment,
        Vec3::new(82.0, 448.0, 35.0),
        14.0,
    );
    spawn_fixed_text(
        commands,
        HudText::Talents,
        Vec3::new(82.0, 407.0, 35.0),
        14.0,
    );
    spawn_fixed_text(
        commands,
        HudText::Message,
        Vec3::new(-315.0, 360.0, 35.0),
        16.0,
    );

    spawn_fixed_label(commands, "Inventory", Vec3::new(-290.0, -236.0, 35.0), 17.0);
    spawn_fixed_label(commands, "Stash", Vec3::new(60.0, -236.0, 35.0), 17.0);
    spawn_fixed_label(commands, "Equipped", Vec3::new(60.0, -382.0, 35.0), 15.0);

    spawn_inventory_cells(
        commands,
        InventorySource::Inventory,
        -292.0,
        -282.0,
        6,
        4,
        42.0,
    );
    spawn_inventory_cells(commands, InventorySource::Stash, 60.0, -282.0, 5, 3, 42.0);
    spawn_inventory_cells(
        commands,
        InventorySource::Equipment,
        60.0,
        -422.0,
        4,
        1,
        42.0,
    );
    spawn_item_tooltip(commands);
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
                ScreenFixed { offset },
                InventoryCell { source, index },
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

fn spawn_fixed_label(commands: &mut Commands, label: &'static str, offset: Vec3, font_size: f32) {
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
        ScreenFixed { offset },
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
}

fn begin_current_map(run: &mut RunState, database: &GameDatabase) {
    let map = &database.maps[run.map_index];
    run.status = RunStatus::Running;
    run.next_pack_index = 0;
    run.enemies_spawned = 0;
    run.enemies_defeated = 0;
    run.enemies_total = map.total_enemies();
    run.transition_remaining = 0.0;
    run.message = format!("{} map opened", map.name);
}

fn handle_map_transitions(
    time: Res<Time>,
    database: Res<GameDatabase>,
    mut run: ResMut<RunState>,
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut Health, &mut AttackClock), With<Player>>,
    cleanup_query: Query<Entity, With<MapEntity>>,
    mut profile: ResMut<PlayerProfile>,
) {
    if run.status == RunStatus::Running {
        return;
    }

    run.transition_remaining -= time.delta_secs();
    if run.transition_remaining > 0.0 {
        return;
    }

    if run.status == RunStatus::Cleared {
        run.map_index = (run.map_index + 1) % database.maps.len();
        if run.map_index == 0 {
            run.atlas_tier += 1;
        }
    }

    for entity in &cleanup_query {
        commands.entity(entity).despawn();
    }

    if let Ok((mut transform, mut health, mut clock)) = player_query.single_mut() {
        let stats = profile.derived_stats(&database);
        transform.translation.x = PLAYER_START_X;
        transform.translation.y = PLAYER_Y;
        health.max = stats.max_health;
        health.current = stats.max_health;
        clock.remaining = 0.0;
    }

    if run.status == RunStatus::Dead {
        profile.respawns += 1;
    }

    begin_current_map(&mut run, &database);
}

fn spawn_enemy_packs(
    mut commands: Commands,
    database: Res<GameDatabase>,
    mut run: ResMut<RunState>,
    player_query: Query<&Transform, With<Player>>,
) {
    if run.status != RunStatus::Running {
        return;
    }

    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let map = &database.maps[run.map_index];

    while let Some(pack) = map.packs.get(run.next_pack_index) {
        if pack.spawn_x > player_transform.translation.x + SPAWN_AHEAD_DISTANCE {
            break;
        }

        let archetype = pack.kind.archetype();
        for index in 0..pack.count {
            let spawn_x = pack.spawn_x + index as f32 * 54.0;
            let tier_scale =
                1.0 + (run.atlas_tier - 1) as f32 * 0.16 + map.area_level as f32 * 0.08;
            let enemy_id = run.next_enemy_id;
            run.next_enemy_id += 1;
            let enemy = spawn_placeholder_actor(
                &mut commands,
                archetype.visual,
                Vec3::new(spawn_x, PLAYER_Y + 4.0, 3.0),
                (
                    Enemy {
                        id: enemy_id,
                        name: archetype.name,
                        gold_reward: (archetype.gold_reward as f32 * tier_scale) as u32,
                        xp_reward: (archetype.xp_reward as f32 * tier_scale) as u32,
                        item_chance: archetype.item_chance,
                        damage: archetype.damage * tier_scale,
                        armor: archetype.armor * tier_scale,
                        attacks_per_second: archetype.attacks_per_second,
                        move_speed: archetype.move_speed,
                    },
                    Health {
                        current: archetype.max_health * tier_scale,
                        max: archetype.max_health * tier_scale,
                    },
                    AttackClock { remaining: 0.45 },
                    CharacterVisual {
                        base_color: archetype.visual.color,
                    },
                    MapEntity,
                ),
            );
            spawn_health_bar(
                &mut commands,
                enemy,
                archetype.visual.size.x + 12.0,
                42.0,
                true,
            );
        }

        run.enemies_spawned += pack.count;
        run.next_pack_index += 1;
    }
}

fn move_player(
    time: Res<Time>,
    run: Res<RunState>,
    mut player_query: Query<&mut Transform, With<Player>>,
    enemy_query: Query<(&Transform, &Health), (With<Enemy>, Without<Player>)>,
) {
    if run.status != RunStatus::Running {
        return;
    }

    let Ok(mut player_transform) = player_query.single_mut() else {
        return;
    };

    let enemy_blocks_path = enemy_query.iter().any(|(enemy_transform, health)| {
        health.current > 0.0
            && enemy_transform.translation.x > player_transform.translation.x - 8.0
            && enemy_transform.translation.x - player_transform.translation.x < PLAYER_ATTACK_RANGE
    });

    if !enemy_blocks_path {
        player_transform.translation.x += PLAYER_SPEED * time.delta_secs();
    }
}

fn move_enemies(
    time: Res<Time>,
    run: Res<RunState>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &Enemy, &Health), Without<Player>>,
) {
    if run.status != RunStatus::Running {
        return;
    }

    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (mut enemy_transform, enemy, health) in &mut enemy_query {
        if health.current <= 0.0 {
            continue;
        }
        let distance = enemy_transform.translation.x - player_transform.translation.x;
        if distance > ENEMY_ATTACK_RANGE {
            enemy_transform.translation.x -= enemy.move_speed * time.delta_secs();
        }
        if enemy_transform.translation.x < player_transform.translation.x + 32.0 {
            enemy_transform.translation.x = player_transform.translation.x + 32.0;
        }
    }
}

fn player_attack(
    time: Res<Time>,
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    run: Res<RunState>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut AttackClock), With<Player>>,
    mut enemy_targets: ParamSet<(
        Query<(Entity, &Transform, &Health), With<Enemy>>,
        Query<(&Transform, &mut Health, &Enemy), With<Enemy>>,
    )>,
) {
    if run.status != RunStatus::Running {
        return;
    }

    let Ok((player_transform, mut clock)) = player_query.single_mut() else {
        return;
    };
    clock.remaining = (clock.remaining - time.delta_secs()).max(0.0);
    if clock.remaining > 0.0 {
        return;
    }

    let mut target = None;
    let mut best_distance = f32::MAX;
    for (entity, enemy_transform, health) in &enemy_targets.p0() {
        let distance = enemy_transform.translation.x - player_transform.translation.x;
        if health.current > 0.0
            && (0.0..=PLAYER_ATTACK_RANGE).contains(&distance)
            && distance < best_distance
        {
            target = Some(entity);
            best_distance = distance;
        }
    }

    let Some(target) = target else {
        return;
    };

    let stats = profile.derived_stats(&database);
    if let Ok((enemy_transform, mut health, enemy)) = enemy_targets.p1().get_mut(target) {
        let damage = damage_after_armor(stats.damage, enemy.armor);
        health.current -= damage;
        clock.remaining = 1.0 / stats.attacks_per_second;
        spawn_floating_text(
            &mut commands,
            format!("-{damage:.0}"),
            enemy_transform.translation + Vec3::new(0.0, 46.0, 20.0),
            Color::srgb(1.0, 0.83, 0.45),
        );
    }
}

fn enemies_attack(
    time: Res<Time>,
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    run: Res<RunState>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Health), With<Player>>,
    mut enemy_query: Query<(&Transform, &Enemy, &Health, &mut AttackClock), Without<Player>>,
) {
    if run.status != RunStatus::Running {
        return;
    }

    let Ok((player_transform, mut player_health)) = player_query.single_mut() else {
        return;
    };
    let player_stats = profile.derived_stats(&database);

    for (enemy_transform, enemy, enemy_health, mut clock) in &mut enemy_query {
        if enemy_health.current <= 0.0 {
            continue;
        }
        clock.remaining = (clock.remaining - time.delta_secs()).max(0.0);
        let distance = enemy_transform.translation.x - player_transform.translation.x;
        if distance > ENEMY_ATTACK_RANGE || clock.remaining > 0.0 {
            continue;
        }

        let damage = damage_after_armor(enemy.damage, player_stats.armor);
        player_health.current -= damage;
        clock.remaining = 1.0 / enemy.attacks_per_second;
        spawn_floating_text(
            &mut commands,
            format!("-{damage:.0}"),
            player_transform.translation + Vec3::new(0.0, 58.0, 20.0),
            Color::srgb(1.0, 0.35, 0.35),
        );
    }
}

fn resolve_combat_outcomes(
    database: Res<GameDatabase>,
    mut profile: ResMut<PlayerProfile>,
    mut run: ResMut<RunState>,
    mut rng: ResMut<LootRng>,
    mut commands: Commands,
    enemy_query: Query<(Entity, &Enemy, &Health, &Transform), Without<Player>>,
    mut player_query: Query<&mut Health, With<Player>>,
) {
    if run.status != RunStatus::Running {
        return;
    }

    let mut leveled = false;
    for (entity, enemy, health, transform) in &enemy_query {
        if health.current > 0.0 {
            continue;
        }

        commands.entity(entity).despawn();
        run.enemies_defeated += 1;
        profile.gold += enemy.gold_reward;
        leveled |= profile.gain_xp(enemy.xp_reward, &database);
        run.message = format!(
            "{} #{} defeated: +{} gold",
            enemy.name, enemy.id, enemy.gold_reward
        );
        spawn_floating_text(
            &mut commands,
            format!("+{}g", enemy.gold_reward),
            transform.translation + Vec3::new(0.0, 72.0, 20.0),
            Color::srgb(0.95, 0.71, 0.28),
        );

        let loot_chance = enemy.item_chance + profile.derived_stats(&database).loot_bonus;
        if rng.percent() <= loot_chance {
            let item = roll_item(
                &mut rng,
                &database,
                run.atlas_tier + database.maps[run.map_index].area_level,
            );
            let item_name = describe_item(&item, &database);
            let item_color = database.items[item.def_id].tint;
            let destination = profile.add_item(item, &database);
            run.message = match destination {
                ItemDestination::Equipped => format!("Equipped {item_name}"),
                ItemDestination::Inventory => format!("Looted {item_name}"),
                ItemDestination::Stash => format!("Stashed {item_name}"),
                ItemDestination::Lost => format!("No room for {item_name}"),
            };
            spawn_loot_flash(&mut commands, transform.translation, item_color);
        }
    }

    if leveled {
        if let Ok(mut player_health) = player_query.single_mut() {
            let stats = profile.derived_stats(&database);
            player_health.max = stats.max_health;
            player_health.current =
                (player_health.current + stats.max_health * 0.35).min(stats.max_health);
        }
        run.message = format!("Level {} reached", profile.level);
    }

    if let Ok(player_health) = player_query.single() {
        if player_health.current <= 0.0 {
            run.status = RunStatus::Dead;
            run.transition_remaining = 2.3;
            run.message = "Defeated. Rebuilding the map".into();
            return;
        }
    }

    if run.enemies_defeated >= run.enemies_total
        && run.next_pack_index >= database.maps[run.map_index].packs.len()
    {
        run.status = RunStatus::Cleared;
        run.transition_remaining = 2.0;
        run.message = format!("{} cleared", database.maps[run.map_index].name);
    }
}

fn roll_item(rng: &mut LootRng, database: &GameDatabase, item_level: u32) -> ItemInstance {
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

fn describe_item(item: &ItemInstance, database: &GameDatabase) -> String {
    let definition = &database.items[item.def_id];
    format!(
        "{} {} ilvl {}",
        item.rarity.name(),
        definition.name,
        item.item_level
    )
}

fn item_tooltip_text(item: &ItemInstance, database: &GameDatabase) -> String {
    let definition = &database.items[item.def_id];
    let damage = item_damage_bonus(item, definition);
    let armor = item_armor_bonus(item, definition);
    let life = item_life_bonus(item, definition);
    let mut lines = vec![
        definition.name.to_string(),
        format!("{} {}", item.rarity.name(), definition.slot.name()),
        format!("Item level {}  |  Power {}", item.item_level, item.power),
        String::new(),
        definition.description.to_string(),
        String::new(),
    ];

    if damage > 0.0 {
        lines.push(format!("Damage +{damage:.0}"));
    }
    if armor > 0.0 {
        lines.push(format!("Armor +{armor:.0}"));
    }
    if life > 0.0 {
        lines.push(format!("Life +{life:.0}"));
    }
    lines.push(item_slot_effect(definition.slot).to_string());
    if let Some(extra_effect) = rarity_effect(item.rarity) {
        lines.push(extra_effect.to_string());
    }

    lines.join("\n")
}

fn item_damage_bonus(item: &ItemInstance, definition: &ItemDefinition) -> f32 {
    let power = item.power as f32;
    match definition.slot {
        ItemSlot::Weapon => power * 1.4,
        ItemSlot::Trinket => power * 0.45,
        ItemSlot::Shield | ItemSlot::Armor => 0.0,
    }
}

fn item_armor_bonus(item: &ItemInstance, definition: &ItemDefinition) -> f32 {
    let power = item.power as f32;
    match definition.slot {
        ItemSlot::Shield => power * 1.2,
        ItemSlot::Armor => power * 1.7,
        ItemSlot::Weapon | ItemSlot::Trinket => 0.0,
    }
}

fn item_life_bonus(item: &ItemInstance, definition: &ItemDefinition) -> f32 {
    let power = item.power as f32;
    match definition.slot {
        ItemSlot::Armor => power * 2.5,
        ItemSlot::Trinket => power * 1.2,
        ItemSlot::Weapon | ItemSlot::Shield => 0.0,
    }
}

fn item_slot_effect(slot: ItemSlot) -> &'static str {
    match slot {
        ItemSlot::Weapon => "Effect: increases auto-attack hit damage.",
        ItemSlot::Shield => "Effect: reduces incoming hit damage through armor.",
        ItemSlot::Armor => "Effect: grants armor and maximum life.",
        ItemSlot::Trinket => "Effect: adds a mix of damage and survival stats.",
    }
}

fn rarity_effect(rarity: Rarity) -> Option<&'static str> {
    match rarity {
        Rarity::Normal => None,
        Rarity::Magic => Some("Magic effect: stronger single modifier roll."),
        Rarity::Rare => Some("Rare effect: multiple stronger modifier rolls."),
    }
}

fn rarity_color(rarity: Rarity) -> Color {
    match rarity {
        Rarity::Normal => Color::srgb(0.72, 0.72, 0.68),
        Rarity::Magic => Color::srgb(0.32, 0.49, 0.95),
        Rarity::Rare => Color::srgb(0.96, 0.70, 0.26),
    }
}

fn damage_after_armor(raw_damage: f32, armor: f32) -> f32 {
    (raw_damage - armor * 0.28).max(1.0)
}

fn sync_character_visuals(mut query: Query<(&mut Sprite, &CharacterVisual, &Health)>) {
    for (mut sprite, visual, health) in &mut query {
        let health_ratio = (health.current / health.max).clamp(0.0, 1.0);
        sprite.color = if health_ratio < 0.35 {
            visual.base_color.mix(&Color::srgb(0.95, 0.22, 0.18), 0.45)
        } else {
            visual.base_color
        };
    }
}

fn sync_health_bars(
    mut commands: Commands,
    health_query: Query<(&Health, &Transform), Without<HealthBar>>,
    mut bar_query: Query<(Entity, &HealthBar, &mut Transform, &mut Sprite)>,
) {
    for (bar_entity, bar, mut transform, mut sprite) in &mut bar_query {
        let Ok((health, target_transform)) = health_query.get(bar.target) else {
            commands.entity(bar_entity).despawn();
            continue;
        };

        let ratio = (health.current / health.max).clamp(0.0, 1.0);
        let width = if bar.is_fill {
            (bar.width * ratio).max(1.0)
        } else {
            bar.width
        };
        sprite.custom_size = Some(Vec2::new(width, 6.0));
        transform.translation.x = target_transform.translation.x - (bar.width - width) * 0.5;
        transform.translation.y = target_transform.translation.y + bar.y_offset;
    }
}

fn tick_timed_entities(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut TimedDespawn,
        Option<&mut Transform>,
        Option<&mut TextColor>,
    )>,
) {
    for (entity, mut timed, transform, text_color) in &mut query {
        timed.remaining -= time.delta_secs();
        if let Some(mut transform) = transform {
            transform.translation.y += timed.drift_y * time.delta_secs();
        }
        if let Some(mut text_color) = text_color {
            let alpha = (timed.remaining / timed.total).clamp(0.0, 1.0);
            text_color.0 = text_color.0.with_alpha(alpha);
        }
        if timed.remaining <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };
    camera_transform.translation.x = player_transform.translation.x + 125.0;
}

fn update_item_tooltip(
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    window_query: Query<&Window>,
    cell_query: Query<
        (&InventoryCell, &ScreenFixed),
        (Without<ItemTooltipBackground>, Without<ItemTooltipText>),
    >,
    mut background_query: Query<
        (&mut ScreenFixed, &mut Sprite, &mut Visibility),
        (With<ItemTooltipBackground>, Without<ItemTooltipText>),
    >,
    mut text_query: Query<
        (
            &mut ScreenFixed,
            &mut Text2d,
            &mut TextColor,
            &mut Visibility,
        ),
        (With<ItemTooltipText>, Without<ItemTooltipBackground>),
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

    let Ok(window) = window_query.single() else {
        *background_visibility = Visibility::Hidden;
        *text_visibility = Visibility::Hidden;
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        *background_visibility = Visibility::Hidden;
        *text_visibility = Visibility::Hidden;
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
            item_for_cell(cell, &profile)
        } else {
            None
        }
    });

    let Some(item) = hovered_item else {
        *background_visibility = Visibility::Hidden;
        *text_visibility = Visibility::Hidden;
        return;
    };

    tooltip_text.0 = item_tooltip_text(item, &database);
    let line_count = tooltip_text.0.lines().count() as f32;
    let tooltip_height = (line_count * 17.0 + TOOLTIP_PADDING * 2.0).max(110.0);
    let mut top_left = cursor_offset + Vec2::new(18.0, -18.0);
    let right_edge = WINDOW_WIDTH as f32 * 0.5 - TOOLTIP_PADDING;
    let left_edge = -(WINDOW_WIDTH as f32) * 0.5 + TOOLTIP_PADDING;
    let top_edge = WINDOW_HEIGHT as f32 * 0.5 - TOOLTIP_PADDING;
    let bottom_edge = -(WINDOW_HEIGHT as f32) * 0.5 + TOOLTIP_PADDING;

    if top_left.x + TOOLTIP_WIDTH > right_edge {
        top_left.x = cursor_offset.x - TOOLTIP_WIDTH - 18.0;
    }
    if top_left.x < left_edge {
        top_left.x = left_edge;
    }
    if top_left.y > top_edge {
        top_left.y = top_edge;
    }
    if top_left.y - tooltip_height < bottom_edge {
        top_left.y = bottom_edge + tooltip_height;
    }

    let rarity_tint = rarity_color(item.rarity);
    background_sprite.custom_size = Some(Vec2::new(TOOLTIP_WIDTH, tooltip_height));
    background_sprite.color = rarity_tint.mix(&Color::srgba(0.03, 0.025, 0.025, 0.96), 0.82);
    background_fixed.offset = Vec3::new(
        top_left.x + TOOLTIP_WIDTH * 0.5,
        top_left.y - tooltip_height * 0.5,
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
}

fn sync_screen_fixed_entities(
    camera_query: Query<&Transform, (With<MainCamera>, Without<ScreenFixed>)>,
    mut fixed_query: Query<(&ScreenFixed, &mut Transform), Without<MainCamera>>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };
    for (fixed, mut transform) in &mut fixed_query {
        transform.translation.x = camera_transform.translation.x + fixed.offset.x;
        transform.translation.y = camera_transform.translation.y + fixed.offset.y;
        transform.translation.z = fixed.offset.z;
    }
}

fn sync_progress_bar(
    database: Res<GameDatabase>,
    run: Res<RunState>,
    player_query: Query<&Transform, (With<Player>, Without<ProgressBarFill>)>,
    mut query: Query<(&mut Sprite, &mut Transform), (With<ProgressBarFill>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok((mut sprite, mut transform)) = query.single_mut() else {
        return;
    };

    let map = &database.maps[run.map_index];
    let travel_progress = ((player_transform.translation.x - PLAYER_START_X)
        / (map.finish_x - PLAYER_START_X))
        .clamp(0.0, 1.0);
    let kill_progress = if run.enemies_total == 0 {
        0.0
    } else {
        run.enemies_defeated as f32 / run.enemies_total as f32
    };
    let progress = travel_progress.max(kill_progress).clamp(0.0, 1.0);
    let width = 320.0 * progress;
    sprite.custom_size = Some(Vec2::new(width.max(1.0), 12.0));
    transform.translation.x = transform.translation.x - transform.translation.x.fract();
}

fn sync_inventory_grid(
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    mut query: Query<(&InventoryCell, &mut Sprite)>,
) {
    for (cell, mut sprite) in &mut query {
        let item = item_for_cell(cell, &profile);

        sprite.color = if let Some(item) = item {
            let definition = &database.items[item.def_id];
            let _asset_key = definition.asset_key;
            match item.rarity {
                Rarity::Normal => definition.tint,
                Rarity::Magic => definition.tint.mix(&Color::srgb(0.35, 0.55, 1.0), 0.45),
                Rarity::Rare => definition.tint.mix(&Color::srgb(1.0, 0.74, 0.24), 0.55),
            }
        } else {
            Color::srgba(0.10, 0.10, 0.11, 0.98)
        };
    }
}

fn item_for_cell<'a>(cell: &InventoryCell, profile: &'a PlayerProfile) -> Option<&'a ItemInstance> {
    match cell.source {
        InventorySource::Inventory => profile.inventory.get(cell.index).and_then(Option::as_ref),
        InventorySource::Stash => profile.stash.get(cell.index).and_then(Option::as_ref),
        InventorySource::Equipment => profile.equipment.get(cell.index).and_then(Option::as_ref),
    }
}

fn sync_hud_text(
    database: Res<GameDatabase>,
    profile: Res<PlayerProfile>,
    run: Res<RunState>,
    player_query: Query<&Health, With<Player>>,
    mut query: Query<(&HudText, &mut Text2d)>,
) {
    let map = &database.maps[run.map_index];
    let class = profile.class(&database);
    let stats = profile.derived_stats(&database);
    let attributes = profile.attributes(&database);
    let health_text = player_query
        .single()
        .map(|health| format!("HP {:.0}/{:.0}", health.current.max(0.0), health.max))
        .unwrap_or_else(|_| "HP --".into());

    for (kind, mut text) in &mut query {
        text.0 = match kind {
            HudText::Header => format!(
                "{} Lv.{}  |  Gold {}  |  Atlas Tier {}",
                class.name, profile.level, profile.gold, run.atlas_tier
            ),
            HudText::Stats => format!(
                "{}  |  DMG {:.0}  ARM {:.0}  APS {:.2}\nSTR {}  DEX {}  INT {}  VIT {}",
                health_text,
                stats.damage,
                stats.armor,
                stats.attacks_per_second,
                attributes.strength,
                attributes.dexterity,
                attributes.intelligence,
                attributes.vitality
            ),
            HudText::Equipment => equipment_summary(&profile, &database),
            HudText::Talents => talent_summary(&profile, &database),
            HudText::Message => format!(
                "{}  |  {} {}/{}",
                run.message, map.name, run.enemies_defeated, run.enemies_total
            ),
        };
    }
}

fn equipment_summary(profile: &PlayerProfile, database: &GameDatabase) -> String {
    let mut lines = vec!["Equipment".to_string()];
    for slot in [
        ItemSlot::Weapon,
        ItemSlot::Shield,
        ItemSlot::Armor,
        ItemSlot::Trinket,
    ] {
        let text = profile.equipment[slot.index()]
            .as_ref()
            .map(|item| {
                let definition = &database.items[item.def_id];
                format!(
                    "{} +{} ilvl {}",
                    definition.name, item.power, item.item_level
                )
            })
            .unwrap_or_else(|| "Empty".into());
        lines.push(format!("{}: {}", slot.name(), text));
    }
    lines.join("\n")
}

fn talent_summary(profile: &PlayerProfile, database: &GameDatabase) -> String {
    let mut lines = vec![format!("Talents  Points {}", profile.talent_points)];
    for (index, talent) in database.talents.iter().enumerate() {
        lines.push(format!(
            "{} {}/{}",
            talent.name, profile.allocated_talents[index], talent.max_points
        ));
    }
    lines.join("\n")
}

fn spawn_placeholder_actor<T: Bundle>(
    commands: &mut Commands,
    visual: VisualProfile,
    translation: Vec3,
    bundle: T,
) -> Entity {
    let _asset_key = visual.asset_key;
    commands
        .spawn((
            Sprite::from_color(visual.color, visual.size),
            Transform::from_translation(translation),
            bundle,
        ))
        .id()
}

fn spawn_rect(commands: &mut Commands, color: Color, size: Vec2, translation: Vec3) -> Entity {
    commands
        .spawn((
            Sprite::from_color(color, size),
            Transform::from_translation(translation),
        ))
        .id()
}

fn spawn_health_bar(
    commands: &mut Commands,
    target: Entity,
    width: f32,
    y_offset: f32,
    is_map_entity: bool,
) {
    let mut background = commands.spawn((
        Sprite::from_color(Color::srgba(0.03, 0.03, 0.03, 0.95), Vec2::new(width, 6.0)),
        Transform::from_xyz(0.0, 0.0, 18.0),
        HealthBar {
            target,
            width,
            y_offset,
            is_fill: false,
        },
    ));
    if is_map_entity {
        background.insert(MapEntity);
    }

    let mut fill = commands.spawn((
        Sprite::from_color(Color::srgb(0.32, 0.82, 0.34), Vec2::new(width, 6.0)),
        Transform::from_xyz(0.0, 0.0, 19.0),
        HealthBar {
            target,
            width,
            y_offset,
            is_fill: true,
        },
    ));
    if is_map_entity {
        fill.insert(MapEntity);
    }
}

fn spawn_floating_text(commands: &mut Commands, text: String, translation: Vec3, color: Color) {
    commands.spawn((
        Text2d::new(text),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(Justify::Center),
        Anchor::CENTER,
        Transform::from_translation(translation),
        TimedDespawn {
            remaining: 0.75,
            total: 0.75,
            drift_y: 32.0,
        },
        MapEntity,
    ));
}

fn spawn_loot_flash(commands: &mut Commands, translation: Vec3, color: Color) {
    commands.spawn((
        Sprite::from_color(color, Vec2::splat(18.0)),
        Transform {
            translation: translation + Vec3::new(0.0, 28.0, 10.0),
            rotation: Quat::from_rotation_z(0.785),
            ..default()
        },
        TimedDespawn {
            remaining: 1.15,
            total: 1.15,
            drift_y: 10.0,
        },
        MapEntity,
    ));
}
