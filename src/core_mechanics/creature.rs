use crate::core_mechanics::attributes::{self, Attribute, AttributeCalcs};
use crate::core_mechanics::defenses::{self, Defense, DefenseCalcs};
use std::collections::HashMap;

pub struct Creature {
    base_attributes: HashMap<&'static Attribute, i8>,
    pub level: i8,
}

impl Creature {
    pub fn new(level: i8) -> Creature {
        let base_attributes = HashMap::<&Attribute, i8>::new();
        return Creature {
            base_attributes,
            level,
        };
    }

    pub fn set_level(&mut self, level: i8) {
        self.level = level;
    }

    pub fn to_latex(&self) -> String {
        return format!(
            "
                HP {hit_points}, AD {armor}, Fort {fortitude}, Ref {reflex}, Ment {mental}
                Attr: {attributes}
            ",
            attributes = format_creature_attributes(self).join(", "),
            armor = self.calc_defense(defenses::ARMOR),
            fortitude = self.calc_defense(defenses::FORT),
            hit_points = self.calc_hit_points(),
            mental = self.calc_defense(defenses::MENT),
            reflex = self.calc_defense(defenses::REF),
        );
    }
}

impl AttributeCalcs for Creature {
    fn get_base_attribute(&self, attribute: &'static Attribute) -> i8 {
        if let Some(a) = self.base_attributes.get(attribute) {
            *a
        } else {
            0
        }
    }

    fn calc_total_attribute(&self, attribute: &'static Attribute) -> i8 {
        Attribute::calculate_total(self.get_base_attribute(attribute), self.level)
    }

    fn set_base_attribute(&mut self, attribute: &'static Attribute, value: i8) {
        if let Some(a) = self.base_attributes.get_mut(attribute) {
            *a = value;
        } else {
            self.base_attributes.insert(attribute, value);
        }
    }
}

pub trait CoreStatistics {
    fn calc_accuracy(&self) -> i8;
    fn calc_hit_points(&self) -> i32;
}

// Calculation functions
impl CoreStatistics for Creature {
    fn calc_accuracy(&self) -> i8 {
        // note implicit floor due to integer storage
        return self.level + self.get_base_attribute(attributes::PER) / 2;
    }

    fn calc_hit_points(&self) -> i32 {
        let hp_from_level = match self.level {
            1 => 11,
            2 => 12,
            3 => 13,
            4 => 15,
            5 => 17,
            6 => 19,
            7 => 22,
            8 => 25,
            9 => 28,
            10 => 31,
            11 => 35,
            12 => 39,
            13 => 44,
            14 => 50,
            15 => 56,
            16 => 63,
            17 => 70,
            18 => 78,
            19 => 88,
            20 => 100,
            21 => 115,
            _ => panic!("Invalid level {}", self.level),
        };

        return hp_from_level + self.get_base_attribute(attributes::CON) as i32;
    }
}

impl DefenseCalcs for Creature {
    fn calc_defense(&self, defense: &'static Defense) -> i8 {
        return self.level + self.get_base_attribute(defense.associated_attribute());
    }
}

pub struct CreatureAttribute {
    attribute: &'static Attribute,
    base: i8,
    total: i8,
}

fn format_creature_attributes(creature: &Creature) -> Vec<String> {
    return Attribute::all()
        .iter()
        .map(|attribute| {
            let base = creature.get_base_attribute(attribute);
            if base > 0 {
                return format!(
                    "{} {} ({})",
                    attribute.shorthand_name(),
                    creature.calc_total_attribute(attribute),
                    base
                );
            } else {
                return format!("{} {}", attribute.shorthand_name(), base);
            }
        })
        .collect::<Vec<String>>();
}
