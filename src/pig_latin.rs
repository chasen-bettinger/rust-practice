pub mod pig_latin {
    use std::fmt::format;

    pub fn work(target: &String) -> String {
        let first_letter = target.chars().next().unwrap();
        println!("{}", first_letter);
        if "aeiouAEIOU".contains(first_letter) {
            format(format_args!("{}-{}", target, "hay"))
        } else {
            let rest_of_string = &target[1..];
            println!("{}", rest_of_string);
            format(format_args!(
                "{}-{}{}",
                rest_of_string,
                first_letter,
                String::from("ay")
            ))
        }
    }
}
