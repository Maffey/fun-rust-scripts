use crate::utilities::get_parsed_user_input;
use std::{thread, time};
use winconsole::console;

pub fn run_simple_timer() {
    let mut timer_duration: u32 =
        get_parsed_user_input("Please, provide duration for the timer in seconds:");

    loop {
        print_clock(timer_duration);
        if timer_duration == 0 {
            break;
        }
        timer_duration = timer_duration - 1;
        thread::sleep(time::Duration::from_secs(1));
    }

    beep_alarm();
}

fn beep_alarm() {
    for _ in 0..7 {
        console::beep(1000, 400);
        console::beep(2000, 200);
    }
}

fn print_clock(timer_duration: u32) {
    let seconds = timer_duration % 60;
    let minutes = (timer_duration / 60) % 60;
    let hours = (timer_duration / 60) / 60;
    println!("{hours:0>2}:{minutes:0>2}:{seconds:0>2}");
}
