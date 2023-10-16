use crate::utilities::{get_parsed_user_input, INPUT_READ_ERROR};
use std::io;
use std::process::exit;
use std::str::FromStr;

const ELEMENT_OF_SEQUENCE_PROMPT: &str = "Which element of the sequence do you want to calculate?";
const PRESENT_RESULT: &str = "Here you go! The result is: ";
const ARITHMETIC_OPTION: &str = "ari";
const GEOMETRIC_OPTION: &str = "geo";
const INCORRECT_COMMAND_ERROR: &str = "Incorrect command. Try again.";

#[derive(PartialEq)]
enum Option {
    Arithmetic,
    Geometric,
    Quit,
}

impl FromStr for Option {
    // "()" means "no value produced", which essentially means error won't be returned.
    type Err = ();

    fn from_str(option: &str) -> Result<Option, Self::Err> {
        match option {
            ARITHMETIC_OPTION => Ok(Option::Arithmetic),
            GEOMETRIC_OPTION => Ok(Option::Geometric),
            "quit" => Ok(Option::Quit),
            "q" => Ok(Option::Quit),
            _ => Err(()),
        }
    }
}

pub fn run_calculate_sequence_easy() {
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
            Option::Arithmetic => process_arithmetic_sequence(a1, a2 - a1, element_to_calculate),
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

fn calculate_nth_element_arithmetic(a1: f32, r: f32, n: u32) -> f32 {
    a1 + ((n as f32) - 1.0) * r
}

fn calculate_nth_element_geometric(a1: f32, q: f32, n: u32) -> f32 {
    a1 * q.powf((n as f32) - 1.0)
}

#[cfg(test)]
mod tests {
    use crate::calculate_sequence_easy::*;
    use rstest::rstest;

    #[rstest]
    #[case(0.0, 0.0, 0, 0.0)]
    #[case(1.0, 2.0, 2, 3.0)]
    #[case(5.0, 10.0, 20, 195.0)]
    fn test_calculate_nth_element_arithmetic(
        #[case] a1: f32,
        #[case] r: f32,
        #[case] n: u32,
        #[case] expected_result: f32,
    ) {
        assert_eq!(calculate_nth_element_arithmetic(a1, r, n), expected_result);
    }

    #[rstest]
    #[case(1.0, 2.0, 2, 2.0)]
    #[case(3.0, 3.0, 3, 27.0)]
    fn test_calculate_nth_element_geometric(
        #[case] a1: f32,
        #[case] r: f32,
        #[case] n: u32,
        #[case] expected_result: f32,
    ) {
        assert_eq!(calculate_nth_element_geometric(a1, r, n), expected_result);
    }

    #[test]
    fn test_calculate_nth_element_geometric_return_nan_when_zeroes_provided() {
        assert!(calculate_nth_element_geometric(0.0, 0.0, 0).is_nan());
    }
}
