// use clap::{App, Arg};
use rise::classes;

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

    let mut classes = classes::Class::all();
    classes.sort_by(|a, b| a.name().cmp(b.name()));
    let classes_latex: Vec<String> = classes.iter().map(|e| e.latex_section()).collect();
    println!("{}", classes_latex.join("\n"));
}
