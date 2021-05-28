use crate::monsters::{animals, creature_type, monster_group, Monster};
use std::collections::HashMap;

pub fn generate_monster_entries() -> HashMap<&'static creature_type::CreatureType, Vec<MonsterEntry>> {
    let mut entries_by_type = HashMap::new();
    entries_by_type.insert(creature_type::ANIMAL, animals::animals());
    return entries_by_type;
}

pub enum MonsterEntry {
    Monster(Monster),
    MonsterGroup(monster_group::MonsterGroup),
}

impl MonsterEntry {
    pub fn to_latex(&self) -> String {
        if let MonsterEntry::Monster(m) = self {
            return m.to_section(None);
        } else if let MonsterEntry::MonsterGroup(m) = self {
            return m.to_latex();
        } else {
            panic!("Nonsensical monter entry");
        }
    }
}