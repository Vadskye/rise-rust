use crate::classes::definition as class_definition;
use crate::latex_formatting;
use numerics::Numerics;
use titlecase::titlecase;

pub fn generate_latex_basic_class_abilities(class: &class_definition::Class) -> String {
    return latex_formatting::latexify(
        format!(
            "
                \\subsection<Basic Class Abilities>
                If you are a {name}, you gain the following abilities.

                \\cf<{shorthand_name}><Defenses>
                You gain the following bonuses to your \\glossterm<defenses>: {defenses}.

                \\cf<{shorthand_name}><Resources>
                You have the following \\glossterm<resources>:
                \\begin<itemize>
                    \\item {insight_points}, which you can spend to gain additional abilities or proficiencies (see \\pcref<Insight Points>).
                    \\item {skill_points}, which you can spend to learn skills (see \\pcref<Skills>).
                    \\item {attunement_points}, which you can use to attune to items and abilities that affect you (see \\pcref<Attunement Points>).
                    \\item A \\plus{fatigue_tolerance} bonus to your \\glossterm<fatigue tolerance>, which makes it easier for you to use powerful abilities that fatigue you (see \\pcref<Fatigue>).
                \\end<itemize>

                \\cf<{shorthand_name}><Weapon Proficiencies>
            ",
            attunement_points = generate_labeled_english_number( class.attunement_points(), "\\glossterm<attunement point>", "\\glossterm<attunement points>"),
            defenses = generate_latex_defense_bonuses(&class.defenses()).trim(),
            fatigue_tolerance = class.fatigue_tolerance(),
            insight_points = generate_labeled_english_number(class.insight_points(), "\\glossterm<insight point>", "\\glossterm<insight points>"),
            name = class.name(),
            shorthand_name = class.shorthand_name(),
            skill_points = generate_labeled_english_number(class.skill_points(), "\\glossterm<skill point>", "\\glossterm<skill points>"),
        )
    );
}

fn generate_latex_defense_bonuses(defenses: &class_definition::ClassDefenseBonuses) -> String {
    return latex_formatting::latexify(format!("
        \\plus{armor} Armor, \\plus{fortitude} Fortitude, \\plus{reflex} Reflex, \\plus{mental} Mental
    ", armor=defenses.armor, fortitude=defenses.fortitude, reflex=defenses.reflex, mental=defenses.mental));
}

fn generate_labeled_english_number(val: u8, singular: &str, plural: &str) -> String {
    let converter = Numerics::builder().build();
    let english_number = converter.convert_number(val).unwrap();
    let suffix = if val == 1 { singular } else { plural };
    return format!("{} {}", titlecase(&english_number[0]), suffix);
}
