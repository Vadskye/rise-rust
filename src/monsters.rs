mod animals;
pub mod challenge_rating;
pub mod creature_type;
pub mod monster_entry;
pub mod monster_group;

use crate::core_mechanics::attacks::HasAttacks;
use crate::core_mechanics::attributes::{self, Attribute, HasAttributes};
use crate::core_mechanics::damage_absorption::HasDamageAbsorption;
use crate::core_mechanics::defenses::{self, HasDefenses};
use crate::core_mechanics::resources::{self, HasResources};
use crate::core_mechanics::{attacks, creature, movement_modes, HasCreatureMechanics};
use crate::equipment::{weapons, HasEquipment};
use crate::latex_formatting;
use std::collections::HashMap;

pub struct Monster {
    challenge_rating: &'static challenge_rating::ChallengeRating,
    creature: creature::Creature,
    creature_type: &'static creature_type::CreatureType,
    description: Option<String>,
    knowledge: Option<HashMap<i8, String>>,
    movement_modes: Vec<movement_modes::MovementMode>,
}

pub struct FullMonsterDefinition {
    attributes: Vec<i8>,
    challenge_rating: &'static challenge_rating::ChallengeRating,
    creature_type: &'static creature_type::CreatureType,
    description: Option<&'static str>,
    knowledge: Vec<(i8, &'static str)>,
    level: i8,
    movement_modes: Option<Vec<movement_modes::MovementMode>>,
    name: &'static str,
    weapons: Vec<weapons::Weapon>,
}

impl Monster {
    pub fn new(
        challenge_rating: &'static challenge_rating::ChallengeRating,
        creature_type: &'static creature_type::CreatureType,
        level: i8,
    ) -> Monster {
        return Monster {
            challenge_rating,
            creature_type,
            creature: creature::Creature::new(level),
            description: None,
            knowledge: None,
            movement_modes: vec![],
        };
    }

    pub fn fully_defined(def: FullMonsterDefinition) -> Monster {
        let mut creature = creature::Creature::new(def.level);
        creature.set_name(def.name.to_string());
        for (i, attribute) in attributes::all_attributes().iter().enumerate() {
            creature.set_base_attribute(attribute, def.attributes[i]);
        }
        for weapon in def.weapons {
            creature.add_weapon(weapon);
        }
        let base_knowledge_difficulty = def.level + 5;
        let mut knowledge_option = None;
        if def.knowledge.len() > 0 {
            let mut knowledge = HashMap::new();
            for (modifier, text) in def.knowledge {
                knowledge.insert(base_knowledge_difficulty + modifier, text.to_string());
            }
            knowledge_option = Some(knowledge)
        }
        return Monster {
            challenge_rating: def.challenge_rating,
            creature_type: def.creature_type,
            creature,
            description: if let Some(d) = def.description {
                Some(d.to_string())
            } else {
                None
            },
            knowledge: knowledge_option,
            movement_modes: if let Some(m) = def.movement_modes {
                m
            } else {
                vec![movement_modes::MovementMode::Land(
                    &movement_modes::SpeedCategory::Normal,
                )]
            },
        };
    }

    pub fn standard_monster(
        challenge_rating: &'static challenge_rating::ChallengeRating,
        level: i8,
        starting_attribute: Option<i8>,
        creature_type: Option<&'static creature_type::CreatureType>,
    ) -> Monster {
        let mut creature = creature::Creature::new(level);
        creature.add_weapon(weapons::Weapon::Slam);
        if let Some(a) = starting_attribute {
            creature.set_base_attribute(attributes::STR, a);
            creature.set_base_attribute(attributes::DEX, a);
            creature.set_base_attribute(attributes::CON, a);
            creature.set_base_attribute(attributes::INT, a);
            creature.set_base_attribute(attributes::PER, a);
            creature.set_base_attribute(attributes::WIL, a);
        }
        let creature_type = if let Some(a) = creature_type {
            a
        } else {
            creature_type::PLANEFORGED
        };
        return Monster {
            challenge_rating,
            creature,
            creature_type,
            description: None,
            knowledge: None,
            movement_modes: vec![movement_modes::MovementMode::Land(
                &movement_modes::SpeedCategory::Normal,
            )],
        };
    }

    pub fn set_level(&mut self, level: i8) {
        self.creature.level = level;
    }
}

impl HasAttributes for Monster {
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

impl HasAttacks for Monster {
    fn calc_accuracy(&self) -> i8 {
        return self.creature.calc_accuracy()
            + self.challenge_rating.accuracy_bonus()
            + (self.creature.level + 1) / 6;
    }

    fn calc_damage_per_round_multiplier(&self) -> f64 {
        return self.creature.calc_damage_per_round_multiplier()
            * self.challenge_rating.damage_per_round_multiplier();
    }

    fn calc_damage_increments(&self, is_strike: bool) -> i8 {
        let level_modifier = if is_strike {
            (self.creature.level - 1) / 3
        } else {
            0
        };
        // A rank 3 spell can get a +1d damage bonus just from rank upgrades.
        // This is a little overly specific, but it represents the idea that monsters are using
        // more powerful spells and maneuvers at higher levels. The numbers are a little spiky for
        // strikes, but this has to be at the same level as the strike damage upgrades - the whole
        // point is that the strike upgrades are trying to keep pace with the automatic spell rank
        // upgrades.
        let special_attack_modifier = (self.creature.level - 1) / 6;
        return self.creature.calc_damage_increments(is_strike)
            + self.challenge_rating.damage_increments()
            + level_modifier
            + special_attack_modifier;
    }

    fn calc_power(&self, is_magical: bool) -> i8 {
        let level_scaling = match self.creature.level / 3 {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => 4,
            5 => 6,
            6 => 8,
            7 => 12,
            8 => 16,
            _ => panic!("Invalid level '{}'", self.creature.level),
        };
        return self.creature.calc_power(is_magical) + level_scaling;
    }
}

impl HasEquipment for Monster {
    fn add_weapon(&mut self, weapon: weapons::Weapon) {
        self.creature.add_weapon(weapon);
    }

    fn weapons(&self) -> &Vec<weapons::Weapon> {
        return &self.creature.weapons();
    }
}

impl HasDamageAbsorption for Monster {
    fn calc_damage_resistance(&self) -> i32 {
        return ((self.creature.calc_damage_resistance() as f64)
            * 2.0
            * self.challenge_rating.dr_multiplier()) as i32;
    }
    fn calc_hit_points(&self) -> i32 {
        return ((self.creature.calc_hit_points() as f64)
            * 1.5
            * self.challenge_rating.hp_multiplier()) as i32;
    }
}

impl HasDefenses for Monster {
    fn calc_defense(&self, defense: &'static defenses::Defense) -> i8 {
        return self.creature.calc_defense(defense)
            + self.creature_type.defense_bonus(defense)
            + self.challenge_rating.defense_bonus()
            + (self.creature.level + 3) / 6;
    }
}

impl HasResources for Monster {
    fn calc_resource(&self, resource: &'static resources::Resource) -> i8 {
        return self.creature.calc_resource(resource);
    }
}

// No need for explicit funtions here - it's handled by the above functions
impl HasCreatureMechanics for Monster {}

// LaTeX conversion
impl Monster {
    pub fn to_section(&self, section_name: Option<&str>) -> String {
        let section_name = section_name.unwrap_or("monsubsection");
        let name = if let Some(ref n) = self.creature.name {
            n
        } else {
            panic!("Monster has no name")
        };
        return latex_formatting::latexify(format!(
            // TODO: some of these vspace values look unnecessary / conflicting
            "
                \\begin<{section_name}><{name}><{level}>[{cr}]
                    \\vspace<-1em>\\spelltwocol<><{size} {type}>\\vspace<-1em>
                    \\vspace<0em>

                    {description}
                    {knowledge}

                    {content}
                    {footer}
                \\end<{section_name}>
                {abilities}
            ",
            section_name = section_name,
            name = name,
            level = self.creature.level,
            cr = self.challenge_rating.to_string(),
            size = "Medium", // TODO
            type = self.creature_type.name(),
            description = self.description.as_deref().unwrap_or(""),
            knowledge = self.latex_knowledge().trim(),
            content = self.latex_content().trim(),
            footer = self.latex_footer().trim(),
            abilities = "", // TODO
        ));
    }

    fn latex_content(&self) -> String {
        return format!(
            "
                \\begin<spellcontent>
                  \\begin<spelltargetinginfo>
                    \\pari \\textbf<HP> {hp}
                        \\monsep \\textbf<DR> {dr}
                        \\monsep \\textbf<AD> {armor}
                        \\monsep \\textbf<Fort> {fort}
                        \\monsep \\textbf<Ref> {ref}
                        \\monsep \\textbf<Ment> {ment}
                    \\pari \\textbf<{strike_maybe_plural}> {strikes}
                  \\end<spelltargetinginfo>
                \\end<spellcontent>
            ",
            hp = self.calc_hit_points(),
            dr = self.calc_damage_resistance(),
            armor = self.calc_defense(defenses::ARMOR),
            fort = self.calc_defense(defenses::FORT),
            ref = self.calc_defense(defenses::REF),
            ment = self.calc_defense(defenses::MENT),
            strike_maybe_plural = if self.creature.weapons.len() > 1 { "Strikes" } else { "Strike" },
            strikes = attacks::calc_attacks(self)
                .iter()
                .map(|a| a.to_latex())
                .collect::<Vec<String>>()
                .join(";\\par "),
        );
    }

    fn latex_footer(&self) -> String {
        return format!(
            "
                \\begin<monsterfooter>
                  \\pari \\textbf<Speeds> {speeds} \\monsep
                    \\textbf<Space> {space} \\monsep
                    \\textbf<Reach> {reach}
                  \\pari \\textbf<Awareness> {awareness}
                  \\pari \\textbf<Attributes> {attributes}
                  % This is sometimes useful for debugging, but isn't actually useful information in general.
                  % To the extent that raw accuracy or power is important, that should already be
                  % included in more specific attacks or abilities.
                  % \\pari \\textbf<Accuracy> {accuracy} \\monsep {power}
                  \\pari \\textbf<Alignment> {alignment}
                \\end<monsterfooter>
            ",
            speeds = self.movement_modes.iter().map(
                |m| format!("{} {} ft.", m.name(), m.calc_speed(&self.creature.size))
            ).collect::<Vec<String>>().join("; "),
            space = "5 ft.",        // TODO
            reach = "5 ft.",        // TODO
            // TODO: figure out skill training
            awareness =
                latex_formatting::modifier(self.creature.get_base_attribute(attributes::PER)),
            attributes = self.latex_attributes(),
            accuracy = latex_formatting::modifier(self.calc_accuracy()),
            power = self.latex_power(),
            alignment = latex_formatting::uppercase_first_letter("always true neutral"), // TODO
        );
    }

    fn latex_power(&self) -> String {
        let mundane_power = self.calc_power(false);
        let magical_power = self.calc_power(true);
        if mundane_power == magical_power {
            return format!("\\textbf<Power> {}", mundane_power);
        } else {
            return format!(
                "\\textbf<Mundane Power> {mundane} \\monsep \\textbf<Magical Power> {magical}",
                mundane = mundane_power,
                magical = magical_power
            );
        }
    }

    fn latex_attributes(&self) -> String {
        return attributes::Attribute::all()
            .iter()
            .map(|a| format!("{} {}", a.shorthand_name(), self.calc_total_attribute(a)))
            .collect::<Vec<String>>()
            .join(", ");
    }

    fn latex_knowledge(&self) -> String {
        if let Some(ref knowledge) = self.knowledge {
            return knowledge
                .iter()
                .map(|(difficulty, text)| {
                    return format!(
                        "\\parhead<Knowledge ({subskill}) {difficulty}> {text}",
                        subskill = self.creature_type.knowledge(), // TODO
                        difficulty = difficulty,
                        text = text,
                    );
                })
                .collect::<Vec<String>>()
                .join("\n");
        } else {
            return "".to_string();
        }
    }
}
