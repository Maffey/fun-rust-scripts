use crate::utilities::get_parsed_user_input;


pub fn run_calculate_alcohol_percentages() {
let alcohol_content: f32 =
        get_parsed_user_input("How much alcohol content (in percentage) is in your alcoholic drink?");
println!("{alcohol_content}")
}