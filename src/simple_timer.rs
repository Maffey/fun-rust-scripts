use winconsole::console;

fn beep_alarm() {
    for _ in 0..7 {
        console::beep(1000, 400);
        console::beep(2000, 200);
    }
}

pub fn run_simple_timer() {
    beep_alarm();
}
