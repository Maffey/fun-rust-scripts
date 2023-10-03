pub fn run_calculate_sequence_easy() {
    // TODO add tests for sports
    let result1 = calculate_nth_element_arithmetic(1.0, 2.0, 3);
    let result2 = calculate_nth_element_geometric(1.0, 2.0, 3);
}

// def calc_nth_element_arth(a1, r, n):
// return a1 + (n - 1) * r
fn calculate_nth_element_arithmetic(a1: f32, r: f32, n: u32) -> f32 {
    a1 + ((n as f32) - 1.0) * r
}

// def calc_nth_element_geo(a1, q, n):
// return a1 * q**(n-1)
fn calculate_nth_element_geometric(a1: f32, q: f32, n: u32) -> f32 {
    a1 * q.powf((n as f32) - 1.0)
}

// while True:
// type_of_sequence = str(input("Please enter type of sequence. ('geo' or 'arth' or 'quit'): "))
// if type_of_sequence == 'quit':
// break
// first_element = float(input("Please provide the first element: "))
// second_element = float(input("Please provide the second element: "))
//
// if type_of_sequence == "arth":
// difference = second_element - first_element
// print(f"Standard formula: An = {first_element} + (n-1)*{difference}")
// nth = int(input("Which element of the sequence do you want to calculate: "))
// print("Here you go!\n" + str(calc_nth_element_arth(first_element, difference, nth)))
// else:
// difference = second_element / first_element
// print(f"Standard formula: An = {first_element} * {difference}^(n-1)")
// nth = int(input("Which element of the sequence do you want to calculate?"))
// print("Here you go!\n" + str(calc_nth_element_geo(first_element, difference, nth)))
