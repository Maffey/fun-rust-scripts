use std::io;
use std::process::exit;

const ELEMENT_OF_SEQUENCE_PROMPT: &str = "Which element of the sequence do you want to calculate?";
const INPUT_READ_ERROR: &str = "Failed to read line!";
const NOT_A_NUMBER_ERORR: &str = "Not a number. Try again.";
const PRESENT_RESULT: &str = "Here you go! The result is: ";

pub fn run_calculate_sequence_easy() {
    // TODO add tests for sports
    // TODO Make release when ready, also for sports
    loop {
        println!("Please enter type of sequence. ('geo' or 'arth' or 'quit'): ");
        let mut sequence_type: String = String::new();
        io::stdin()
            .read_line(&mut sequence_type)
            .expect(INPUT_READ_ERROR);

        match sequence_type.trim() {
            "arth" => process_arithmetic_sequence(
                get_parsed_user_input("Please provide the first element: "),
                get_parsed_user_input("Please provide the second element: "),
            ),
            "geo" => process_geometric_sequence(
                get_parsed_user_input("Please provide the first element: "),
                get_parsed_user_input("Please provide the second element: "),
            ),
            "quit" => exit(0),
            _ => println!("Incorrect command. Try again."),
        }
    }
}

fn process_arithmetic_sequence(a1: f32, a2: f32) {
    let element_to_calculate: u32 = get_parsed_user_input(ELEMENT_OF_SEQUENCE_PROMPT);
    let difference: f32 = a2 - a1;

    println!("Standard formula: An = {a1} + (n-1) * {difference}");
    let result: f32 = calculate_nth_element_arithmetic(a1, difference, element_to_calculate);
    println!("{PRESENT_RESULT}{result}")
}

fn process_geometric_sequence(a1: f32, a2: f32) {
    // TODO also refactorable i think
    let element_to_calculate: u32 = get_parsed_user_input(ELEMENT_OF_SEQUENCE_PROMPT);
    let difference: f32 = a2 / a1;

    println!("Standard formula: An = {a1} * {difference}^(n-1)");
    let result: f32 = calculate_nth_element_geometric(a1, difference, element_to_calculate);
    println!("{PRESENT_RESULT}{result}")
}

fn get_parsed_user_input<T: std::str::FromStr>(input_prompt: &str) -> T {
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

fn calculate_nth_element_arithmetic(a1: f32, r: f32, n: u32) -> f32 {
    a1 + ((n as f32) - 1.0) * r
}

fn calculate_nth_element_geometric(a1: f32, q: f32, n: u32) -> f32 {
    a1 * q.powf((n as f32) - 1.0)
}
