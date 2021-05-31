// use clap::{App, Arg};
use rise::monsters::creature_type;
use rise::monsters::monster_entry;

fn main() {
    // let matches = App::new("standard monster")
    //     .version("1.0")
    //     .arg(
    //         Arg::with_name("output")
    //             .short("o")
    //             .long("output")
    //             .required(true)
    //             .takes_value(true),
    //     )
    //     .get_matches();
    // let output = matches.value_of("output").unwrap();

    let entries = monster_entry::generate_monster_entries();
    for creature_type in vec![creature_type::CreatureType::Animal] {
        let mut entries: Vec<&monster_entry::MonsterEntry> =
            entries[&creature_type].iter().collect();
        entries.sort_by(|a, b| a.name().unwrap_or("").cmp(b.name().unwrap_or("")));
        let entries_latex: Vec<String> = entries.iter().map(|e| e.to_latex()).collect();
        println!(
            "{}\n{}",
            creature_type.latex_section_header(),
            entries_latex.join("\n")
        );
    }
}
