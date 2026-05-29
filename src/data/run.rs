use bevy::prelude::*;

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
    pub(crate) fn from_state(state: u64) -> Self {
        Self { state }
    }

    pub(crate) fn state(&self) -> u64 {
        self.state
    }

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

    pub(crate) fn unit(&mut self) -> f32 {
        self.percent() / 100.0
    }
}

pub(crate) fn damage_after_armor(raw_damage: f32, armor: f32) -> f32 {
    (raw_damage - armor * 0.16).max(1.0)
}
