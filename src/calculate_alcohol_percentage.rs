use crate::utilities::get_parsed_user_input;

pub fn run_calculate_alcohol_percentages() {
    let alcohol_content_of_proper_alcohol: f32 = get_parsed_user_input(
        "How much alcohol content (in percentage) is in your alcohol?",
    );
    let alcohol_content_of_final_drink: f32 = get_parsed_user_input(
        "How much alcohol (in percentage) has your final drink?",
    );
    let drink_ratio: f32 = alcohol_content_of_proper_alcohol / alcohol_content_of_final_drink;
    println!("Your drink has 1:{drink_ratio} ratio. {drink_ratio} portions of beverage per 1 portion of your alcohol.")
}
