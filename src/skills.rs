use crate::core_mechanics::attributes::Attribute;
// use itertools::Itertools;
use std::fmt;

pub enum Skill {
    Agility,
    Awareness,
    Climb,
    Craft,
    CreatureHandling,
    Deception,
    Deduction,
    Devices,
    Disguise,
    Endurance,
    Flexibility,
    Intimidate,
    Jump,
    Knowledge,
    Linguistics,
    Medicine,
    Perform,
    Persuasion,
    Profession,
    Ride,
    SleightOfHand,
    SocialInsight,
    Spellsense,
    Stealth,
    Survival,
    Swim,
}

impl Skill {
    pub fn attribute(&self) -> Option<Attribute> {
        match self {
            Self::Agility => Some(Attribute::Dexterity),
            Self::Awareness => Some(Attribute::Perception),
            Self::Climb => Some(Attribute::Strength),
            Self::Craft => Some(Attribute::Intelligence),
            Self::CreatureHandling => Some(Attribute::Perception),
            Self::Deception => None,
            Self::Deduction => Some(Attribute::Intelligence),
            Self::Devices => Some(Attribute::Intelligence),
            Self::Disguise => Some(Attribute::Intelligence),
            Self::Endurance => Some(Attribute::Constitution),
            Self::Flexibility => Some(Attribute::Dexterity),
            Self::Intimidate => None,
            Self::Jump => Some(Attribute::Strength),
            Self::Knowledge => Some(Attribute::Intelligence),
            Self::Linguistics => Some(Attribute::Intelligence),
            Self::Medicine => Some(Attribute::Intelligence),
            Self::Perform => None,
            Self::Persuasion => None,
            Self::Profession => None,
            Self::Ride => Some(Attribute::Dexterity),
            Self::SleightOfHand => Some(Attribute::Dexterity),
            Self::SocialInsight => Some(Attribute::Perception),
            Self::Spellsense => Some(Attribute::Perception),
            Self::Stealth => Some(Attribute::Dexterity),
            Self::Survival => Some(Attribute::Perception),
            Self::Swim => Some(Attribute::Strength),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Agility => "agility",
            Self::Awareness => "awareness",
            Self::Climb => "climb",
            Self::Craft => "craft",
            Self::CreatureHandling => "creature handling",
            Self::Deception => "deception",
            Self::Deduction => "deduction",
            Self::Devices => "devices",
            Self::Disguise => "disguise",
            Self::Endurance => "endurance",
            Self::Flexibility => "flexibility",
            Self::Intimidate => "intimidate",
            Self::Jump => "jump",
            Self::Knowledge => "knowledge",
            Self::Linguistics => "linguistics",
            Self::Medicine => "medicine",
            Self::Perform => "perform",
            Self::Persuasion => "persuasion",
            Self::Profession => "profession",
            Self::Ride => "ride",
            Self::SleightOfHand => "sleight of hand",
            Self::SocialInsight => "social insight",
            Self::Spellsense => "spellsense",
            Self::Stealth => "stealth",
            Self::Survival => "survival",
            Self::Swim => "swim",
        }
    }
}

impl fmt::Display for Skill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
