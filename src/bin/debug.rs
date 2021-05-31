use rise::core_mechanics::attributes::{self, HasAttributes};
use rise::equipment;
use rise::equipment::HasEquipment;
use rise::monsters;

fn main() {
    // let barbarian = classes::Class::Barbarian;
    // println!("Barbarian! {}", barbarian.to_latex());
    // let cleric = classes::Class::Cleric;
    // println!("Cleric! {}", cleric.to_latex());
    // let rogue = classes::Class::Rogue;
    // println!("Rogue! {}", rogue.to_latex());

    // let mut barbarian = core_mechanics::character::Character::new(classes::BARBARIAN, 1);
    // println!("{}", barbarian.to_latex());
    // barbarian.set_base_attribute(attributes::STR, -1);
    // barbarian.set_base_attribute(attributes::DEX, 4);
    // barbarian.set_base_attribute(attributes::INT, 2);
    // println!("{}", barbarian.to_latex());
    // barbarian.set_level(10);
    // println!("{}", barbarian.to_latex());

    let mut bear = monsters::Monster::new(
        monsters::challenge_rating::ChallengeRating::Three,
        monsters::creature_type::CreatureType::Animal,
        5,
    );
    bear.set_base_attribute(attributes::STR, 4);
    bear.add_weapon(equipment::weapons::Weapon::Bite);
    println!("{}", bear.to_section(None));
    bear.set_level(6);
    bear.add_weapon(equipment::weapons::Weapon::Claw);
    println!("{}", bear.to_section(None));
}
