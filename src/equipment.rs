use std::fmt;

pub enum WeaponGroup {
    Armor,
    Axes,
    Blades,
    Bows,
    Clublike,
}

impl WeaponGroup {
    fn weapons(&self) -> Vec<Weapon> {
        match self {
            Self::Armor => vec![Weapon::ArmorSpikes],
            Self::Axes => vec![Weapon::Battleaxe, Weapon::Greataxe],
            Self::Blades => vec![Weapon::Broadsword, Weapon::Greataxe],
            Self::Bows => vec![Weapon::Longbow],
            Self::Clublike => vec![Weapon::Sap],
        }
    }
}

pub enum Weapon {
    ArmorSpikes,
    Battleaxe,
    Broadsword,
    Greataxe,
    Greatsword,
    Longbow,
    Sap,
    Scimitar,
    Sickle,
}

impl Weapon {
    pub fn name(&self) -> &str {
        match self {
            Self::ArmorSpikes => "armor spike",
            Self::Battleaxe => "battleaxe",
            Self::Broadsword => "broadsword",
            Self::Greataxe => "greataxe",
            Self::Greatsword => "greatsword",
            Self::Longbow => "longbow",
            Self::Sap => "sap",
            Self::Scimitar => "scimitar",
            Self::Sickle => "sickle",
        }
    }

    pub fn plural_name(&self) -> String {
        return format!("{}s", self.name())
    }
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub enum ArmorUsageClass {
    Light,
    Medium,
    Heavy,
}

impl ArmorUsageClass {
    pub fn name(&self) -> &str {
        match self {
            Self::Light => "light",
            Self::Medium => "medium",
            Self::Heavy => "heavy",
        }
    }
}

impl fmt::Display for ArmorUsageClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
