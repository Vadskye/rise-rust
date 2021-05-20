mod basic_class_abilities;

use crate::classes::{self, archetype_rank_abilities, archetypes};
use crate::latex_formatting;
use titlecase::titlecase;

pub fn generate_latex_class_definition(class: &classes::Class) -> String {
    let archetypes = class.archetypes();
    let archetype_names: Vec<&str> = archetypes.iter().map(|a| a.name()).collect();
    return latex_formatting::latexify(format!(
        "
            \\newpage
            \\section<{class_name}>\\label<{class_name}>

            {archetype_table}

            \\classbasics<Alignment> {class_alignment}.

            \\classbasics<Archetypes> {class_name}s have the {archetype_names} archetypes.

            {basic_class_abilities}

            {archetypes}
        ",
        archetype_names = latex_formatting::join_string_list(&archetype_names).unwrap(),
        archetype_table = generate_archetype_table(class).trim(),
        archetypes = class
            .archetypes()
            .iter()
            .map(
                |a| generate_archetype_description(a, class.shorthand_name())
                    .trim()
                    .to_string()
            )
            .collect::<Vec<String>>()
            .join("\n\n"),
        basic_class_abilities =
            basic_class_abilities::generate_latex_basic_class_abilities(class).trim(),
        class_name = titlecase(class.name()),
        class_alignment = class.alignment(),
    ));
}

fn generate_archetype_table(class: &classes::Class) -> String {
    let archetypes = class.archetypes();
    return format!(
        "
            \\begin<dtable!*>
                \\lcaption<{class_name} Progression>
                \\begin<dtabularx><\\textwidth><l l {archetype_columns}>
                    \\tb<Rank> & \\tb<Min Level> & {archetype_headers} \\tableheaderrule
                    {rank_rows}
                \\end<dtabularx>
            \\end<dtable!*>
        ",
        archetype_columns = vec![">{\\lcol}X"; archetypes.len()].join(" "),
        archetype_headers = archetypes
            .iter()
            .map(|a| format!("\\tb<{}>", a.name()))
            .collect::<Vec<String>>()
            .join(" & "),
        class_name = titlecase(class.name()),
        rank_rows = generate_rank_rows(class),
    );
}

fn generate_rank_rows(class: &classes::Class) -> String {
    let mut rank_rows = Vec::new();
    let abilities_by_archetype_rank = generate_ability_names_by_archetype_rank(class);
    for rank in 0..abilities_by_archetype_rank.len() {
        rank_rows.push(format!(
            "
                {rank} & {minimum_level} & {archetype_abilities} \\\\
            ",
            rank = rank,
            minimum_level = if rank == 0 {
                "\\tdash".to_string()
            } else {
                format!("{}", rank * 3 - 2)
            },
            archetype_abilities = abilities_by_archetype_rank[rank],
        ))
    }
    return rank_rows
        .iter()
        .map(|s| s.trim())
        .collect::<Vec<&str>>()
        .join("\n");
}

fn generate_ability_names_by_archetype_rank(class: &classes::Class) -> Vec<String> {
    let mut abilities_by_rank_and_archetype: Vec<Vec<String>> = Vec::new();
    for archetype in class.archetypes() {
        for (rank, rank_abilities) in archetype.rank_abilities().iter().enumerate() {
            if abilities_by_rank_and_archetype.get(rank).is_none() {
                abilities_by_rank_and_archetype.push(Vec::new());
            }
            abilities_by_rank_and_archetype[rank].push(latex_formatting::uppercase_first_letter(
                &rank_abilities
                    .iter()
                    .map(|a| a.name.to_owned())
                    .collect::<Vec<String>>()
                    .join(", "),
            ));
        }
    }
    let mut abilities_by_rank: Vec<String> = Vec::new();
    for rank in 0..abilities_by_rank_and_archetype.len() {
        abilities_by_rank.push(abilities_by_rank_and_archetype[rank].join(" & "));
    }
    return abilities_by_rank;
}

fn generate_archetype_description(
    archetype: &archetypes::ClassArchetype,
    class_shorthand: &str,
) -> String {
    return format!(
        "
            \\newpage
            \\subsection<{archetype_name}>
            {short_description}

            {rank_abilities}
        ",
        archetype_name = titlecase(archetype.name()),
        rank_abilities = archetype
            .rank_abilities()
            .iter()
            .flatten()
            .map(|a| generate_rank_ability(a, class_shorthand).trim().to_string())
            .collect::<Vec<String>>()
            .join("\n\n"),
        short_description = archetype.short_description(),
    );
}

fn generate_rank_ability(
    rank_ability: &archetype_rank_abilities::RankAbility,
    class_shorthand: &str,
) -> String {
    return format!(
        "
            \\cf<{class_shorthand}>[{rank}]<{ability_name}>
            {ability_description}
        ",
        ability_description = rank_ability.description,
        ability_name = titlecase(rank_ability.name),
        class_shorthand = class_shorthand,
        rank = rank_ability.rank,
    );
}
