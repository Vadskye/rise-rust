#![allow(dead_code)]
mod classes;
mod core_mechanics;
mod equipment;
mod latex_formatting;
mod skills;

fn main() {
    // let barbarian = classes::Class::Barbarian;
    // println!("Barbarian! {}", barbarian.to_latex());
    // let cleric = classes::Class::Cleric;
    // println!("Cleric! {}", cleric.to_latex());
    // let rogue = classes::Class::Rogue;
    // println!("Rogue! {}", rogue.to_latex());

    let mut creature = core_mechanics::creature::Creature::new(1);
    println!("{}", creature.to_latex());
    creature.set_base_attribute(&core_mechanics::attributes::STR, -1);
    creature.set_base_attribute(&core_mechanics::attributes::DEX, 4);
    println!("{}", creature.to_latex());
    creature.set_level(10);
    println!("{}", creature.to_latex());
}
