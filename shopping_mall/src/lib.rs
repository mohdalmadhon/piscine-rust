pub mod mall;
pub use mall::*;

use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    mall.floors
        .values()
        .flat_map(|floor| floor.stores.iter())
        .max_by_key(|(_, store)| store.square_meters)
        .map(|(name, store)| (name.clone(), store.clone()))
        .unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&str, Employee)> {
    let employees: Vec<(&String, &Employee)> = mall
        .floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .flat_map(|store| store.employees.iter())
        .collect();

    let max = employees
        .iter()
        .map(|(_, e)| e.salary)
        .fold(f64::MIN, f64::max);

    employees
        .into_iter()
        .filter(|(_, e)| e.salary == max)
        .map(|(name, e)| (name.as_str(), *e))
        .collect()
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let employees: usize = mall
        .floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.employees.len())
        .sum();

    employees + mall.guards.len()
}

pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let total_size: u64 = mall.floors.values().map(|floor| floor.size_limit).sum();

    let needed = (total_size / 200) as usize; 

    for (name, guard) in guards {
        if mall.guards.len() >= needed {
            break;
        }
        mall.hire_guard(name, guard);
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let (from, to) = employee.working_hours;
                let change = employee.salary * 0.1;
                if to - from >= 10 {
                    employee.raise(change);
                } else {
                    employee.cut(change);
                }
            }
        }
    }
}