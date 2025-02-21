use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum Effect {
    Slow,
    Weakening,
}

impl FromStr for Effect {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "slow" => Ok(Self::Slow),
            "weakening" => Ok(Self::Weakening),
            _ => Err(()),
        }
    }
}
