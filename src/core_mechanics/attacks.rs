use crate::core_mechanics::damage_dice;
use crate::equipment::HasEquipment;
use crate::latex_formatting;

pub struct Attack {
    accuracy_modifier: i8,
    damage_dice: damage_dice::DamageDice,
    damage_modifier: i8,
    name: String,
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
            name: w.name().to_string(),
        })
        .collect();
}

pub trait HasAttacks {
    fn calc_accuracy(&self) -> i8;
    fn calc_damage_increments(&self, is_strike: bool) -> i8;
    fn calc_power(&self, is_magical: bool) -> i8;
}

impl Attack {
    pub fn to_latex(&self) -> String {
        return format!(
            "{name} {accuracy} ({damage_dice}{damage_modifier})",
            name = self.name,
            accuracy = latex_formatting::modifier(self.accuracy_modifier),
            damage_dice = self.damage_dice.to_string(),
            damage_modifier = latex_formatting::modifier(self.damage_modifier)
        );
    }
}
