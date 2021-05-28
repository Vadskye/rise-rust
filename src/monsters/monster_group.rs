use crate::latex_formatting;
use crate::monsters::Monster;
use titlecase::titlecase;

pub struct MonsterGroup {
    monsters: Vec<Monster>,
    name: String,
}

impl MonsterGroup {
    pub fn new(name: &str, monsters: Vec<Monster>) -> MonsterGroup {
        return MonsterGroup {
            monsters,
            name: name.to_string(),
        };
    }

    pub fn to_latex(&self) -> String {
        // TODO: include general description and/or knowledge checks
        return latex_formatting::latexify(format!(
            "
                \\subsection<{name}>
                {monsters}
            ",
            name = titlecase(self.name.as_str()),
            monsters = self
                .monsters
                .iter()
                .map(|m| m.to_section(Some("monsubsubsection")))
                .collect::<Vec<String>>()
                .join("\n"),
        ));
    }
}
