use crate::core_mechanics::{damage_dice, defenses};
use crate::equipment::HasEquipment;
use crate::latex_formatting;

pub struct Attack {
    pub accuracy_modifier: i8,
    pub damage_dice: damage_dice::DamageDice,
    pub damage_modifier: i8,
    // TODO: support multiple defenses?
    pub defense: &'static defenses::Defense,
    pub name: String,
}

pub fn calc_attacks<T: HasAttacks + HasEquipment>(creature: &T) -> Vec<Attack> {
    // TODO: combine maneuvers with weapons and handle non-weapon attacks
    return creature
        .weapons()
        .iter()
        .map(|w| Attack {
            accuracy_modifier: w.accuracy() + creature.calc_accuracy(),
            damage_dice: w.damage_dice().add(creature.calc_damage_increments(true)),
            damage_modifier: creature.calc_power(false),
            defense: defenses::ARMOR,
            name: w.name().to_string(),
        })
        .collect();
}

pub trait HasAttacks {
    fn calc_accuracy(&self) -> i8;
    fn calc_damage_increments(&self, is_strike: bool) -> i8;
    fn calc_damage_per_round_multiplier(&self) -> f64;
    fn calc_power(&self, is_magical: bool) -> i8;
}

impl Attack {
    pub fn latex_shorthand(&self) -> String {
        return format!(
            "{name} {accuracy} ({damage_dice}{damage_modifier})",
            name = latex_formatting::uppercase_first_letter(self.name.as_str()),
            accuracy = latex_formatting::modifier(self.accuracy_modifier),
            damage_dice = self.damage_dice.to_string(),
            damage_modifier = latex_formatting::modifier(self.damage_modifier)
        );
    }

    pub fn latex_ability_block(&self) -> String {
        let ability_components: Vec<Option<String>> =
            vec![Some(self.latex_type_prefix()), Some(self.latex_effect())];
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
            name = latex_formatting::uppercase_first_letter(self.name.as_str()),
        );
    }

    fn latex_type_prefix(&self) -> String {
        // TODO: take into account tags and usage time
        String::from("Instant")
    }

    fn latex_effect(&self) -> String {
        return format!(
            "
                The $name makes a strike with a {accuracy} accuracy bonus.
                \\hit {damage} damage.
            ",
            accuracy = latex_formatting::modifier(self.accuracy_modifier),
            damage = format!(
                "{}{}",
                self.damage_dice.to_string(),
                latex_formatting::modifier(self.damage_modifier)
            ),
        );
    }
}
