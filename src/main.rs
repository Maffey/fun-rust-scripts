use crate::ackermann_function::run_ackermann_function;
use crate::calculate_sequence_easy::run_calculate_sequence_easy;
use crate::simple_timer::run_simple_timer;
use crate::text_rpg_game::run_rpg_game;
use crate::throughput::run_throughput_converter;
use calculate_alcohol_percentage::run_calculate_alcohol_percentages;
use clap::Parser;
use std::str::FromStr;

// TODO move programs to separate directory

mod ackermann_function;
mod calculate_alcohol_percentage;
mod calculate_sequence_easy;
mod simple_timer;
mod text_rpg_game;
mod throughput;
mod utilities;

// TODO show user programs that can be run.

#[derive(Debug, PartialEq)]
enum Program {
    // TODO: Implement Display instead of debug.
    CalculateAlcoholPercentage,
    CalculateSequenceEasy,
    Throughput,
    TextRpgGame,
    AckermannFunction,
    SimpleTimer,
}

impl FromStr for Program {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "calculate_alcohol_percentage" => Ok(Program::CalculateAlcoholPercentage),
            "calculate-alcohol-percentage" => Ok(Program::CalculateAlcoholPercentage),
            "cap" => Ok(Program::CalculateAlcoholPercentage),

            "calculate_sequence_easy" => Ok(Program::CalculateSequenceEasy),
            "calculate-sequence-easy" => Ok(Program::CalculateSequenceEasy),
            "cse" => Ok(Program::CalculateSequenceEasy),

            "throughput" => Ok(Program::Throughput),
            "text_rpg_game" => Ok(Program::TextRpgGame),
            "rpg" => Ok(Program::TextRpgGame),
            "ackermann" => Ok(Program::AckermannFunction),
            "simple-timer" => Ok(Program::SimpleTimer),
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

fn parse_argument() -> Program {
    let args = Arguments::parse();
    let program = Program::from_str(&args.program)
        .expect("Error. Program not recognized. Check help for more details.");
    program
}

fn run_program(program: &Program) {
    println!("Running {:?}...", &program);
    match &program {
        Program::CalculateAlcoholPercentage => run_calculate_alcohol_percentages(),
        Program::CalculateSequenceEasy => run_calculate_sequence_easy(),
        Program::Throughput => run_throughput_converter(),
        Program::TextRpgGame => run_rpg_game(),
        Program::AckermannFunction => run_ackermann_function(),
        Program::SimpleTimer => run_simple_timer(),
    }
}

fn main() {
    env_logger::init();
    let program: Program = parse_argument();
    run_program(&program);
}
