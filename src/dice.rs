#![allow(dead_code)]

use rand::Rng;
use std::fmt;

#[derive(Clone)]
pub struct Die {
    pub sides: u8,
}

impl fmt::Debug for Die {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "d{}", self.sides)
    }
}

impl Die {
    pub fn roll(&self) -> u8 {
        rand::rng().random_range(1..=self.sides)
    }

    pub fn to_string(&self) -> String {
        format!("d{}", self.sides)
    }
}

#[derive(Debug, Clone)]
pub struct Dice {
    pub dice: Vec<Die>,
}

impl Dice {
    pub fn new(dice: Vec<Die>) -> Self {
        Dice { dice }
    }

    pub fn to_string(&self) -> String {
        self.dice
            .iter()
            .map(|die| die.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}
