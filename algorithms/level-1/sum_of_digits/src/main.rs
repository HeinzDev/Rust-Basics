use std::io;

fn main() {
    println!("Enter an integer: ");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    
    let num: i32 = input.trim().parse().expect("Invalid input. Please enter an integer.");
    
    let mut sum = 0;
    let mut n = num;
    
    while n != 0 {
        sum += n % 10;
        n /= 10;
    }
    
    println!("The sum of the digits of {} is: {}", num, sum);
}

