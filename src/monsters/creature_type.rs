use crate::core_mechanics::defenses::{self, Defense};
use crate::latex_formatting;
use titlecase::titlecase;
use std::fmt;

pub enum CreatureType {
    Aberration,
    Animal,
    Planeforged,
    Undead,
}

pub static ABERRATION: &CreatureType = &CreatureType::Aberration;
pub static ANIMAL: &CreatureType = &CreatureType::Animal;
pub static PLANEFORGED: &CreatureType = &CreatureType::Planeforged;
pub static UNDEAD: &CreatureType = &CreatureType::Undead;

impl CreatureType {
    pub fn defense_bonus(&self, defense: &'static Defense) -> i8 {
        match self {
            Self::Aberration => match defense {
                Defense::Armor => 4,
                Defense::Fortitude => 5,
                Defense::Reflex => 4,
                Defense::Mental => 6,
            },
            Self::Animal => match defense {
                Defense::Armor => 4,
                Defense::Fortitude => 6,
                Defense::Reflex => 5,
                Defense::Mental => 4,
            },
            Self::Planeforged => match defense {
                Defense::Armor => 4,
                Defense::Fortitude => 5,
                Defense::Reflex => 5,
                Defense::Mental => 5,
            },
            Self::Undead => match defense {
                Defense::Armor => 4,
                Defense::Fortitude => 4,
                Defense::Reflex => 5,
                Defense::Mental => 6,
            },
        }
    }

    pub fn from_string(text: String) -> &'static Self {
        match text.as_str() {
            "aberration" => ABERRATION,
            "animal" => ANIMAL,
            "planeforged" => PLANEFORGED,
            "undead" => UNDEAD,
            _ => panic!("Invalid creature type '{}'", text),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Aberration => "aberration",
            Self::Animal => "animal",
            Self::Planeforged => "planeforged",
            Self::Undead => "undead",
        }
    }

    pub fn plural_name(&self) -> String {
        match self {
            Self::Undead => self.name().to_string(),
            _ => format!("{}s", self.name()),
        }
    }

    pub fn latex_section_header(&self) -> String {
        return format!(
            "
                \\newpage
                \\section<{plural_name_title}>

                All {plural_name} have the following properties unless noted otherwise in their description:
                \\begin<itemize>
                    \\item {defenses}
                \\end<itemize>
            ",
            plural_name_title = titlecase(self.plural_name().as_str()),
            plural_name = self.plural_name(),
            defenses = self.latex_defenses(),
        );
    }

    fn latex_defenses(&self) -> String {
        return format!(
            "\\textbf<Defenses:> {armor} Armor, {fort} Fortitude, {ref} Reflex, {ment} Mental",
            armor=latex_formatting::modifier(self.defense_bonus(defenses::ARMOR)),
            fort=latex_formatting::modifier(self.defense_bonus(defenses::FORT)),
            ref=latex_formatting::modifier(self.defense_bonus(defenses::REF)),
            ment=latex_formatting::modifier(self.defense_bonus(defenses::MENT)),
        );
    }
}

impl fmt::Display for CreatureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
