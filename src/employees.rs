// note to self:
// big things that I have learned:
//  - enforcement of what the structure of json should be in during io is great and I shouldn't fight it
//  - should not use an instance of a file to read and write; single responsibility principle!
//  - i have no idea how to idiomatically handle .expect statements and I'm not sure what to even put in there
//  - drop function is great! I'm not sure how to handle block scopes yet with everything else going on

pub mod employees {
    use serde::{Deserialize, Serialize};
    use std::fs::OpenOptions;
    use std::io::Write;

    pub type Employees = Vec<Employee>;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Employee {
        pub name: String,
        pub department: String,
    }

    impl Employee {
        pub fn add(&self, e: &Employee) {
            let readable_file = OpenOptions::new()
                .read(true)
                .open("./src/employees.json")
                .unwrap();

            // Read the JSON contents of the file as an instance of `User`.
            let mut all_employees: Employees =
                serde_json::from_reader(&readable_file).expect("AN ERROR OCCURRED");

            drop(readable_file);

            let mut file = OpenOptions::new()
                .write(true)
                .append(false)
                .open("./src/employees.json")
                .unwrap();

            let employee_as_copy = e.clone();
            all_employees.push(employee_as_copy);

            let json_pretty = serde_json::to_string_pretty(&all_employees).expect("bad :(");

            file.write_all(&json_pretty.as_bytes()).expect(":(");

            match file.flush() {
                Ok(_) => println!("Success!"),
                Err(err) => println!("failure :(, {}", err),
            }
        }
    }
}
