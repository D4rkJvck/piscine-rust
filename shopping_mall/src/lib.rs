pub mod mall;

pub use mall::floor::*;
pub use mall::*;

pub fn biggest_store(mall: mall::Mall) -> mall::floor::store::Store {
    mall.floors
        .iter()
        .flat_map(|f| f.stores.iter())
        .max_by_key(|s| s.square_meters)
        .cloned()
        .unwrap()
}

pub fn highest_paid_employee(mall: mall::Mall) -> mall::floor::store::employee::Employee {
    todo!()
}

pub fn nbr_of_employees(mall: mall::Mall) -> usize {
    todo!()
}

pub fn check_for_securities(mall: &mall::Mall, guards: Vec<mall::guard::Guard>) {
    todo!()
}

pub fn cut_or_raise(mall: &mall::Mall) {
    todo!()
}
