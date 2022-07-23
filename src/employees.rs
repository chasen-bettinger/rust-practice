pub mod employees {
    use serde::Deserialize;
    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    #[derive(Deserialize, Debug)]
    pub struct Employee {
        pub name: String,
        pub department: String,
    }

    impl Employee {
        pub fn add(&self) {
            println!("add!");
            let u = read_user_from_file("./src/employees.json").unwrap();
            // let output = fs::read("./src/employees.json").unwrap();
            println!("{:?}", u);
        }
    }

    fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Employee, Box<dyn Error>> {
        // Open the file in read-only mode with buffer.
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        let u = serde_json::from_reader(reader)?;

        println!("{:?}", u);

        // Return the `User`.
        Ok(u)
    }
}
