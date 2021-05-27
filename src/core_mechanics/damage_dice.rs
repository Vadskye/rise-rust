pub struct DamageDice {
    count: u8,
    increments: u8,
    size: u8,
}

pub static D6: u8 = 4;
pub static D8: u8 = 5;
pub static D10: u8 = 6;

impl DamageDice {
    pub fn new(increments: u8) -> DamageDice {
        // 4d10+ has different scaling
        if increments >= 13 {
            return DamageDice {
                count: increments - 9,
                increments,
                size: 10,
            };
        }

        let mut increments = increments;
        let mut count: u8 = 1;
        while increments > 6 {
            increments -= 3;
            count *= 2;
        }
        let size = match increments {
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 4,
            4 => 6,
            5 => 8,
            6 => 10,
            _ => panic!("Invalid dice increments {}", increments),
        };
        return DamageDice { count, increments, size };
    }

    pub fn add(&self, increments: u8) -> DamageDice {
        Self::new(self.increments + increments)
    }

    pub fn format(&self) -> String {
        if self.size == 1 {
            return "1".to_string();
        } else {
            return format!("{}d{}", self.count, self.size);
        }
    }
}
