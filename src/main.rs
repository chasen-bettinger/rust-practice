use crate::vector_props::vector_props::PersonalVector;

mod vector_props;
fn main() {
    println!("Hello, world!");
    let a_vector = vec![10, 2, 38, 23, 38, 23, 21];
    let v: PersonalVector = vector_props::vector_props::PersonalVector::new(a_vector);
    let median = v.median();
    println!("{}", median);

    let mode = v.mode();
    println!("{}", mode);
}
