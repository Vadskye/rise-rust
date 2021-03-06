// use clap::{App, Arg};
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

    // TODO: generate table of contents entries for A-Z
    let mut entries = monster_entry::generate_monster_entries();
    entries.sort_by(|a, b| a.name().unwrap_or("").cmp(b.name().unwrap_or("")));
    let entries_latex: Vec<String> = entries.iter().map(|e| e.to_latex()).collect();
    println!("{}", entries_latex.join("\n"));
}
