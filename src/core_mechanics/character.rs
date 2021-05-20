use crate::classes::Class;
use crate::core_mechanics::creature;

struct Character {
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
}

impl creature::CreatureCalculation for Character {
    fn calc_accuracy(&self) -> i8 {
        return self.creature.calc_accuracy();
    }
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
    fn calc_hit_points(&self) -> i32 {
        return self.creature.calc_hit_points();
    }
}
