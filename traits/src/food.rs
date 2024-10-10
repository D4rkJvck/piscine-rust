pub trait Food {
    fn gives(&self) -> f64;
}

pub struct Fruit {
    pub weight_in_kg: f64,
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        self.weight_in_kg * 4.0
    }
}

//________________________________________________________________
//

pub struct Meat {
    pub weight_in_kg: f64,
    pub fat_content: f64,
}

impl Food for Meat {
    fn gives(&self) -> f64 {
        let protein_ratio = self.weight_in_kg - self.fat_content;
        self.fat_content * 9.0 + protein_ratio * 4.0
    }
}
