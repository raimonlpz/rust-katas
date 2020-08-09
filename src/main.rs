mod ex_four;
mod ex_one;
mod ex_three;
mod ex_two;

fn main() {
    calc_ex_one([9, 3, 5, 55, 76, 4, 4, 4, 65, 4]);
    calc_ex_two(String::from("first"));
    calc_ex_two(String::from("apple"));
    calc_ex_three(30);
    calc_ex_four("Engineering", "Sally");
}

fn calc_ex_one(list: [i32; 10]) -> () {
    let mut intgs = ex_one::Integers {
        list: list.to_vec(),
    };
    let mean: f32 = ex_one::Integers::calc_mean(&intgs);
    let median: i32 = ex_one::Integers::calc_median(&mut intgs);
    let mode: i32 = ex_one::Integers::calc_mode(&intgs);
    println!("EX.1: Mean: {} | Median: {} | Mode: {}", mean, median, mode);
}

fn calc_ex_two(word: String) -> () {
    let wrd = ex_two::PigLatin { word };
    let pig_res: String = ex_two::PigLatin::pig_latin_converter(&wrd);
    println!("EX.2: {}", pig_res);
}

fn calc_ex_three(n: i32) {
    let fib = ex_three::calc_fib_num(n);
    println!("EX.3: {}", fib);
}

fn calc_ex_four(department: &str, employee: &str) -> () {
    println!("EX.4:");
    let mut comp = ex_four::Company::new();
    comp.add_employee_to_dept(department, employee);
    comp.add_employee_to_dept("RRHH", "Jerry");
    comp.add_employee_to_dept("RRHH", "Rai");
    comp.add_employee_to_dept("RRHH", "Peter");
    comp.add_employee_to_dept("RRHH", "Alice");
    comp.add_employee_to_dept("Engineering", "Michael");
    comp.add_employee_to_dept("Engineering", "Louis");
    comp.remove_employee_from_dept("Engineering", "Sally");
    comp.remove_employee_from_dept("Engineering", "Bill");
    comp.remove_employee_from_dept("Marketing", "Filly");
    comp.retrieve_department("RRHH");
    comp.retrieve_department("DevOps");
    comp.retrieve_company();
}
