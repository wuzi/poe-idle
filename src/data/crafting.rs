use crate::constants::CRAFTING_SLOT_COUNT;

use super::{
    GameDatabase, ItemDestination, ItemInstance, LootRng, PlayerProfile, Rarity, item_gold_value,
    roll_item_of_rarity,
};

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

pub(crate) enum LiquidationResult {
    Liquidated { items: usize, gold: u32 },
    Empty,
}

impl PlayerProfile {
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

    pub(crate) fn crafting_liquidation_value(&self) -> u32 {
        self.crafting
            .iter()
            .filter_map(Option::as_ref)
            .fold(0_u32, |total, item| {
                total.saturating_add(item_gold_value(item))
            })
    }

    pub(crate) fn crafting_liquidation_count(&self) -> usize {
        self.crafting.iter().filter(|slot| slot.is_some()).count()
    }

    pub(crate) fn liquidate_crafting_items(&mut self) -> LiquidationResult {
        let mut sold_items = 0;
        let mut gold = 0_u32;

        for slot in &mut self.crafting {
            if let Some(item) = slot.take() {
                sold_items += 1;
                gold = gold.saturating_add(item_gold_value(&item));
            }
        }

        if sold_items == 0 {
            return LiquidationResult::Empty;
        }

        self.gold = self.gold.saturating_add(gold);
        LiquidationResult::Liquidated {
            items: sold_items,
            gold,
        }
    }
}
