use std::io;
use std::str::FromStr;

pub(crate) const INPUT_READ_ERROR: &str = "Failed to read line!";
const NOT_A_NUMBER_ERORR: &str = "Not a number. Try again.";

pub fn get_parsed_user_input<T: FromStr>(input_prompt: &str) -> T {
    loop {
        println!("{input_prompt}");
        let mut variable: String = String::new();

        io::stdin()
            .read_line(&mut variable)
            .expect(INPUT_READ_ERROR);

        let variable: T = match variable.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("{NOT_A_NUMBER_ERORR}");
                continue;
            }
        };
        return variable;
    }
}
