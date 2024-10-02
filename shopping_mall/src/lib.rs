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

pub fn highest_paid_employee(mall: mall::Mall) -> Vec<mall::floor::store::employee::Employee> {
    mall.floors
        .iter()
        .flat_map(|f| f.stores.iter().flat_map(|s| s.employees.iter()))
        .filter(|e| e.salary >= 1200.0)
        .cloned()
        .collect::<Vec<mall::floor::store::employee::Employee>>()
}

pub fn nbr_of_employees(mall: mall::Mall) -> usize {
    mall.guards.len()
        + mall
            .floors
            .iter()
            .flat_map(|f| f.stores.iter())
            .map(|s| s.employees.len())
            .sum::<usize>()
}

pub fn check_for_securities(mall: &mall::Mall, guards: Vec<mall::guard::Guard>) {
    while 
}

pub fn cut_or_raise(mall: &mall::Mall) {
    todo!()
}
