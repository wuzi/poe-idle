use bevy::prelude::*;

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

    pub(super) fn apply(self, effects: &mut TalentEffects, points: f32) {
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

pub(crate) fn knight_talents() -> Vec<TalentNode> {
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

pub(crate) fn ranger_talents() -> Vec<TalentNode> {
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

pub(crate) fn acolyte_talents() -> Vec<TalentNode> {
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
