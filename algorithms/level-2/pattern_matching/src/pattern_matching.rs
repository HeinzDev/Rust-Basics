use std::any::Any;

pub fn process_input(input: &dyn Any) {
    match input {
        i if i.is::<Vec<i32>>() => {
            println!("The input is a vector, size: {}", input.downcast_ref::<Vec<i32>>().unwrap().len());
        },
        i if i.is::<i32>() => {
            println!("The input is a number, double the number is: {}", input.downcast_ref::<i32>().unwrap() * 2);
        },
        i if i.is::<String>() => {
            println!("The input is a string/phrase containing {} characters", input.downcast_ref::<String>().unwrap().len());
        },
        _ => { println!("Error in the input"); },
    }
}