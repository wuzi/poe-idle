use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct UiState {
    pub(crate) active_panel: ActivePanel,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            active_panel: ActivePanel::Inventory,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ActivePanel {
    Inventory,
    Character,
}

#[derive(Component)]
pub(crate) struct MainCamera;

#[derive(Component)]
pub(crate) struct Player;

#[derive(Component)]
pub(crate) struct Enemy {
    pub(crate) id: u64,
    pub(crate) name: &'static str,
    pub(crate) gold_reward: u32,
    pub(crate) xp_reward: u32,
    pub(crate) item_chance: f32,
    pub(crate) damage: f32,
    pub(crate) armor: f32,
    pub(crate) attacks_per_second: f32,
    pub(crate) move_speed: f32,
}

#[derive(Component)]
pub(crate) struct Health {
    pub(crate) current: f32,
    pub(crate) max: f32,
}

#[derive(Component)]
pub(crate) struct AttackClock {
    pub(crate) remaining: f32,
}

#[derive(Component)]
pub(crate) struct MapEntity;

#[derive(Component)]
pub(crate) struct CharacterVisual {
    pub(crate) base_color: Color,
}

#[derive(Component)]
pub(crate) struct ScreenFixed {
    pub(crate) offset: Vec3,
}

#[derive(Component)]
pub(crate) enum HudText {
    Header,
    Stats,
    Message,
}

#[derive(Component)]
pub(crate) struct BottomButton {
    pub(crate) panel: ActivePanel,
    pub(crate) size: Vec2,
}

#[derive(Component)]
pub(crate) struct BottomButtonLabel {
    pub(crate) panel: ActivePanel,
}

#[derive(Component)]
pub(crate) struct CharacterPanelPiece;

#[derive(Component)]
pub(crate) enum CharacterPanelText {
    Header,
    Status,
    Combat,
    Attributes,
    Equipment,
    Talents,
    Upgrades,
}

#[derive(Component)]
pub(crate) struct InventoryCell {
    pub(crate) source: InventorySource,
    pub(crate) index: usize,
}

#[derive(Clone, Copy)]
pub(crate) enum InventorySource {
    Inventory,
    Stash,
    Equipment,
}

#[derive(Component)]
pub(crate) struct ProgressBarFill;

#[derive(Component)]
pub(crate) struct ItemTooltipBackground;

#[derive(Component)]
pub(crate) struct ItemTooltipText;

#[derive(Component)]
pub(crate) struct HealthBar {
    pub(crate) target: Entity,
    pub(crate) width: f32,
    pub(crate) y_offset: f32,
    pub(crate) is_fill: bool,
}

#[derive(Component)]
pub(crate) struct TimedDespawn {
    pub(crate) remaining: f32,
    pub(crate) total: f32,
    pub(crate) drift_y: f32,
}
