#![allow(dead_code, unused_imports, unused_variables, unused_mut, unreachable_code)]

pub struct Allergies {
    score: u8
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128
}

impl Allergies {
    pub fn new(score: i32) -> Self {
        Allergies {score: score as u8}
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen = *allergen as u8;
        self.score & allergen != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
      let all_of_them = vec![
          Allergen::Eggs,
          Allergen::Peanuts,
          Allergen::Shellfish,
          Allergen::Strawberries,
          Allergen::Tomatoes,
          Allergen::Chocolate,
          Allergen::Pollen,
          Allergen::Cats
      ];
      all_of_them.into_iter().filter(|al| self.is_allergic_to(al)).collect()
    }
}