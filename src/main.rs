//again, i am very sorry for the colors >T_T<

use std::io;
use colored::*;

fn main() {
    #[warn(dead_code)]
    #[derive(Debug)]

    struct Employee {
        stc_name: String,
        stc_salary: i64,
        stc_id: String,
        stc_type: String,
    }

    struct Emp_data(i64, String);

    fn set_salary(i: u32) -> Emp_data {
        println!("RECEIVED: {}", i);
        match i {
            1 => Emp_data(50000, String::from("JUNIOR ENGINEER")),
            2 => Emp_data(60000, String::from("SENIOR ENGINEER")),
            _ => Emp_data(0, String::from("UNEMPLOYED")),
        }
    }

    println!("{}", "********************************".bright_yellow());
    println!("{}    {}    {}", "*".bright_yellow(), "MODULE 4 - CHALLENGE 2".bright_green(), "*".bright_yellow(),);
    println!("{}", "********************************".bright_yellow());
    
    let mut em_name = String::new();
    let mut em_id = String::new();
    let mut em_type: u32 = 0;
    
    
    println!("{}", "ENTER EMPLOYEE NAME: ".bright_cyan());
    io::stdin().read_line(&mut em_name).expect("INVALID INPUT");
    println!("{}", "--------------------------------".bright_yellow());
    println!("{}", "ENTER EMPLOYEE ID: ".bright_cyan());
    io::stdin().read_line(&mut em_id).expect("INVALID INPUT");
    println!("{}", "--------------------------------".bright_yellow());
    println!("{}", "EMPLOYEE TYPE: ".bright_cyan());
    println!("{}{}", "1 - ".bright_cyan(), "JUNIOR ENGINEER".bright_white());
    println!("{}{}", "2 - ".bright_cyan(), "SENIOR ENGINEER".bright_white());
    loop {
        let mut em_t = String::new();
        println!("{}", "ENTER EMPLOYEE TYPE CODE: ".bright_cyan());
        io::stdin()
            .read_line(&mut em_t)
            .expect("PLEASE ENTER A NUMBER BETWEEN 1 AND 2");

        em_type = match em_t.trim().parse(){
            Ok(n) => match n{
                1 | 2 => n,
                _ => {
                    println!("{} {}", "ERROR:".red(), "PLEASE ENTER A NUMBER BETWEEN 1 AND 2");
                    continue;
                },
            },
            Err(_) => {
                println!("{} {}", "ERROR:".red(), "PLEASE ENTER A NUMBER BETWEEN 1 AND 2");
                continue;
            },
        };
        break;
    }

    let emp_data = set_salary(em_type);

    let em_name = String::from(em_name.trim());
    let em_id = String::from(em_id.trim());
    
    let new_employee: Employee = Employee {
        stc_name: em_name,
        stc_salary: emp_data.0,
        stc_id: em_id,
        stc_type: emp_data.1,
    };
    
    println!("{}", "********************************".bright_yellow());
    println!("         {}           ", "NEW EMPLOYEE".bright_green());
    println!("{}", "--------------------------------".bright_yellow());
    println!("{}{}", "NAME: ".bright_cyan(), new_employee.stc_name);
    println!("{}{}", "SALARY: ".bright_cyan(), new_employee.stc_salary);
    println!("{}{}", "EMPLOYEE ID: ".bright_cyan(), new_employee.stc_id);
    println!("{}{}", "EMPLOYEE TYPE: ".bright_cyan(), new_employee.stc_type);
    println!("{}", "********************************".bright_yellow());
}
