use std::io;
use std::process::exit;
use std::time::Instant;

fn convert_bytes(value: u64, target_units: &str) -> f32 {
    match target_units {
        "kilobits" => (value / 125) as f32,
        "kilobytes" => (value / 1_000) as f32,
        "megabytes" => (value / 1_000_000) as f32,
        "gigabytes" => (value / 1_000_000_000) as f32,
        _ => {
            println!("Unknown unit. Returning value as-is.");
            value as f32
        }
    }
}

fn process_user_input(args: (u64, &str)) {
    let throughput = convert_bytes(args.0, args.1);
    println!("{} bytes = {} {}", args.0, throughput, args.1);
}

pub fn run_throughput_converter() {
    let mut user_throughput = String::new();
    println!("Average throughput in bytes per second: ");
    io::stdin()
        .read_line(&mut user_throughput)
        .expect("Failed to read line.");

    let mut user_units = String::new();
    println!("Desired target units you want to convert to [kilobits]: ");
    io::stdin()
        .read_line(&mut user_units)
        .expect("Failed to read line.");

    let now = Instant::now();

    let user_throughput: u64 = match user_throughput.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Incorrect value!");
            exit(1);
        }
    };

    let mut user_units: &str = user_units.trim();
    if user_units.is_empty() {
        user_units = "kilobits";
    }

    process_user_input((user_throughput, user_units));

    let elapsed_time = now.elapsed();

    println!("Performance: {:?}", elapsed_time);
    println!("Press ENTER to close the program...");
    io::stdin().read_line(&mut String::new()).unwrap();
}

//  Performance: 69µs (0.000069 seconds). 14.5 faster than Python.

#[cfg(test)]
mod tests {
    use crate::throughput::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, "", 1.0)]
    #[case(1_250, "kilobits", 10.0)]
    #[case(1_000, "kilobytes", 1.0)]
    #[case(5_000_000, "megabytes", 5.0)]
    #[case(10_000_000_000, "gigabytes", 10.0)]
    fn test_convert_bytes(
        #[case] value: u64,
        #[case] target_units: &str,
        #[case] expected_result: f32,
    ) {
        assert_eq!(convert_bytes(value, target_units), expected_result)
    }
}
