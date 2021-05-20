use crate::core_mechanics::attributes::Attribute;
use std::collections::HashMap;

pub struct Creature {
    attributes: HashMap<&'static Attribute, CreatureAttribute>,
    pub level: u8,
}

impl Creature {
    pub fn new(level: u8) -> Creature {
        let attributes = HashMap::<&Attribute, CreatureAttribute>::new();
        return Creature { attributes, level };
    }

    pub fn set_base_attribute(&mut self, attribute: &'static Attribute, value: i8) {
        let creature_attribute = CreatureAttribute {
            attribute,
            base: value,
            total: Attribute::calculate_total(value, self.level),
        };
        if let Some(a) = self.attributes.get_mut(attribute) {
            *a = creature_attribute;
        } else {
            self.attributes.insert(attribute, creature_attribute);
        }
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level;
        for attribute in Attribute::all() {
            let current_val = if let Some(ca) = self.attributes.get(attribute) {
                Some(ca.base)
            } else {
                None
            };
            if let Some(base) = current_val {
                self.set_base_attribute(attribute, base);
            }
        }
    }

    pub fn explain_combat_stats(&self) -> String {
        return format!(
            "Attr: {attributes}",
            attributes = format_creature_attributes(self).join(", "),
        );
    }
}

pub struct CreatureAttribute {
    attribute: &'static Attribute,
    base: i8,
    total: i8,
}

fn format_creature_attributes(creature: &Creature) -> Vec<String> {
    return Attribute::all()
        .iter()
        .map(|attribute| {
            if let Some(ca) = creature.attributes.get(attribute) {
                if ca.base > 0 {
                    return format!("{} {} ({})", attribute.shorthand_name(), ca.total, ca.base);
                } else {
                    return format!("{} {}", attribute.shorthand_name(), ca.total);
                }
            } else {
                return format!("{} {}", attribute.shorthand_name(), 0);
            }
        })
        .collect::<Vec<String>>();
}
