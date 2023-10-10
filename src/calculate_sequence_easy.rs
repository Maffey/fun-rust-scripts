use std::io;
use std::process::exit;
use std::str::FromStr;

const ELEMENT_OF_SEQUENCE_PROMPT: &str = "Which element of the sequence do you want to calculate?";
const INPUT_READ_ERROR: &str = "Failed to read line!";
const NOT_A_NUMBER_ERORR: &str = "Not a number. Try again.";
const PRESENT_RESULT: &str = "Here you go! The result is: ";
const ARITHMETIC_OPTION: &str = "ari";
const GEOMETRIC_OPTION: &str = "geo";
const INCORRECT_COMMAND_ERROR: &str = "Incorrect command. Try again.";

#[derive(PartialEq)]
enum Option {
    Arthmetic,
    Geometric,
    Quit,
}

impl FromStr for Option {
    // "()" means "no value produced", which essentially means error won't be returned.
    type Err = ();

    fn from_str(option: &str) -> Result<Option, Self::Err> {
        match option {
            ARITHMETIC_OPTION => Ok(Option::Arthmetic),
            GEOMETRIC_OPTION => Ok(Option::Geometric),
            "quit" => Ok(Option::Quit),
            "q" => Ok(Option::Quit),
            _ => Err(()),
        }
    }
}

pub fn run_calculate_sequence_easy() {
    // TODO add tests for sports
    // TODO Make release when ready, also for sports
    loop {
        println!("Please enter type of sequence. ('{GEOMETRIC_OPTION}' or '{ARITHMETIC_OPTION}' or 'quit'): ");
        let mut sequence_type: String = String::new();
        io::stdin()
            .read_line(&mut sequence_type)
            .expect(INPUT_READ_ERROR);

        let sequence_type: Option = match sequence_type.trim().parse() {
            Ok(option) => option,
            Err(_) => {
                println!("{INCORRECT_COMMAND_ERROR}");
                continue;
            }
        };

        if sequence_type == Option::Quit {
            exit(0);
        }

        let a1: f32 = get_parsed_user_input("Please provide the first element: ");
        let a2: f32 = get_parsed_user_input("Please provide the second element: ");
        let element_to_calculate: u32 = get_parsed_user_input(ELEMENT_OF_SEQUENCE_PROMPT);

        match sequence_type {
            Option::Arthmetic => process_arithmetic_sequence(a1, a2 - a1, element_to_calculate),
            Option::Geometric => process_geometric_sequence(a1, a2 / a1, element_to_calculate),
            _ => println!("{INCORRECT_COMMAND_ERROR}"),
        }
    }
}

fn process_arithmetic_sequence(a1: f32, difference: f32, n: u32) {
    println!("Standard formula: An = {a1} + (n-1) * {difference}");
    let result: f32 = calculate_nth_element_arithmetic(a1, difference, n);
    println!("{PRESENT_RESULT}{result}")
}

fn process_geometric_sequence(a1: f32, difference: f32, n: u32) {
    println!("Standard formula: An = {a1} * {difference}^(n-1)");
    let result: f32 = calculate_nth_element_geometric(a1, difference, n);
    println!("{PRESENT_RESULT}{result}")
}

fn get_parsed_user_input<T: FromStr>(input_prompt: &str) -> T {
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
