use crate::latex_formatting;
use numerics::Numerics;
use std::fmt;
use titlecase::titlecase;

pub enum Class {
    Barbarian,
    Cleric,
    Rogue,
}

impl Class {
    fn attunement_points(&self) -> u8 {
        match self {
            Class::Barbarian => 1,
            Class::Cleric => 2,
            Class::Rogue => 1,
        }
    }

    fn defenses(&self) -> ClassDefenseBonuses {
        match self {
            Class::Barbarian => ClassDefenseBonuses {
                armor: 1,
                fortitude: 7,
                reflex: 5,
                mental: 3,
            },
            Class::Cleric => ClassDefenseBonuses {
                armor: 1,
                fortitude: 5,
                reflex: 3,
                mental: 7,
            },
            Class::Rogue => ClassDefenseBonuses {
                armor: 1,
                fortitude: 3,
                reflex: 7,
                mental: 5,
            },
        }
    }

    fn fatigue_tolerance(&self) -> u8 {
        match self {
            Class::Barbarian => 4,
            Class::Cleric => 1,
            Class::Rogue => 2,
        }
    }

    fn insight_points(&self) -> u8 {
        match self {
            Class::Barbarian => 1,
            Class::Cleric => 3,
            Class::Rogue => 3,
        }
    }

    fn name(&self) -> &str {
        match self {
            Class::Barbarian => "barbarian",
            Class::Cleric => "cleric",
            Class::Rogue => "rogue",
        }
    }

    fn shorthand_name(&self) -> &str {
        match self {
            Class::Barbarian => "Bbn",
            Class::Cleric => "Clr",
            Class::Rogue => "Rog",
        }
    }

    fn skill_points(&self) -> u8 {
        match self {
            Class::Barbarian => 9,
            Class::Cleric => 6,
            Class::Rogue => 12,
        }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Class({})", self.name())
    }
}

#[derive(Debug)]
pub struct ClassDefenseBonuses {
    pub armor: u8,
    pub fortitude: u8,
    pub mental: u8,
    pub reflex: u8,
}

pub fn generate_latex_basic_class_abilities(class: &Class) -> String {
    return latex_formatting::latexify(format!(
        "
            \\subsection<Basic Class Abilities>
            If you are a {name}, you gain the following abilities.

            \\cf<{shorthand_name}><Defenses>
            You gain the following bonuses to your \\glossterm<defenses>: {defenses}.

            \\cf<{shorthand_name}><Resources>
            You have the following \\glossterm<resources>:
            \\begin<itemize>
                \\item {insight_points}, which you can spend to gain additional abilities or proficiencies (see \\pcref<Insight Points>).
                \\item {skill_points}, which you can spend to learn skills (see \\pcref<Skills>).
                \\item {attunement_points}, which you can use to attune to items and abilities that affect you (see \\pcref<Attunement Points>).
                \\item A \\plus{fatigue_tolerance} bonus to your \\glossterm<fatigue tolerance>, which makes it easier for you to use powerful abilities that fatigue you (see \\pcref<Fatigue>).
            \\end<itemize>
        ",
        attunement_points = generate_labeled_english_number(class.attunement_points(), "\\glossterm<attunement point>", "\\glossterm<attunement points>"),
        defenses = generate_latex_defense_bonuses(&class.defenses()).trim(),
        fatigue_tolerance = class.fatigue_tolerance(),
        insight_points = generate_labeled_english_number(class.insight_points(), "\\glossterm<insight point>", "\\glossterm<insight points>"),
        name = class.name(),
        shorthand_name = class.shorthand_name(),
        skill_points = generate_labeled_english_number(class.skill_points(), "\\glossterm<skill point>", "\\glossterm<skill points>"),
    ));
}

pub fn generate_latex_defense_bonuses(defenses: &ClassDefenseBonuses) -> String {
    return latex_formatting::latexify(format!("
        \\plus{armor} Armor, \\plus{fortitude} Fortitude, \\plus{reflex} Reflex, \\plus{mental} Mental
    ", armor=defenses.armor, fortitude=defenses.fortitude, reflex=defenses.reflex, mental=defenses.mental));
}

pub fn generate_labeled_english_number(val: u8, singular: &str, plural: &str) -> String {
    let converter = Numerics::builder().build();
    let english_number = converter.convert_number(val).unwrap();
    let suffix = if val == 1 { singular } else { plural };
    return format!("{} {}", titlecase(&english_number[0]), suffix);
}
