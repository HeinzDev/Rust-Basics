mod pattern_matching;
use pattern_matching::process_input;

fn main() {
    let number: i32 = 42;
    let text: String = String::from("Hello, World!");
    let array: Vec<i32> = vec![1, 2, 3, 4, 5];

    process_input(&number);
    process_input(&text);
    process_input(&array);
}
