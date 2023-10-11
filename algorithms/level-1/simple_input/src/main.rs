use std::io;

fn main(){
    let mut input = String::new();
    while input.trim() != "fim" {
        input = String::from("");
        io::stdin().read_line(&mut input).expect("Unable to obtain the input.");
        println!("input: {}",input);
    }
}