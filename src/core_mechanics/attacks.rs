use crate::core_mechanics::{damage_dice, defenses, HasCreatureMechanics};
use crate::equipment::{HasEquipment, weapons};
use crate::latex_formatting;

pub struct StrikeAttackDefinition {
    accuracy_modifier: i8,
    damage_dice_increments: i8,
    damage_modifier: i8,
    defense: &'static defenses::Defense,
    is_magical: bool,
    name: String,
    weapon: weapons::Weapon,
}

pub struct StandaloneAttackDefinition {
    accuracy_modifier: i8,
    damage_dice: damage_dice::DamageDice,
    damage_modifier: i8,
    defense: &'static defenses::Defense,
    is_magical: bool,
    name: String,
}

pub enum Attack {
    StrikeAttack(StrikeAttackDefinition),
    StandaloneAttack(StandaloneAttackDefinition),
}

pub fn calc_strikes<T: HasAttacks + HasEquipment>(creature: &T) -> Vec<Attack> {
    // TODO: combine maneuvers with weapons and handle non-weapon attacks
    return creature
        .weapons()
        .iter()
        .map(|w| Attack::StrikeAttack(StrikeAttackDefinition {
            accuracy_modifier: 0,
            damage_dice_increments: 0,
            damage_modifier: 0,
            defense: defenses::ARMOR,
            is_magical: false,
            name: w.name().to_string(),
            weapon: *w,
        }))
        .collect();
}

pub trait HasAttacks {
    fn calc_accuracy(&self) -> i8;
    fn calc_damage_increments(&self, is_strike: bool) -> i8;
    fn calc_damage_per_round_multiplier(&self) -> f64;
    fn calc_power(&self, is_magical: bool) -> i8;
}

impl Attack {
    pub fn latex_shorthand<T: HasCreatureMechanics>(&self, creature: &T) -> String {
        return format!(
            "{name} {accuracy} ({damage_dice}{damage_modifier})",
            name = latex_formatting::uppercase_first_letter(self.name()),
            accuracy = latex_formatting::modifier(self.calc_accuracy(creature)),
            damage_dice = self.calc_damage_dice(creature).to_string(),
            damage_modifier = latex_formatting::modifier(self.calc_damage_modifier(creature))
        );
    }

    pub fn name(&self) -> &str {
        match self {
            Attack::StrikeAttack(d) => d.name.as_str(),
            Attack::StandaloneAttack(d) => d.name.as_str(),
        }
    }

    pub fn defense(&self) -> &'static defenses::Defense {
        match self {
            Attack::StrikeAttack(d) => d.defense,
            Attack::StandaloneAttack(d) => d.defense,
        }
    }
}

// Calculation functions
impl Attack {
    pub fn calc_accuracy<T: HasCreatureMechanics>(&self, creature: &T) -> i8 {
        match self {
            Attack::StrikeAttack(d) => d.accuracy_modifier + d.weapon.accuracy() + creature.calc_accuracy(),
            Attack::StandaloneAttack(d) => d.accuracy_modifier + creature.calc_accuracy(),
        }
    }

    pub fn calc_damage_dice<T: HasCreatureMechanics>(&self, creature: &T) -> damage_dice::DamageDice {
        match self {
            Attack::StrikeAttack(d) => d.weapon.damage_dice().add(d.damage_dice_increments + creature.calc_damage_increments(true)),
            Attack::StandaloneAttack(d) => d.damage_dice.add(creature.calc_damage_increments(false)),
        }
    }

    pub fn calc_damage_modifier<T: HasCreatureMechanics>(&self, creature: &T) -> i8 {
        match self {
            Attack::StrikeAttack(d) => d.damage_modifier + creature.calc_power(d.is_magical),
            Attack::StandaloneAttack(d) => d.damage_modifier + creature.calc_power(d.is_magical),
        }
    }
}

// LaTeX generation functions
impl Attack{
    pub fn latex_ability_block<T: HasCreatureMechanics>(&self, creature: &T) -> String {
        let ability_components: Vec<Option<String>> =
            vec![Some(self.latex_type_prefix()), Some(self.latex_effect(creature))];
        let ability_components = ability_components
            .iter()
            .filter(|c| c.is_some())
            .map(|c| c.as_deref().unwrap())
            .collect::<Vec<&str>>();
        return format!(
            "
                \\begin<{ability_environment}><{name}>
                    {ability_components}
                \\end<{ability_environment}>
            ",
            ability_environment = "freeability", // TODO
            ability_components = ability_components.join("\n\\rankline\n\n\\noindent "),
            name = latex_formatting::uppercase_first_letter(self.name()),
        );
    }

    fn latex_type_prefix(&self) -> String {
        // TODO: take into account tags and usage time
        String::from("Instant")
    }

    fn latex_effect<T: HasCreatureMechanics>(&self, creature: &T) -> String {
        return format!(
            "
                The $name makes a strike with a {accuracy} accuracy bonus.
                \\hit {damage} damage.
            ",
            accuracy = latex_formatting::modifier(self.calc_accuracy(creature)),
            damage = format!(
                "{}{}",
                self.calc_damage_dice(creature).to_string(),
                latex_formatting::modifier(self.calc_damage_modifier(creature))
            ),
        );
    }
}
