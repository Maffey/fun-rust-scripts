pub fn run_ackermann_function() {
    // Higher m, n values will cause stack overflow.
    for current_m in 0..=3 {
        for current_n in 0..=8 {
            let current_ackermann = ackermann(current_m, current_n);
            println!("Input: m: {current_m}, n: {current_n}\tResult: {current_ackermann:?}")
        }
    }
}

fn ackermann(m: i64, n: i64) -> i64 {
    if m == 0 {
        n + 1
    } else if m > 0 && n == 0 {
        ackermann(m - 1, 1)
    } else {
        ackermann(m - 1, ackermann(m, n - 1))
    }
}
