use crate::classes::Class;
use crate::classes::archetype_rank_abilities;

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

    pub fn rank_abilities(&self) -> Vec<Vec<archetype_rank_abilities::RankAbility>> {
        return archetype_rank_abilities::archetype_rank_abilities(self);
    }

    pub fn short_description(&self) -> &str {
        match self {
            // Barbarian
            Self::BattleforgedResilience => "This archetype improves your durability in combat.",
            Self::Battlerager => "This archetype grants you a devastating rage, improving your combat prowess.",
            Self::OutlandSavage => "This archetype improves your mobility and combat prowess with direct, brutal abilities.",
            Self::PrimalWarrior => "This archetype grants you abilities to use in combat and improves your physical skills.",
            Self::Totemist => "This archetype allows you to embody the spirits of apex predators to improve your combat ability.",
            // Cleric
            Self::ClericDivineMagic => "
                This archetype grants you the ability to cast divine spells.
                All abilities from this archetype are \\glossterm{magical}.
            ",
            Self::DivineSpellMastery => "
                This archetype improves the divine spells you cast.
                You must have the Divine Magic archetype from the cleric class to gain the abilities from this archetype.
                All abilities from this archetype are \\glossterm{magical}.
            ",
            Self::DomainInfluence => "
                This archetype grants you divine influence over two domains of your choice.
                All abilities from this archetype are \\glossterm{magical}.
            ",
            Self::Healer => "This archetype grants you healing abilities.",
            Self::Preacher => "This archetype grants you the ability to inspire your allies and denounce or even convert your foes.",
            // Rogue
            Self::Assassin => "This archetype improves your agility, stealth, and combat prowess against unaware targets.",
            Self::BardicMusic => "This archetype grants you the ability to inspire your allies and impair your foes with musical performances.",
            Self::CombatTrickster => "This archetype grants you abilities to use in combat and improves your combat prowess.",
            Self::JackOfAllTrades => "This archetype improves your skills.",
            Self::SuaveScoundrel => "This archetype improves your deceptiveness and helps you make use of that talent in combat.",
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
