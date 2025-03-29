use std::{collections::HashMap, io};

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let department_name = get_input("Enter department name: ");

        if department_name.eq_ignore_ascii_case("exit") {
            break;
        }

        let employee_name = get_input("Enter employee name: ");

        departments
            .entry(department_name)
            .or_insert_with(Vec::new)
            .push(employee_name);

        println!("{departments:#?}");
    }

    // loop {
    //     let mut department_name = String::new();

    //     println!("Enter department name: ");
    //     io::stdin()
    //         .read_line(&mut department_name)
    //         .expect("Failed to read line");
    //     let department_name = department_name.trim().to_ascii_lowercase().to_string();

    //     let mut employee_name = String::new();
    //     get_name(&mut employee_name);

    // match departments.get_mut(&department_name) {
    //     Some(names) => {
    //         names.push(employee_name);
    //     }
    //     None => {
    //         departments.insert(department_name, vec![employee_name]);
    //     }
    // }

    //     println!("{departments:#?}");
    // }
}

fn get_name(name: &mut String) {
    println!("Enter employee name: ");
    io::stdin().read_line(name).expect("Failed to read line");

    *name = name.trim().to_ascii_lowercase().to_string();
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_ascii_lowercase().to_string()
}
