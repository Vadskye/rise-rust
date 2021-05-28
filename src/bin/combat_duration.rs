use clap::{App, Arg};
use rise::monsters::{Monster, challenge_rating, creature_type};
use rise::core_mechanics::combat;

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
            Arg::with_name("creature type")
                .short("m")
                .long("creature-type")
                .takes_value(true),
        )
        .get_matches();

    let challenge_rating = matches.value_of("challenge rating").unwrap();
    let creature_type = matches.value_of("creature type").unwrap_or("planeforged");
    let starting_attribute = matches.value_of("starting attribute").unwrap_or("0");

    for level in vec![2, 5, 8, 11, 14, 17, 20] {
        let monster = Monster::standard_monster(
            challenge_rating::ChallengeRating::from_string(challenge_rating.to_string()),
            level,
            Some(starting_attribute.parse::<i8>().unwrap()),
            Some(creature_type::CreatureType::from_string(creature_type.to_string())),
        );
        println!("L{:0>2}: {}", level, combat::run_combat(vec![&monster], vec![&monster]));
        // println!("L{:0<2}: {}", level, combat::run_combat(vec![&monster, &monster], vec![&monster, &monster]));
    }

}
