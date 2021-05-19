mod classes;
mod core_mechanics;
mod equipment;
mod latex_formatting;
mod skills;

fn main() {
    let barbarian = classes::definition::Class::Barbarian;
    println!(
        "Barbarian! {}",
        classes::latex::generate_latex_basic_class_abilities(&barbarian)
    );
    let cleric = classes::definition::Class::Cleric;
    println!(
        "Cleric! {}",
        classes::latex::generate_latex_basic_class_abilities(&cleric)
    );
    let rogue = classes::definition::Class::Rogue;
    println!(
        "Rogue! {}",
        classes::latex::generate_latex_basic_class_abilities(&rogue)
    );
}
