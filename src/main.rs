use crate::calculate_sequence_easy::run_calculate_sequence_easy;
use clap::Parser;
use std::str::FromStr;

mod calculate_sequence_easy;

// TODO show user programs that can be run.

#[derive(Debug, PartialEq)]
enum Program {
    // TODO: Implement Display instead of debug.
    CalculateSequenceEasy,
}

impl FromStr for Program {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "calculate_sequence_easy" => Ok(Program::CalculateSequenceEasy),
            _ => Err(()),
        }
    }
}

/// Run one of the rewritten Python-to-Rust scripts.
#[derive(Parser)]
struct Arguments {
    /// Program to run
    // TODO: use ValueEnum instead: https://docs.rs/clap/latest/clap/trait.ValueEnum.html
    program: String,
}

fn parse_arguments() {
    let args = Arguments::parse();
    let program = Program::from_str(&args.program)
        .expect("Error. Program not recognized. Check help for more details.");
    run_program(&program);
}

fn run_program(program: &Program) {
    println!("Running {:?}...", &program);
    match &program {
        Program::CalculateSequenceEasy => run_calculate_sequence_easy(),
    }
}

fn main() {
    parse_arguments();
}
