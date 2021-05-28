use crate::equipment::weapons;
use crate::monsters::challenge_rating::{CR1, CR3, CR4};
use crate::monsters::creature_type::ANIMAL;
use crate::monsters::monster_entry::MonsterEntry;
use crate::monsters::{monster_group, FullMonsterDefinition, Monster};

pub fn animals() -> Vec<MonsterEntry> {
    let mut monsters: Vec<MonsterEntry> = vec![];

    monsters.push(MonsterEntry::Monster(Monster::fully_defined(
        FullMonsterDefinition {
            attributes: vec![2, 2, 1, -8, 1, -1],
            challenge_rating: CR1,
            creature_type: ANIMAL,
            level: 1,
            name: "Baboon",
            weapons: vec![weapons::Weapon::Bite],
        },
    )));

    monsters.push(MonsterEntry::MonsterGroup(
        monster_group::MonsterGroup::new(
            "Bears",
            vec![
                Monster::fully_defined(FullMonsterDefinition {
                    attributes: vec![3, 0, 3, -8, 0, -1],
                    challenge_rating: CR3,
                    creature_type: ANIMAL,
                    level: 1,
                    name: "Black bear",
                    weapons: vec![weapons::Weapon::Bite, weapons::Weapon::Claw],
                }),
                Monster::fully_defined(FullMonsterDefinition {
                    attributes: vec![3, 0, 3, -8, 0, -1],
                    challenge_rating: CR4,
                    creature_type: ANIMAL,
                    level: 20,
                    name: "Brown bear",
                    weapons: vec![weapons::Weapon::Bite, weapons::Weapon::Claw],
                }),
            ],
        ),
    ));

    return monsters;
}
