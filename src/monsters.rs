pub mod challenge_rating;
pub mod creature_type;

use crate::core_mechanics::attributes::{Attribute, AttributeCalcs};
use crate::core_mechanics::defenses::DefenseCalcs;
use crate::core_mechanics::{creature, defenses};

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

    // Eventually, pulling latex from the creature won't work - a class can't modify a creature's
    // HP. However, it's convenient for now.
    pub fn to_latex(&self) -> String {
        return format!(
            "
                {creature_latex}
                {creature_type} {level}
            ",
            creature_latex = self.creature.to_latex().trim(),
            creature_type = self.creature_type.name(),
            level = self.creature.level,
        );
    }
}

impl AttributeCalcs for Monster {
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

impl creature::CoreStatistics for Monster {
    fn calc_accuracy(&self) -> i8 {
        return self.creature.calc_accuracy() + self.challenge_rating.accuracy_bonus();
    }
    fn calc_hit_points(&self) -> i32 {
        return ((self.creature.calc_hit_points() as f64) * self.challenge_rating.hp_multiplier())
            as i32;
    }
}

impl DefenseCalcs for Monster {
    fn calc_defense(&self, defense: &'static defenses::Defense) -> i8 {
        return self.creature.calc_defense(defense) + self.creature_type.defense_bonus(defense);
    }
}
