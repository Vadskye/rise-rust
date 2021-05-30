pub enum Size {
    Fine,
    Diminuitive,
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
    Gargantuan,
    Colossal,
}

impl Size {
    pub fn base_speed(&self) -> i32 {
        match self {
            Size::Fine => 5,
            Size::Diminuitive => 10,
            Size::Tiny => 15,
            Size::Small => 20,
            Size::Medium => 30,
            Size::Large => 40,
            Size::Huge => 50,
            Size::Gargantuan => 60,
            Size::Colossal => 70,
        }
    }
}
