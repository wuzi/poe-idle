use bevy::app::AppExit;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::constants::{CRAFTING_SLOT_COUNT, EQUIPMENT_SLOT_COUNT, INVENTORY_SIZE, STASH_SIZE};
use crate::data::{ClassId, GameDatabase, ItemInstance, LootRng, PlayerProfile, RunState};

const SAVE_VERSION: u32 = 1;
const SAVE_INTERVAL_SECONDS: f32 = 5.0;

#[derive(Resource)]
pub(crate) struct SaveState {
    path: PathBuf,
    timer: Timer,
    dirty: bool,
    last_error: Option<String>,
}

impl SaveState {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            timer: Timer::from_seconds(SAVE_INTERVAL_SECONDS, TimerMode::Repeating),
            dirty: true,
            last_error: None,
        }
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }
}

pub(crate) struct LoadedGame {
    pub(crate) profile: PlayerProfile,
    pub(crate) run: RunState,
    pub(crate) rng: LootRng,
    pub(crate) save_state: SaveState,
    pub(crate) loaded_from_disk: bool,
}

#[derive(Serialize, Deserialize)]
struct SaveFile {
    version: u32,
    #[serde(default)]
    profile: PlayerProfile,
    #[serde(default)]
    run: SavedRunProgress,
    #[serde(default = "default_rng_state")]
    rng_state: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
struct SavedRunProgress {
    map_index: usize,
    atlas_tier: u32,
}

impl Default for SavedRunProgress {
    fn default() -> Self {
        Self {
            map_index: 0,
            atlas_tier: 1,
        }
    }
}

impl SavedRunProgress {
    fn from_run(run: &RunState) -> Self {
        Self {
            map_index: run.map_index,
            atlas_tier: run.atlas_tier.max(1),
        }
    }
}

pub(crate) fn load_saved_game(database: &GameDatabase) -> LoadedGame {
    let path = save_file_path();
    let default_game = || LoadedGame {
        profile: PlayerProfile::default(),
        run: RunState::default(),
        rng: LootRng::default(),
        save_state: SaveState::new(path.clone()),
        loaded_from_disk: false,
    };

    let bytes = match fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return default_game(),
        Err(error) => {
            eprintln!("Could not read save file {}: {error}", path.display());
            return default_game();
        }
    };

    let save = match serde_json::from_slice::<SaveFile>(&bytes) {
        Ok(save) => save,
        Err(error) => {
            eprintln!("Could not parse save file {}: {error}", path.display());
            return default_game();
        }
    };

    if save.version != SAVE_VERSION {
        eprintln!(
            "Ignoring save file {} with unsupported version {}",
            path.display(),
            save.version
        );
        return default_game();
    }

    let mut profile = save.profile;
    repair_profile_after_load(&mut profile, database);
    if save.run.atlas_tier > 1 {
        profile.highest_unlocked_map_index = database.maps.len().saturating_sub(1);
    } else {
        profile.highest_unlocked_map_index = profile
            .highest_unlocked_map_index
            .max(save.run.map_index)
            .min(database.maps.len().saturating_sub(1));
    }

    let mut run = RunState::default();
    run.map_index = save
        .run
        .map_index
        .min(profile.highest_unlocked_map_index(database));
    run.atlas_tier = 1;

    LoadedGame {
        profile,
        run,
        rng: LootRng::from_state(save.rng_state),
        save_state: SaveState::new(path),
        loaded_from_disk: true,
    }
}

pub(crate) fn autosave_game(
    time: Res<Time>,
    mut save_state: ResMut<SaveState>,
    profile: Res<PlayerProfile>,
    run: Res<RunState>,
    rng: Res<LootRng>,
) {
    if profile.is_changed() || run.is_changed() || rng.is_changed() {
        save_state.dirty = true;
    }

    save_state.timer.tick(time.delta());
    if !save_state.dirty || !save_state.timer.just_finished() {
        return;
    }

    match write_save_file(save_state.path(), &profile, &run, &rng) {
        Ok(()) => {
            save_state.dirty = false;
            save_state.last_error = None;
        }
        Err(error) => {
            if save_state.last_error.as_deref() != Some(error.as_str()) {
                eprintln!("Could not autosave game: {error}");
            }
            save_state.last_error = Some(error);
        }
    }
}

pub(crate) fn save_on_exit(
    mut exit_events: MessageReader<AppExit>,
    save_state: Res<SaveState>,
    profile: Res<PlayerProfile>,
    run: Res<RunState>,
    rng: Res<LootRng>,
) {
    if exit_events.read().next().is_none() {
        return;
    }

    if let Err(error) = write_save_file(save_state.path(), &profile, &run, &rng) {
        eprintln!("Could not save game on exit: {error}");
    }
}

fn write_save_file(
    path: &Path,
    profile: &PlayerProfile,
    run: &RunState,
    rng: &LootRng,
) -> Result<(), String> {
    let save = SaveFile {
        version: SAVE_VERSION,
        profile: profile.clone_for_save(),
        run: SavedRunProgress::from_run(run),
        rng_state: rng.state(),
    };
    let bytes = serde_json::to_vec_pretty(&save).map_err(|error| error.to_string())?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }

    let temp_path = path.with_extension("json.tmp");
    fs::write(&temp_path, bytes).map_err(|error| error.to_string())?;
    fs::rename(&temp_path, path).map_err(|error| error.to_string())?;
    Ok(())
}

fn repair_profile_after_load(profile: &mut PlayerProfile, database: &GameDatabase) {
    if !database
        .classes
        .iter()
        .any(|class| class.id == profile.class_id)
    {
        profile.class_id = ClassId::default();
    }

    profile.level = profile.level.max(1);
    profile.inventory.resize(INVENTORY_SIZE, None);
    profile.stash.resize(STASH_SIZE, None);
    profile.crafting.resize(CRAFTING_SLOT_COUNT, None);
    profile.equipment.resize(EQUIPMENT_SLOT_COUNT, None);
    profile.highest_unlocked_map_index = profile
        .highest_unlocked_map_index
        .min(database.maps.len().saturating_sub(1));

    remove_invalid_items(&mut profile.inventory, database);
    remove_invalid_items(&mut profile.stash, database);
    remove_invalid_items(&mut profile.crafting, database);
    repair_equipment(profile, database);
    repair_talents_after_load(profile, database);
}

fn remove_invalid_items(slots: &mut [Option<ItemInstance>], database: &GameDatabase) {
    for slot in slots {
        if slot
            .as_ref()
            .is_some_and(|item| item.def_id >= database.items.len())
        {
            *slot = None;
        }
    }
}

fn repair_equipment(profile: &mut PlayerProfile, database: &GameDatabase) {
    let mut displaced_items = Vec::new();
    for (index, slot) in profile.equipment.iter_mut().enumerate() {
        let Some(item) = slot.take() else {
            continue;
        };
        let Some(definition) = database.items.get(item.def_id) else {
            continue;
        };
        if definition.slot.index() == index {
            *slot = Some(item);
        } else {
            displaced_items.push(item);
        }
    }

    for item in displaced_items {
        profile.add_item(item);
    }
}

fn repair_talents_after_load(profile: &mut PlayerProfile, database: &GameDatabase) {
    profile.ensure_talent_slots(database);

    let max_points: Vec<u8> = profile
        .talent_tree(database)
        .iter()
        .map(|node| node.max_points)
        .collect();
    for (points, max_points) in profile.allocated_talents.iter_mut().zip(max_points) {
        *points = (*points).min(max_points);
    }

    let requirements: Vec<Option<usize>> = profile
        .talent_tree(database)
        .iter()
        .map(|node| node.requires)
        .collect();
    loop {
        let mut changed = false;
        for index in 0..profile.allocated_talents.len() {
            let Some(required) = requirements[index] else {
                continue;
            };
            if required >= profile.allocated_talents.len()
                || profile.allocated_talents[required] == 0
            {
                if profile.allocated_talents[index] > 0 {
                    profile.allocated_talents[index] = 0;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }

    while profile.spent_talent_points() > profile.total_talent_points() {
        let Some(points) = profile
            .allocated_talents
            .iter_mut()
            .rev()
            .find(|points| **points > 0)
        else {
            break;
        };
        *points -= 1;
    }
}

fn save_file_path() -> PathBuf {
    if let Ok(data_home) = env::var("XDG_DATA_HOME") {
        if !data_home.is_empty() {
            return PathBuf::from(data_home).join("poe-idle").join("save.json");
        }
    }

    if let Ok(home) = env::var("HOME") {
        if !home.is_empty() {
            return PathBuf::from(home)
                .join(".local")
                .join("share")
                .join("poe-idle")
                .join("save.json");
        }
    }

    PathBuf::from("poe-idle-save.json")
}

fn default_rng_state() -> u64 {
    LootRng::default().state()
}

trait ProfileSaveClone {
    fn clone_for_save(&self) -> PlayerProfile;
}

impl ProfileSaveClone for PlayerProfile {
    fn clone_for_save(&self) -> PlayerProfile {
        PlayerProfile {
            class_id: self.class_id,
            level: self.level,
            xp: self.xp,
            gold: self.gold,
            allocated_talents: self.allocated_talents.clone(),
            inventory: self.inventory.clone(),
            stash: self.stash.clone(),
            crafting: self.crafting.clone(),
            equipment: self.equipment.clone(),
            highest_unlocked_map_index: self.highest_unlocked_map_index,
            respawns: self.respawns,
            starter_items_seeded: self.starter_items_seeded,
        }
    }
}
