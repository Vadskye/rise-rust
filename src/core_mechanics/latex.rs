use crate::core_mechanics::attacks::{self, HasAttacks};
use crate::core_mechanics::attributes::{self, HasAttributes};
use crate::core_mechanics::damage_absorption::HasDamageAbsorption;
use crate::core_mechanics::defenses::{self, HasDefenses};
use crate::equipment::HasEquipment;

pub fn format_creature<T: HasAttacks + HasAttributes + HasDamageAbsorption + HasDefenses + HasEquipment>(
    creature: &T,
) -> String {
    format!(
        "
            HP {hit_points}, DR {damage_resistance}
            AD {armor}, Fort {fortitude}, Ref {reflex}, Ment {mental}
            {attacks}
            Attr: {attributes}
        ",
        attacks = attacks::calc_attacks(creature)
            .iter()
            .map(|a| a.to_latex())
            .collect::<Vec<String>>()
            .join("; "),
        attributes = format_creature_attributes(creature).join(", "),
        armor = creature.calc_defense(defenses::ARMOR),
        fortitude = creature.calc_defense(defenses::FORT),
        damage_resistance = creature.calc_damage_resistance(),
        hit_points = creature.calc_hit_points(),
        mental = creature.calc_defense(defenses::MENT),
        reflex = creature.calc_defense(defenses::REF),
    )
}

fn format_creature_attributes(creature: &impl HasAttributes) -> Vec<String> {
    return attributes::Attribute::all()
        .iter()
        .map(|attribute| {
            let base = creature.get_base_attribute(attribute);
            if base > 0 {
                return format!(
                    "{} {} ({})",
                    attribute.shorthand_name(),
                    creature.calc_total_attribute(attribute),
                    base
                );
            } else {
                return format!("{} {}", attribute.shorthand_name(), base);
            }
        })
        .collect::<Vec<String>>();
}