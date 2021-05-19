use crate::classes::Class;

pub enum ClassArchetype {
    // Barbarian
    BattleforgedResilience,
    Battlerager,
    OutlandSavage,
    PrimalWarrior,
    Totemist,
    // Cleric
    ClericDivineMagic,
    DivineSpellMastery,
    DomainInfluence,
    Healer,
    Preacher,
    // Rogue
    Assassin,
    BardicMusic,
    CombatTrickster,
    JackOfAllTrades,
    SuaveScoundrel,
}

impl ClassArchetype {
    pub fn class(&self) -> Class {
        match self {
            // Barbarian
            Self::BattleforgedResilience => Class::Barbarian,
            Self::Battlerager => Class::Barbarian,
            Self::OutlandSavage => Class::Barbarian,
            Self::PrimalWarrior => Class::Barbarian,
            Self::Totemist => Class::Barbarian,
            // Cleric
            Self::ClericDivineMagic => Class::Cleric,
            Self::DivineSpellMastery => Class::Cleric,
            Self::DomainInfluence => Class::Cleric,
            Self::Healer => Class::Cleric,
            Self::Preacher => Class::Cleric,
            // Rogue
            Self::Assassin => Class::Rogue,
            Self::BardicMusic => Class::Rogue,
            Self::CombatTrickster => Class::Rogue,
            Self::JackOfAllTrades => Class::Rogue,
            Self::SuaveScoundrel => Class::Rogue,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            // Barbarian
            Self::BattleforgedResilience => "Battleforged Resilience",
            Self::Battlerager => "Battlerager",
            Self::OutlandSavage => "Outland Savage",
            Self::PrimalWarrior => "Primal Warrior",
            Self::Totemist => "Totemist",
            // Cleric
            Self::ClericDivineMagic => "Divine Magic",
            Self::DivineSpellMastery => "Divine Spell Mastery",
            Self::DomainInfluence => "Domain Influence",
            Self::Healer => "Healer",
            Self::Preacher => "Preacher",
            // Rogue
            Self::Assassin => "Assassin",
            Self::BardicMusic => "Bardic Music",
            Self::CombatTrickster => "Combat Trickster",
            Self::JackOfAllTrades => "Jack of All Trades",
            Self::SuaveScoundrel => "Suave Scoundrel",
        }
    }
}

pub fn all_archetypes() -> Vec<ClassArchetype> {
    return vec![
        // Barbarian
        ClassArchetype::BattleforgedResilience,
        ClassArchetype::Battlerager,
        ClassArchetype::OutlandSavage,
        ClassArchetype::PrimalWarrior,
        ClassArchetype::Totemist,
        // Cleric
        ClassArchetype::ClericDivineMagic,
        ClassArchetype::DivineSpellMastery,
        ClassArchetype::DomainInfluence,
        ClassArchetype::Healer,
        ClassArchetype::Preacher,
        // Rogue
        ClassArchetype::Assassin,
        ClassArchetype::BardicMusic,
        ClassArchetype::CombatTrickster,
        ClassArchetype::JackOfAllTrades,
        ClassArchetype::SuaveScoundrel,
    ];
}
