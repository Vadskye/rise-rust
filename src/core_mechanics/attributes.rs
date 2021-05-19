pub enum Attribute {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Perception,
    Willpower,
}

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
}

pub fn all_attributes() -> Vec<Attribute> {
    vec![
        Attribute::Strength,
        Attribute::Dexterity,
        Attribute::Constitution,
        Attribute::Intelligence,
        Attribute::Perception,
        Attribute::Willpower,
    ]
}
