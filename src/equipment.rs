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

pub enum ArmorUsageClass {
    Light,
    Medium,
    Heavy,
}
