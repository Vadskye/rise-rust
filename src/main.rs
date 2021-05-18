mod classes;
mod latex_formatting;

fn main() {
    let barbarian = classes::Class::Barbarian;
    println!("Barbarian! {}", classes::generate_latex_basic_class_abilities(&barbarian));
    let cleric = classes::Class::Cleric;
    println!("Cleric! {}", classes::generate_latex_basic_class_abilities(&cleric));
    let rogue = classes::Class::Rogue;
    println!("Rogue! {}", classes::generate_latex_basic_class_abilities(&rogue));
}
