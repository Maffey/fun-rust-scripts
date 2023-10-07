use std::io;
use std::process::exit;

const ELEMENT_OF_SEQUENCE_PROMPT: &str = "Which element of the sequence do you want to calculate?";
const PRESENT_RESULT: &str = "Here you go! The result is: ";

pub fn run_calculate_sequence_easy() {
    // TODO Make release when ready, also for sports
    // TODO add tests for sports
    loop {
        println!("Please enter type of sequence. ('geo' or 'arth' or 'quit'): ");
        let mut sequence_type: String = String::new();
        io::stdin()
            .read_line(&mut sequence_type)
            .expect("Failed to read line!");

        match sequence_type.trim() {
            "arth" => process_arithmetic_sequence(
                parse_user_number_input("Please provide the first element: "),
                parse_user_number_input("Please provide the second element: "),
            ),
            "geo" => process_geometic_sequence(
                parse_user_number_input("Please provide the first element: "),
                parse_user_number_input("Please provide the second element: "),
            ),
            "quit" => exit(0),
            _ => println!("Incorrect command. Try again."),
        }
    }
}

fn parse_user_number_input(input_prompt: &str) -> f32 {
    loop {
        println!("{input_prompt}");
        let mut variable = String::new();

        io::stdin()
            .read_line(&mut variable)
            .expect("Failed to read line!");

        let variable: f32 = match variable.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Not a number. Try again.");
                continue;
            }
        };
        return variable;
    }
}

fn process_arithmetic_sequence(a1: f32, a2: f32) {
    let difference = a2 - a1;
    println!("{ELEMENT_OF_SEQUENCE_PROMPT}");
    let mut element_to_calculate = String::new();
    io::stdin()
        .read_line(&mut element_to_calculate)
        .expect("Failed to read line!");
    // todo generic function
    let element_to_calculate: u32 = match element_to_calculate.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            // todo meh solution
            println!("Not a number. Calculating 10th element by default.");
            10
        }
    };

    println!("Standard formula: An = {a1} * {difference}^(n-1)");
    let result: f32 = calculate_nth_element_arithmetic(a1, difference, element_to_calculate);
    println!("{PRESENT_RESULT}{result}")
}

fn process_geometic_sequence(a1: f32, a2: f32) {
    // difference = second_element / first_element
    // print(f"Standard formula: An = {first_element} * {difference}^(n-1)")
    // nth = int(input("Which element of the sequence do you want to calculate?"))
    // print("Here you go!\n" + str(calc_nth_element_geo(first_element, difference, nth))
}

fn calculate_nth_element_arithmetic(a1: f32, r: f32, n: u32) -> f32 {
    a1 + ((n as f32) - 1.0) * r
}

fn calculate_nth_element_geometric(a1: f32, q: f32, n: u32) -> f32 {
    a1 * q.powf((n as f32) - 1.0)
}
