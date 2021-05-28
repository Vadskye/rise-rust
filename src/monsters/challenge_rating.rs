pub enum ChallengeRating {
    Half,
    One,
    Two,
    Three,
    Four,
}

pub static CRHALF: &ChallengeRating = &ChallengeRating::Half;
pub static CR1: &ChallengeRating = &ChallengeRating::One;
pub static CR2: &ChallengeRating = &ChallengeRating::Two;
pub static CR3: &ChallengeRating = &ChallengeRating::Three;
pub static CR4: &ChallengeRating = &ChallengeRating::Four;

impl ChallengeRating {
    pub fn accuracy_bonus(&self) -> i8 {
        match self {
            Self::Half => 0,
            Self::One => 0,
            Self::Two => 1,
            Self::Three => 2,
            Self::Four => 3,
        }
    }

    pub fn damage_increments(&self) -> i8 {
        match self {
            Self::Half => -2,
            Self::One => -1,
            Self::Two => 0,
            Self::Three => 1,
            Self::Four => 2,
        }
    }

    // CR 3 and CR 4 monsters should be taking multiple actions per round, so their damage
    // is higher than it would appear based purely on their damage dice.
    pub fn damage_per_round_multiplier(&self) -> f64 {
        match self {
            Self::Half => 1.0,
            Self::One => 1.0,
            Self::Two => 1.0,
            Self::Three => 1.5,
            Self::Four => 2.0,
        }
    }

    pub fn defense_bonus(&self) -> i8 {
        match self {
            Self::Half => 0,
            Self::One => 0,
            Self::Two => 1,
            Self::Three => 2,
            Self::Four => 3,
        }
    }

    pub fn dr_multiplier(&self) -> f64 {
        match self {
            Self::Half => 0.0,
            Self::One => 0.0,
            Self::Two => 1.0,
            Self::Three => 2.0,
            Self::Four => 4.0,
        }
    }

    pub fn hp_multiplier(&self) -> f64 {
        match self {
            Self::Half => 0.5,
            Self::One => 1.0,
            Self::Two => 1.0,
            Self::Three => 2.0,
            Self::Four => 3.0,
        }
    }

    pub fn from_string(text: String) -> &'static Self {
        match text.as_str() {
            "0.5" => CRHALF,
            "1" => CR1,
            "2" => CR2,
            "3" => CR3,
            "4" => CR4,
            _ => panic!("Invalid challenge rating '{}'", text),
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
             ChallengeRating::Half => "0.5",
             ChallengeRating::One => "1",
             ChallengeRating::Two => "2",
             ChallengeRating::Three => "3",
             ChallengeRating::Four => "4",
        }
    }
}
