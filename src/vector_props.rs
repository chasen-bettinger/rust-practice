pub mod vector_props {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct PersonalVector {
        value: Vec<i32>,
        sorted_asc: Vec<i32>,
    }

    impl PersonalVector {
        pub fn new(mut a_vector: Vec<i32>) -> PersonalVector {
            let vector_copy = Vec::clone(&a_vector);
            let a_sorted_vec = &mut a_vector;
            a_sorted_vec.sort();

            PersonalVector {
                value: vector_copy,
                sorted_asc: a_sorted_vec.to_vec(),
            }
        }

        pub fn median(&self) -> i32 {
            println!("median!");
            dbg!(&self);

            let length_of_vec_raw: usize = &self.sorted_asc.len() / 2;
            // let length_of_vec: i32 = length_of_vec_raw.round() as i32;
            let test = &self.sorted_asc.get(length_of_vec_raw);
            if let Some(&i) = test {
                i
            } else {
                0
            }
        }
        pub fn mode(&self) -> String {
            let mut hm = HashMap::new();

            for i in &self.sorted_asc {
                hm.entry(i).and_modify(|e| *e += 1).or_insert(0);
            }

            let mut highestTally = 0;
            let mut modeNumbers = Vec::new();

            for (&key, &val) in hm.iter() {
                if val > highestTally {
                    highestTally = val;
                    modeNumbers = vec![key.to_string()];
                    continue;
                }

                if val == highestTally {
                    modeNumbers.push(key.to_string())
                }
            }
            println!("mode");

            return modeNumbers.join(", ");
        }
    }
}
