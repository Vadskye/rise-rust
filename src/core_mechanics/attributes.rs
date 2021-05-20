use std::cmp::PartialEq;

#[derive(Clone, Eq, Hash)]
pub enum Attribute {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Perception,
    Willpower,
}

pub static STR: Attribute = Attribute::Strength;
pub static DEX: Attribute = Attribute::Dexterity;
pub static CON: Attribute = Attribute::Constitution;
pub static INT: Attribute = Attribute::Intelligence;
pub static PER: Attribute = Attribute::Perception;
pub static WIL: Attribute = Attribute::Willpower;

impl Attribute {
    pub fn name(&self) -> &str {
        match self {
            Self::Strength => "strength",
            Self::Dexterity => "dexterity",
            Self::Constitution => "constitution",
            Self::Intelligence => "intelligence",
            Self::Perception => "perception",
            Self::Willpower => "willpower",
        }
    }

    pub fn shorthand_name(&self) -> &str {
        match self {
            Self::Strength => "str",
            Self::Dexterity => "dex",
            Self::Constitution => "con",
            Self::Intelligence => "int",
            Self::Perception => "per",
            Self::Willpower => "wil",
        }
    }

    pub fn calculate_total(base_value: i8, level: i8) -> i8 {
        let slevel = level as i8;
        if base_value <= 0 {
            return base_value;
        } else if base_value == 1 {
            return base_value + slevel / 4;
        } else if base_value == 2 {
            return base_value + slevel / 2;
        } else {
            return base_value + (((slevel - 1) * (base_value - 1)) as f64 / 2.0).ceil() as i8;
        }
    }

    pub fn all() -> Vec<&'static Attribute> {
        vec![
            &STR,
            &DEX,
            &CON,
            &INT,
            &PER,
            &WIL,
        ]
    }
}

impl PartialEq for Attribute {
    fn eq(&self, other:&Self) -> bool {
        return self.name() == other.name();
    }
}
