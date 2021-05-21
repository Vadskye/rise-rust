use crate::classes::Class;
use crate::core_mechanics::attributes::{Attribute, AttributeCalcs};
use crate::core_mechanics::{creature, defenses};

pub struct Character {
    class: Class,
    creature: creature::Creature,
}

impl Character {
    pub fn new(class: Class, level: i8) -> Character {
        return Character {
            class,
            creature: creature::Creature::new(level),
        };
    }

    pub fn set_level(&mut self, level: i8) {
        self.creature.level = level;
    }

    pub fn to_latex(&self) -> String {
        return self.creature.to_latex();
    }
}

impl AttributeCalcs for Character {
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

impl creature::CoreStatistics for Character {
    fn calc_accuracy(&self) -> i8 {
        return self.creature.calc_accuracy();
    }
    fn calc_hit_points(&self) -> i32 {
        return self.creature.calc_hit_points();
    }
}

impl defenses::DefenseCalcs for Character {
    fn calc_armor(&self) -> i8 {
        return self.creature.calc_armor() + self.class.defenses().armor;
    }
    fn calc_fortitude(&self) -> i8 {
        return self.creature.calc_fortitude() + self.class.defenses().fortitude;
    }
    fn calc_reflex(&self) -> i8 {
        return self.creature.calc_reflex() + self.class.defenses().reflex;
    }
    fn calc_mental(&self) -> i8 {
        return self.creature.calc_mental() + self.class.defenses().mental;
    }
}
