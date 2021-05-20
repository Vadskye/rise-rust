use crate::classes::Class;
use crate::classes::archetypes::ClassArchetype;

pub struct RankAbility<'a> {
    pub description: &'a str,
    pub rank: i8,
    pub name: &'a str,
}

pub fn archetype_rank_abilities(archetype: &ClassArchetype) -> Vec<Vec<RankAbility>> {
    // Splitting this up by class has the downside of breaking Rust's compile-time arm
    // checking. However, these functions can get massive, so it has the advantage of
    // making the rank abilities for each class more readable.
    match archetype.class() {
        Class::Barbarian => barbarian_rank_abilities(archetype),
        Class::Cleric => cleric_rank_abilities(archetype),
        Class::Rogue => rogue_rank_abilities(archetype),
    }
}

fn barbarian_rank_abilities(archetype: &ClassArchetype) -> Vec<Vec<RankAbility>> {
    match archetype {
        ClassArchetype::BattleforgedResilience => {
            return vec![
                vec![
                    RankAbility {
                        name: "unbattered resilience",
                        rank: 0,
                        description: "todo",
                    },
                ],
                vec![
                    RankAbility {
                        name: "battle-scarred",
                        rank: 1,
                        description: "todo",
                    },
                ],
            ];
        },
        ClassArchetype::Battlerager => {
            return vec![
                vec![
                    RankAbility {
                        name: "insensible anger",
                        rank: 0,
                        description: "todo",
                    },
                ],
                vec![
                    RankAbility {
                        name: "rage",
                        rank: 1,
                        description: "todo",
                    },
                ],
            ];
        },
        ClassArchetype::OutlandSavage => {
            return vec![
                vec![
                    RankAbility {
                        name: "savage rush",
                        rank: 0,
                        description: "todo",
                    },
                ],
                vec![
                    RankAbility {
                        name: "fast movement",
                        rank: 1,
                        description: "todo",
                    },
                ],
            ];
        },
        ClassArchetype::PrimalWarrior => {
            return vec![
                vec![
                    RankAbility {
                        name: "primal might",
                        rank: 0,
                        description: "todo",
                    },
                ],
                vec![
                    RankAbility {
                        name: "combat styles",
                        rank: 1,
                        description: "todo",
                    },
                ],
            ];
        },
        ClassArchetype::Totemist => {
            return vec![
                vec![
                    RankAbility {
                        name: "animal instincts",
                        rank: 0,
                        description: "todo",
                    },
                ],
                vec![
                    RankAbility {
                        name: "totem animal",
                        rank: 1,
                        description: "todo",
                    },
                ],
            ];
        },
        _ => panic!("Missing archetype '{}'", archetype.name())
    }
}

fn cleric_rank_abilities(archetype: &ClassArchetype) -> Vec<Vec<RankAbility>> {
    match archetype {
        ClassArchetype::ClericDivineMagic => {
            return vec![
                vec![
                    RankAbility {
                        name: "cantrips",
                        rank: 0,
                        description: "todo",
                    },
                ],
                vec![
                    RankAbility {
                        name: "spellcasting",
                        rank: 1,
                        description: "todo",
                    },
                ],
            ];
        },
        ClassArchetype::DivineSpellMastery => {
            return vec![
                vec![
                    RankAbility {
                        name: "mystic sphere",
                        rank: 0,
                        description: "todo",
                    },
                ],
                vec![
                    RankAbility {
                        name: "mystic insight",
                        rank: 1,
                        description: "todo",
                    },
                ],
            ];
        },
        ClassArchetype::DomainInfluence => {
            return vec![
                vec![
                    RankAbility {
                        name: "domains",
                        rank: 0,
                        description: "todo",
                    },
                    RankAbility {
                        name: "domain gift",
                        rank: 0,
                        description: "todo",
                    },
                ],
                vec![
                    RankAbility {
                        name: "domain gift",
                        rank: 1,
                        description: "todo",
                    },
                ],
            ];
        },
        ClassArchetype::Healer => {
            return vec![
                vec![
                    RankAbility {
                        name: "desperate healing",
                        rank: 0,
                        description: "todo",
                    },
                ],
                vec![
                    RankAbility {
                        name: "practiced persuasion",
                        rank: 1,
                        description: "todo",
                    },
                ],
            ];
        },
        ClassArchetype::Preacher => {
            return vec![
                vec![
                    RankAbility {
                        name: "practiced persuasion",
                        rank: 0,
                        description: "todo",
                    },
                ],
                vec![
                    RankAbility {
                        name: "denounce the heathens",
                        rank: 1,
                        description: "todo",
                    },
                ],
            ];
        },
        _ => panic!("Missing archetype '{}'", archetype.name())
    }
}

fn rogue_rank_abilities(archetype: &ClassArchetype) -> Vec<Vec<RankAbility>> {
    match archetype {
        ClassArchetype::Assassin => {
            return vec![
                vec![
                    RankAbility {
                        name: "stealthy instincts",
                        rank: 0,
                        description: "todo",
                    },
                ],
                vec![
                    RankAbility {
                        name: "sneak attack",
                        rank: 1,
                        description: "todo",
                    },
                ],
            ];
        },
        ClassArchetype::BardicMusic => {
            return vec![
                vec![
                    RankAbility {
                        name: "bardic lore",
                        rank: 0,
                        description: "todo",
                    },
                ],
                vec![
                    RankAbility {
                        name: "bardic performances",
                        rank: 1,
                        description: "todo",
                    },
                ],
            ];
        },
        ClassArchetype::CombatTrickster => {
            return vec![
                vec![
                    RankAbility {
                        name: "tricky finesse",
                        rank: 0,
                        description: "todo",
                    },
                ],
                vec![
                    RankAbility {
                        name: "combat styles",
                        rank: 1,
                        description: "todo",
                    },
                ],
            ];
        },
        ClassArchetype::JackOfAllTrades => {
            return vec![
                vec![
                    RankAbility {
                        name: "dabbler",
                        rank: 0,
                        description: "todo",
                    },
                ],
                vec![
                    RankAbility {
                        name: "skill exemplar",
                        rank: 1,
                        description: "todo",
                    },
                ],
            ];
        },
        ClassArchetype::SuaveScoundrel => {
            return vec![
                vec![
                    RankAbility {
                        name: "smooth liar",
                        rank: 0,
                        description: "todo",
                    },
                ],
                vec![
                    RankAbility {
                        name: "confound",
                        rank: 1,
                        description: "todo",
                    },
                ],
            ];
        },
        _ => panic!("Missing archetype '{}'", archetype.name())
    }
}
