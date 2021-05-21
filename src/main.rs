#![allow(dead_code)]
mod classes;
mod core_mechanics;
mod equipment;
mod latex_formatting;
mod skills;

use core_mechanics::attributes::{self, AttributeCalcs};

fn main() {
    // let barbarian = classes::Class::Barbarian;
    // println!("Barbarian! {}", barbarian.to_latex());
    // let cleric = classes::Class::Cleric;
    // println!("Cleric! {}", cleric.to_latex());
    // let rogue = classes::Class::Rogue;
    // println!("Rogue! {}", rogue.to_latex());

    let mut barbarian = core_mechanics::character::Character::new(classes::Class::Barbarian, 1);
    println!("{}", barbarian.to_latex());
    barbarian.set_base_attribute(&attributes::STR, -1);
    barbarian.set_base_attribute(&attributes::DEX, 4);
    println!("{}", barbarian.to_latex());
    barbarian.set_level(10);
    println!("{}", barbarian.to_latex());
}
