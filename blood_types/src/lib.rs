use std::{fmt, str::FromStr};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

const ALL: [BloodType; 8] = [
    BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive },
    BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative },
    BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive },
    BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative },
    BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive },
    BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative },
    BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive },
    BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative },
];

impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen_str, rh_factor) = if let Some(rest) = s.strip_suffix('+') {
            (rest, RhFactor::Positive)
        } else if let Some(rest) = s.strip_suffix('-') {
            (rest, RhFactor::Negative)
        } else {
            return Err(());
        };

        let antigen = match antigen_str {
            "A" => Antigen::A,
            "AB" => Antigen::AB,
            "B" => Antigen::B,
            "O" => Antigen::O,
            _ => return Err(()),
        };

        Ok(BloodType { antigen, rh_factor })
    }
}

impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen = match self.antigen {
            Antigen::A => "A",
            Antigen::AB => "AB",
            Antigen::B => "B",
            Antigen::O => "O",
        };
        let rh = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}", antigen, rh)
    }
}

impl BloodType {
    pub fn can_receive_from(self, other: Self) -> bool {
        let antigen_ok = other.antigen == Antigen::O
            || self.antigen == Antigen::AB
            || self.antigen == other.antigen;

        let rh_ok = self.rh_factor == RhFactor::Positive
            || other.rh_factor == RhFactor::Negative;

        antigen_ok && rh_ok
    }

    pub fn donors(self) -> Vec<Self> {
        ALL.iter()
            .copied()
            .filter(|&other| self.can_receive_from(other))
            .collect()
    }

    pub fn recipients(self) -> Vec<Self> {
        ALL.iter()
            .copied()
            .filter(|&other| other.can_receive_from(self))
            .collect()
    }
}