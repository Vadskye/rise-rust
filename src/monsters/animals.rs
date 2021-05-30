use crate::equipment::weapons;
use crate::monsters::challenge_rating::{CR1, CR3, CR4};
use crate::monsters::creature_type::ANIMAL;
use crate::monsters::monster_entry::MonsterEntry;
use crate::core_mechanics::movement_modes::{MovementMode, SpeedCategory};
use crate::monsters::{monster_group, FullMonsterDefinition, Monster};

pub fn animals() -> Vec<MonsterEntry> {
    let mut monsters: Vec<MonsterEntry> = vec![];

    monsters.push(MonsterEntry::Monster(Monster::fully_defined(
        FullMonsterDefinition {
            attributes: vec![2, 2, 1, -8, 1, -1],
            challenge_rating: CR1,
            creature_type: ANIMAL,
            description: None,
            knowledge: vec![
                (0, "
                    A baboon is a primate adapted to life on the ground.
                    A typical baboon is the size of a big dog.
                "),
                (5, "
                    Baboons prefer open spaces but climb trees to find safe places to rest overnight.
                    They can be aggressive, though they avoid attacking creatures that seem too dangerous.
                "),
            ],
            level: 1,
            movement_modes: None,
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
                    description: None,
                    knowledge: vec![
                        (0, "
                            Black bears are forest-dwelling omnivores that are usually not dangerous unless an interloper threatens their cubs or food supply.
                            They can be pure black, blond, or cinnamon in color and are rarely more than 5 feet long.
                        "),
                    ],
                    level: 3,
                    movement_modes: None,
                    name: "Black bear",
                    weapons: vec![weapons::Weapon::Bite, weapons::Weapon::Claw],
                }),
                Monster::fully_defined(FullMonsterDefinition {
                    attributes: vec![4, 0, 3, -8, 0, -1],
                    challenge_rating: CR4,
                    creature_type: ANIMAL,
                    description: Some("A brown bear's statistics can be used for almost any big bear, including a grizzly bear."),
                    knowledge: vec![
                        (0, "
                            Brown bears tend to be bad-tempered and territorial.
                        "),
                    ],
                    movement_modes: None,
                    level: 5,
                    name: "Brown bear",
                    weapons: vec![weapons::Weapon::Bite, weapons::Weapon::Claw],
                }),
            ],
        ),
    ));

    return monsters;
}
