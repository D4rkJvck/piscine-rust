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

//________________________________________________________________________________________________
//

pub fn highest_paid_employee(mall: mall::Mall) -> Vec<mall::floor::store::employee::Employee> {
    let total_employees: Vec<&mall::floor::store::employee::Employee> = mall
        .floors
        .iter()
        .flat_map(|f| f.stores.iter().flat_map(|s| s.employees.iter()))
        .collect();

    let highest_salary = total_employees
        .iter()
        .map(|e| e.salary)
        .reduce(f64::max)
        .unwrap_or(0.0);

    total_employees
        .into_iter()
        .filter(|e| e.salary >= highest_salary)
        .cloned()
        .collect::<Vec<mall::floor::store::employee::Employee>>()
}

//________________________________________________________________________________________________
//

pub fn nbr_of_employees(mall: mall::Mall) -> usize {
    mall.guards.len()
        + mall
            .floors
            .iter()
            .flat_map(|f| f.stores.iter())
            .map(|s| s.employees.len())
            .sum::<usize>()
}

//________________________________________________________________________________________________
//

pub fn check_for_securities(mall: &mut mall::Mall, mut guards: Vec<mall::guard::Guard>) {
    let surface: u64 = mall.floors.iter().flat_map(|f| f.stores.iter()).map(|s| s.square_meters).sum();
    panic!("Surface: {:?}", surface);

    while surface / guards.len() as u64 > 200 {
        println!("{:#?}", guards.len());
        guards.push(guards[0].clone())
    }

    mall.guards = guards
}

//________________________________________________________________________________________________
//

pub fn cut_or_raise(mall: &mut mall::Mall) {
    mall.floors
        .iter_mut()
        .flat_map(|f| f.stores.iter_mut().flat_map(|s| s.employees.iter_mut()))
        .for_each(|e| {
            let percent = e.salary * 10.0 / 100.0;
            let work_time = e.working_hours.1 - e.working_hours.0;

            if work_time < 10 {
                e.raise((percent * 1000.0).round() / 1000.0);
            } else {
                e.cut((percent * 1000.0).round() / 1000.0);
            }
        });
}
