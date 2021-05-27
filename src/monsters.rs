pub mod challenge_rating;
pub mod creature_type;

use crate::core_mechanics::attacks::HasAttacks;
use crate::core_mechanics::attributes::{Attribute, HasAttributes};
use crate::core_mechanics::damage_absorption::HasDamageAbsorption;
use crate::core_mechanics::defenses::HasDefenses;
use crate::core_mechanics::{creature, defenses, latex};
use crate::equipment::{weapons, HasEquipment};

pub struct Monster {
    challenge_rating: &'static challenge_rating::ChallengeRating,
    creature: creature::Creature,
    creature_type: creature_type::CreatureType,
}

impl Monster {
    pub fn new(
        challenge_rating: &'static challenge_rating::ChallengeRating,
        creature_type: creature_type::CreatureType,
        level: i8,
    ) -> Monster {
        return Monster {
            challenge_rating,
            creature_type,
            creature: creature::Creature::new(level),
        };
    }

    pub fn set_level(&mut self, level: i8) {
        self.creature.level = level;
    }

    pub fn to_latex(&self) -> String {
        return format!(
            "
                {creature_latex}
                {creature_type} {level}
            ",
            creature_latex = latex::format_creature(self),
            creature_type = self.creature_type.name(),
            level = self.creature.level,
        );
    }
}

impl HasAttributes for Monster {
    fn get_base_attribute(&self, attribute: &'static Attribute) -> i8 {
        return self.creature.get_base_attribute(attribute);
    }
    fn calc_total_attribute(&self, attribute: &'static Attribute) -> i8 {
        return self.creature.calc_total_attribute(attribute);
    }
    fn set_base_attribute(&mut self, attribute: &'static Attribute, value: i8) {
        self.creature.set_base_attribute(attribute, value);
    }
}

impl HasAttacks for Monster {
    fn calc_accuracy(&self) -> i8 {
        return self.creature.calc_accuracy()
            + self.challenge_rating.accuracy_bonus()
            + (self.creature.level + 1) / 6;
    }

    fn calc_damage_increments(&self, is_strike: bool) -> i8 {
        let level_modifier = if is_strike {
            (self.creature.level - 1) / 3
        } else {
            0
        };
        return self.creature.calc_damage_increments(is_strike)
            + self.challenge_rating.damage_increments()
            + level_modifier;
    }

    fn calc_power(&self, is_magical: bool) -> i8 {
        return self.creature.calc_power(is_magical);
    }
}

impl HasEquipment for Monster {
    fn add_weapon(&mut self, weapon: weapons::Weapon) {
        self.creature.add_weapon(weapon);
    }

    fn weapons(&self) -> &Vec<weapons::Weapon> {
        return &self.creature.weapons();
    }
}

impl HasDamageAbsorption for Monster {
    fn calc_damage_resistance(&self) -> i32 {
        return ((self.creature.calc_damage_resistance() as f64)
            * 2.0
            * self.challenge_rating.dr_multiplier()) as i32;
    }
    fn calc_hit_points(&self) -> i32 {
        return ((self.creature.calc_hit_points() as f64)
            * 1.5
            * self.challenge_rating.hp_multiplier()) as i32;
    }
}

impl HasDefenses for Monster {
    fn calc_defense(&self, defense: &'static defenses::Defense) -> i8 {
        return self.creature.calc_defense(defense)
            + self.creature_type.defense_bonus(defense)
            + self.challenge_rating.defense_bonus()
            + (self.creature.level + 3) / 6;
    }
}