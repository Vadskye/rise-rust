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
}