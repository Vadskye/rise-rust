use crate::classes::Class;
use crate::core_mechanics::attributes::{Attribute, AttributeCalcs};
use crate::core_mechanics::defenses::DefenseCalcs;
use crate::core_mechanics::resources::ResourceCalcs;
use crate::core_mechanics::{creature, defenses, resources};

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

    // Eventually, pulling latex from the creature won't work - a class can't modify a creature's
    // HP. However, it's convenient for now.
    pub fn to_latex(&self) -> String {
        return format!(
            "
                {creature_latex}
                {class_name} {level}
                AP {ap}, FT {ft}, IP {ip}, SP {sp}
            ",
            creature_latex = self.creature.to_latex().trim(),
            class_name = self.class.name(),
            level = self.creature.level,
            ap = self.calc_resource(resources::AP),
            ft = self.calc_resource(resources::FT),
            ip = self.calc_resource(resources::IP),
            sp = self.calc_resource(resources::SP),
        );
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

impl DefenseCalcs for Character {
    fn calc_defense(&self, defense: &'static defenses::Defense) -> i8 {
        return self.creature.calc_defense(defense) + self.class.defense_bonus(defense);
    }
}

impl ResourceCalcs for Character {
    fn calc_resource(&self, resource: &'static resources::Resource) -> i8 {
        return self.creature.calc_resource(resource) + self.class.resource_bonus(resource);
    }
}
