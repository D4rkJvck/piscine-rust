pub trait Vehicle {
    fn model(&self) -> &str;
    fn year(&self) -> u32;
}

//____________________________________
//

pub struct Car<'a> {
    pub plate_nbr: &'a str,
    pub model: &'a str,
    pub horse_power: u32,
    pub year: u32,
}

impl<'a> Vehicle for Car<'a> {
    fn model(&self) -> &'a str {
        self.model
    }

    fn year(&self) -> u32 {
        self.year
    }
}

//____________________________________
//

pub struct Truck<'a> {
    pub plate_nbr: &'a str,
    pub model: &'a str,
    pub horse_power: u32,
    pub year: u32,
    pub load_tons: u32,
}

impl<'a> Vehicle for Truck<'a> {
    fn model(&self) -> &'a str {
        self.model
    }

    fn year(&self) -> u32 {
        self.year
    }
}

//____________________________________
//

pub fn all_models<'a>(vehicles: Vec<&'a dyn Vehicle>) -> Vec<&'a str> {
    vehicles.iter().map(|v| v.model()).collect()
}
