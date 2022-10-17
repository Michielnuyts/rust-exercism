use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

// eggs (1)
// peanuts (2)
// shellfish (4)
// strawberries (8)
// tomatoes (16)
// chocolate (32)
// pollen (64)
// cats (128)

pub struct Allergies {
    has_allergies: Vec<Allergen>,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let has_allergies = match score {
            2 => vec![Allergen::Eggs, Allergen::Peanuts],
            1 => vec![Allergen::Eggs],
            _ => vec![Allergen::Eggs],
        };

        Self { has_allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        false
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        unimplemented!("Return the list of allergens contained within the score with which the Allergies struct was made.");
    }
}
