use crate::core_mechanics::attributes::{self, AttributeCalcs};
use crate::core_mechanics::creature::CoreStatistics;
use crate::core_mechanics::defenses::{self, DefenseCalcs};

pub fn format_creature<T: AttributeCalcs + CoreStatistics + DefenseCalcs>(creature: &T) -> String {
    format!(
        "
            HP {hit_points}, AD {armor}, Fort {fortitude}, Ref {reflex}, Ment {mental}
            Attr: {attributes}
        ",
        attributes = format_creature_attributes(creature).join(", "),
        armor = creature.calc_defense(defenses::ARMOR),
        fortitude = creature.calc_defense(defenses::FORT),
        hit_points = creature.calc_hit_points(),
        mental = creature.calc_defense(defenses::MENT),
        reflex = creature.calc_defense(defenses::REF),
    )
}

fn format_creature_attributes(creature: &impl AttributeCalcs) -> Vec<String> {
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
