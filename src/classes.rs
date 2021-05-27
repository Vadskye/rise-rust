pub mod archetype_rank_abilities;
pub mod archetypes;
pub mod latex;

use crate::core_mechanics::defenses::Defense;
use crate::core_mechanics::resources::Resource;
use crate::equipment;
use crate::skills::{KnowledgeSubskill, Skill};
use std::fmt;

pub enum Class {
    Barbarian,
    Cleric,
    Rogue,
}

pub static BARBARIAN: &'static Class = &Class::Barbarian;
pub static CLERIC: &'static Class = &Class::Cleric;
pub static ROGUE: &'static Class = &Class::Rogue;

impl Class {
    pub fn attunement_points(&self) -> i8 {
        match self {
            Self::Barbarian => 1,
            Self::Cleric => 2,
            Self::Rogue => 1,
        }
    }

    pub fn archetypes(&self) -> Vec<archetypes::ClassArchetype> {
        return archetypes::all_archetypes()
            .into_iter()
            .filter(|a| a.class().name() == self.name())
            .collect();
    }

    pub fn alignment(&self) -> &str {
        match self {
            _ => "Any",
        }
    }

    pub fn class_skills(&self) -> Vec<Skill> {
        match self {
            Self::Barbarian => vec![
                Skill::Agility,
                Skill::Awareness,
                Skill::Climb,
                Skill::Craft,
                Skill::CreatureHandling,
                Skill::Deception,
                Skill::Endurance,
                Skill::Flexibility,
                Skill::Intimidate,
                Skill::Jump,
                Skill::Medicine,
                Skill::Persuasion,
                Skill::Profession,
                Skill::Ride,
                Skill::Survival,
                Skill::Swim,
            ],
            Self::Cleric => vec![
                Skill::Awareness,
                Skill::Craft,
                Skill::Deception,
                Skill::Deduction,
                Skill::Intimidate,
                Skill::Knowledge(vec![
                    KnowledgeSubskill::Arcana,
                    KnowledgeSubskill::Local,
                    KnowledgeSubskill::Religion,
                    KnowledgeSubskill::Planes,
                ]),
                Skill::Linguistics,
                Skill::Medicine,
                Skill::Persuasion,
                Skill::Profession,
                Skill::SocialInsight,
                Skill::Spellsense,
            ],
            Self::Rogue => vec![
                Skill::Agility,
                Skill::Awareness,
                Skill::Climb,
                Skill::Craft,
                Skill::Deception,
                Skill::Deduction,
                Skill::Devices,
                Skill::Disguise,
                Skill::Flexibility,
                Skill::Intimidate,
                Skill::Jump,
                Skill::Knowledge(vec![
                    KnowledgeSubskill::Dungeoneering,
                    KnowledgeSubskill::Local,
                ]),
                Skill::Linguistics,
                Skill::Perform,
                Skill::Persuasion,
                Skill::Profession,
                Skill::Ride,
                Skill::SleightOfHand,
                Skill::SocialInsight,
                Skill::Stealth,
                Skill::Swim,
            ],
        }
    }

    pub fn defense_bonus(&self, defense: &'static Defense) -> i8 {
        match self {
            Self::Barbarian => match defense {
                Defense::Armor => 1,
                Defense::Fortitude => 7,
                Defense::Reflex => 5,
                Defense::Mental => 3,
            },
            Self::Cleric => match defense {
                Defense::Armor => 1,
                Defense::Fortitude => 5,
                Defense::Reflex => 3,
                Defense::Mental => 7,
            },
            Self::Rogue => match defense {
                Defense::Armor => 1,
                Defense::Fortitude => 3,
                Defense::Reflex => 7,
                Defense::Mental => 5,
            },
        }
    }

    pub fn fatigue_tolerance(&self) -> i8 {
        match self {
            Self::Barbarian => 4,
            Self::Cleric => 1,
            Self::Rogue => 2,
        }
    }

    pub fn insight_points(&self) -> i8 {
        match self {
            Self::Barbarian => 1,
            Self::Cleric => 3,
            Self::Rogue => 3,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Barbarian => "barbarian",
            Self::Cleric => "cleric",
            Self::Rogue => "rogue",
        }
    }

    pub fn resource_bonus(&self, resource: &'static Resource) -> i8 {
        match self {
            Self::Barbarian => match resource {
                Resource::AttunementPoint => 1,
                Resource::FatigueTolerance => 4,
                Resource::InsightPoint => 1,
                Resource::SkillPoint => 9,
            },
            Self::Cleric => match resource {
                Resource::AttunementPoint => 2,
                Resource::FatigueTolerance => 1,
                Resource::InsightPoint => 3,
                Resource::SkillPoint => 6,
            },
            Self::Rogue => match resource {
                Resource::AttunementPoint => 1,
                Resource::FatigueTolerance => 2,
                Resource::InsightPoint => 3,
                Resource::SkillPoint => 12,
            },
        }
    }

    pub fn shorthand_name(&self) -> &str {
        match self {
            Self::Barbarian => "Bbn",
            Self::Cleric => "Clr",
            Self::Rogue => "Rog",
        }
    }

    pub fn skill_points(&self) -> i8 {
        match self {
            Self::Barbarian => 9,
            Self::Cleric => 6,
            Self::Rogue => 12,
        }
    }

    pub fn to_latex(&self) -> String {
        latex::generate_latex_class_definition(self)
    }

    pub fn armor_proficiencies(&self) -> Vec<equipment::ArmorUsageClass> {
        match self {
            Self::Barbarian => vec![
                equipment::ArmorUsageClass::Light,
                equipment::ArmorUsageClass::Medium,
            ],
            Self::Cleric => vec![
                equipment::ArmorUsageClass::Light,
                equipment::ArmorUsageClass::Medium,
            ],
            Self::Rogue => vec![equipment::ArmorUsageClass::Light],
        }
    }

    pub fn weapon_proficiencies(&self) -> WeaponProficiencies {
        match self {
            Self::Barbarian => WeaponProficiencies {
                custom_weapon_groups: 2,
                specific_weapons: None,
                simple_weapons: true,
            },
            Self::Cleric => WeaponProficiencies {
                custom_weapon_groups: 1,
                specific_weapons: None,
                simple_weapons: true,
            },
            Self::Rogue => WeaponProficiencies {
                custom_weapon_groups: 1,
                specific_weapons: Some(vec![equipment::Weapon::Sap]),
                simple_weapons: true,
            },
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug)]
pub struct ClassDefenseBonuses {
    pub armor: i8,
    pub fortitude: i8,
    pub mental: i8,
    pub reflex: i8,
}

pub struct WeaponProficiencies {
    pub custom_weapon_groups: i8,
    pub specific_weapons: Option<Vec<equipment::Weapon>>,
    pub simple_weapons: bool,
}

#[derive(Debug)]
pub struct ClassResources {
    pub attunement_points: i8,
    pub defenses: ClassDefenseBonuses,
    pub fatigue_tolerance: i8,
    pub insight_points: i8,
    pub skill_points: i8,
}
