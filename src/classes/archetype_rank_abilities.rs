use crate::classes::archetypes::ClassArchetype;
use titlecase::titlecase;

mod barbarian;
mod cleric;
mod druid;
mod fighter;
mod monk;
mod paladin;
mod ranger;
mod rogue;
mod sorcerer;
mod warlock;
mod wizard;

pub struct RankAbility<'a> {
    pub description: &'a str,
    pub rank: i8,
    pub name: &'a str,
}

// LaTeX
impl RankAbility<'_> {
    pub fn latex_class_feature(
        &self,
        class_shorthand: &str,
    ) -> String {
        return format!(
            "
                \\cf<{class_shorthand}>[{rank}]<{ability_name}>
                {ability_description}
            ",
            ability_description = self.description,
            ability_name = titlecase(self.name),
            class_shorthand = class_shorthand,
            rank = self.rank,
        );
    }
}

pub fn archetype_rank_abilities(archetype: &ClassArchetype) -> Vec<RankAbility> {
    match archetype {
        // Barbarian
        ClassArchetype::BattleforgedResilience => barbarian::battleforged_resilience(),
        ClassArchetype::Battlerager => barbarian::battlerager(),
        ClassArchetype::OutlandSavage => barbarian::outland_savage(),
        ClassArchetype::PrimalWarrior => barbarian::primal_warrior(),
        ClassArchetype::Totemist => barbarian::totemist(),
        // Cleric
        ClassArchetype::ClericDivineMagic => cleric::divine_magic(),
        ClassArchetype::DivineSpellMastery => cleric::divine_spell_mastery(),
        ClassArchetype::DomainInfluence => cleric::domain_influence(),
        ClassArchetype::Healer => cleric::healer(),
        ClassArchetype::Preacher => cleric::preacher(),
        // Rogue
        ClassArchetype::Assassin => rogue::assassin(),
        ClassArchetype::BardicMusic => rogue::bardic_music(),
        ClassArchetype::CombatTrickster => rogue::combat_trickster(),
        ClassArchetype::JackOfAllTrades => rogue::jack_of_all_trades(),
        ClassArchetype::SuaveScoundrel => rogue::suave_scoundrel(),
    }
}
