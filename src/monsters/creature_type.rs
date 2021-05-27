use crate::core_mechanics::defenses::Defense;
use std::fmt;

pub enum CreatureType {
    Aberration,
    Animal,
    Undead,
}

impl CreatureType {
    pub fn defense_bonus(&self, defense: &'static Defense) -> i8 {
        match self {
            Self::Aberration => match defense {
                Defense::Armor => 1,
                Defense::Fortitude => 5,
                Defense::Reflex => 4,
                Defense::Mental => 6,
            },
            Self::Animal => match defense {
                Defense::Armor => 1,
                Defense::Fortitude => 6,
                Defense::Reflex => 5,
                Defense::Mental => 4,
            },
            Self::Undead => match defense {
                Defense::Armor => 1,
                Defense::Fortitude => 4,
                Defense::Reflex => 5,
                Defense::Mental => 6,
            },
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Aberration => "aberration",
            Self::Animal => "animal",
            Self::Undead => "undead",
        }
    }
}

impl fmt::Display for CreatureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
