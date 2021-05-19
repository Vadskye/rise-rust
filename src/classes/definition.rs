use crate::equipment;
use crate::skills::{Skill, KnowledgeSubskill};
use std::fmt;

pub enum Class {
    Barbarian,
    Cleric,
    Rogue,
}

impl Class {
    pub fn attunement_points(&self) -> u8 {
        match self {
            Self::Barbarian => 1,
            Self::Cleric => 2,
            Self::Rogue => 1,
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
                Skill::Knowledge(vec![KnowledgeSubskill::Arcana, KnowledgeSubskill::Local, KnowledgeSubskill::Religion, KnowledgeSubskill::Planes]),
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
                Skill::Knowledge(vec![KnowledgeSubskill::Dungeoneering, KnowledgeSubskill::Local]),
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

    pub fn defenses(&self) -> ClassDefenseBonuses {
        match self {
            Self::Barbarian => ClassDefenseBonuses {
                armor: 1,
                fortitude: 7,
                reflex: 5,
                mental: 3,
            },
            Self::Cleric => ClassDefenseBonuses {
                armor: 1,
                fortitude: 5,
                reflex: 3,
                mental: 7,
            },
            Self::Rogue => ClassDefenseBonuses {
                armor: 1,
                fortitude: 3,
                reflex: 7,
                mental: 5,
            },
        }
    }

    pub fn fatigue_tolerance(&self) -> u8 {
        match self {
            Self::Barbarian => 4,
            Self::Cleric => 1,
            Self::Rogue => 2,
        }
    }

    pub fn insight_points(&self) -> u8 {
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

    #[allow(dead_code)]
    fn resources(&self) -> ClassResources {
        ClassResources {
            attunement_points: self.attunement_points(),
            defenses: self.defenses(),
            fatigue_tolerance: self.fatigue_tolerance(),
            insight_points: self.insight_points(),
            skill_points: self.skill_points(),
        }
    }

    pub fn shorthand_name(&self) -> &str {
        match self {
            Self::Barbarian => "Bbn",
            Self::Cleric => "Clr",
            Self::Rogue => "Rog",
        }
    }

    pub fn skill_points(&self) -> u8 {
        match self {
            Self::Barbarian => 9,
            Self::Cleric => 6,
            Self::Rogue => 12,
        }
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
    pub armor: u8,
    pub fortitude: u8,
    pub mental: u8,
    pub reflex: u8,
}

pub struct WeaponProficiencies {
    pub custom_weapon_groups: u8,
    pub specific_weapons: Option<Vec<equipment::Weapon>>,
    pub simple_weapons: bool,
}

#[derive(Debug)]
pub struct ClassResources {
    pub attunement_points: u8,
    pub defenses: ClassDefenseBonuses,
    pub fatigue_tolerance: u8,
    pub insight_points: u8,
    pub skill_points: u8,
}
