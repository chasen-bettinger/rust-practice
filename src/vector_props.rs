pub mod vector_props {
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct PersonalVector {
        sorted_asc: Vec<i32>,
    }

    impl PersonalVector {
        pub fn new(mut a_vector: Vec<i32>) -> PersonalVector {
            let a_sorted_vec = &mut a_vector;
            a_sorted_vec.sort();

            PersonalVector {
                sorted_asc: a_sorted_vec.to_vec(),
            }
        }

        pub fn median(&self) -> i32 {
            let length_of_vec: usize = &self.sorted_asc.len() / 2;
            let value_of_index = &self.sorted_asc.get(length_of_vec);
            if let Some(&i) = value_of_index {
                i
            } else {
                // TODO: identify how to return None here with the return type.
                // Ideally the function returns None if we cannot access that part
                // of the vector
                0
            }
        }
        pub fn mode(&self) -> String {
            let mut hm = HashMap::new();

            for i in &self.sorted_asc {
                hm.entry(i).and_modify(|e| *e += 1).or_insert(0);
            }

            let mut highest_tally = 0;
            let mut mode_numbers = Vec::new();

            for (&key, &val) in hm.iter() {
                if val > highest_tally {
                    highest_tally = val;
                    mode_numbers = vec![key.to_string()];
                    continue;
                }

                if val == highest_tally {
                    mode_numbers.push(key.to_string())
                }
            }

            return mode_numbers.join(", ");
        }
    }
}
