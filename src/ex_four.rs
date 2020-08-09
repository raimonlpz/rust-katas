// using hash maps and vectors, create a text interface to allow a user to add employee names to a department in
// a company. f.ex: 'Add Sally to Enginerring' or 'Add Amir to Sales'. then let the user retrieve a list of all people
// in a department or all people in the company by department, sorted alpha...

use std::collections::hash_map::{Entry, HashMap};

#[derive(Debug)]
pub struct Company {
    pub departments: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn new() -> Company {
        let departments: HashMap<String, Vec<String>> = HashMap::new();
        Company { departments }
    }
    pub fn add_employee_to_dept(&mut self, dpt: &str, empl: &str) -> () {
        self.departments
            .entry(String::from(dpt))
            .or_insert_with(Vec::new)
            .push(String::from(empl));

        println!("Adding {} to {}... | {:?}", empl, dpt, self);
    }
    pub fn remove_employee_from_dept(&mut self, dpt: &str, empl: &str) -> () {
        match self.departments.entry(String::from(dpt)) {
            Entry::Occupied(mut employees) => {
                let e: &mut Vec<String> = employees.get_mut();
                if e.contains(&String::from(empl)) {
                    e.retain(|x| x != empl);
                    println!("Removing {} from {}... | {:?}", empl, dpt, self);
                } else {
                    println!("Employee '{}' not found in department '{}'", empl, dpt);
                }
            }
            Entry::Vacant(dept) => println!("Department {:?} not found...", dept.key()),
        }
    }
    pub fn retrieve_department(&self, dpt: &str) -> Vec<String> {
        match self.departments.get(&String::from(dpt)) {
            Some(employees) => {
                let mut e: Vec<String> = employees.to_vec();
                e.sort();
                println!(
                    "Retrieving all employees from '{}' department... | {:?}",
                    dpt, e
                );
                e
            }
            None => {
                println!("Department '{}' not found...", dpt);
                Vec::new()
            }
        }
    }
    pub fn retrieve_company(&self) -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();
        for (k, v) in &self.departments {
            let mut vect = v.to_vec();
            vect.sort();
            map.insert(k.to_string(), vect);
        }
        println!("Retrieving all departments comp data... {:?}", map);
        map
    }
}
