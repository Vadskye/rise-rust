use clap::{App, Arg};
use rise::monsters::{Monster, challenge_rating, creature_type};

fn main() {
    let matches = App::new("standard monster")
        .version("1.0")
        .arg(
            Arg::with_name("starting attribute")
                .short("a")
                .long("starting-attribute")
                .value_name("starting_attribute")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("challenge rating")
                .short("c")
                .long("challenge-rating")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("level")
                .short("l")
                .long("level")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("creature type")
                .short("m")
                .long("creature-type")
                .takes_value(true),
        )
        .get_matches();

    let challenge_rating = matches.value_of("challenge rating").unwrap();
    let level = matches.value_of("level").unwrap();
    let creature_type = matches.value_of("creature type").unwrap_or("planeforged");
    let starting_attribute = matches.value_of("starting attribute").unwrap_or("0");

    let monster = Monster::standard_monster(
        challenge_rating::ChallengeRating::from_string(challenge_rating.to_string()),
        level.parse::<i8>().unwrap(),
        Some(starting_attribute.parse::<i8>().unwrap()),
        Some(creature_type::CreatureType::from_string(creature_type.to_string())),
    );
    println!("{}", monster.to_section(None));
}
