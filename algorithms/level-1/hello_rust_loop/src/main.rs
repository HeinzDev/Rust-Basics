fn main() {
    let messages = vec![
        "Hello, Rust!",
        "Welcome to the Rust world.",
        "Rust is a powerful language.",
        "Keep learning Rust!",
    ];

    let mut index = 0;

    loop {
        println!("{}", messages[index]);
        index = (index + 1) % messages.len();
    }
}

