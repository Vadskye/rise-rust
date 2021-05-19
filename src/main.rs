#![allow(dead_code)]
mod classes;
mod core_mechanics;
mod equipment;
mod latex_formatting;
mod skills;

fn main() {
    let barbarian = classes::Class::Barbarian;
    println!(
        "Barbarian! {}",
        classes::latex::generate_latex_class_definition(&barbarian)
    );
    let cleric = classes::Class::Cleric;
    println!(
        "Cleric! {}",
        classes::latex::generate_latex_class_definition(&cleric)
    );
    let rogue = classes::Class::Rogue;
    println!(
        "Rogue! {}",
        classes::latex::generate_latex_class_definition(&rogue)
    );
}
