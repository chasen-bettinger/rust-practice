use crate::vector_props::vector_props::PersonalVector;

mod employees;
mod pig_latin;
mod vector_props;

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    println!("{}", pattern);

    match pattern.as_str() {
        "vector-stats" => {
            let a_vector = vec![10, 2, 38, 23, 38, 23, 21];
            let v: PersonalVector = vector_props::vector_props::PersonalVector::new(a_vector);
            let median = v.median();
            println!("{}", median);

            let mode = v.mode();
            println!("{}", mode);
        }
        "pig-latin" => {
            let target = String::from("first");
            pig_latin::pig_latin::work(&target);
        }
        "employees" => {
            let em = employees::employees::Employee {
                name: String::from("ok"),
                department: String::from("ok"),
            };
            em.add();
        }
        _ => println!("invalid operation!"),
    }
}
