pub use json::*;

pub struct Food {
    pub name: String,          // <name>
    pub calories: [String; 2], // [<value_in_kJ>, <value_in_kcal>]
    pub fats: f64,             // <fats_in_g>,
    pub carbs: f64,            // <carbs_in_g>,
    pub proteins: f64,         // <proteins_in_g>,
    pub nbr_of_portions: f64,  // <portions>
}

//----------------------------------------------------------------

pub fn calculate_macros(foods: Vec<Food>) -> JsonValue {
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;

    for food in foods {
        let p = food.nbr_of_portions;
        let k_cal = food.calories[1]
            .trim_end_matches("kcal")
            .parse::<f64>()
            .unwrap();

        cals += p * k_cal;
        carbs += p * food.carbs;
        proteins += p * food.proteins;
        fats += p * food.fats;
    }

    object! {
        cals: (cals * 100.0).round() / 100.0,
        carbs: (carbs * 100.0).round() / 100.0,
        proteins: (proteins * 100.0).round() / 100.0,
        fats: (fats * 100.0).round() / 100.0
    }
}
