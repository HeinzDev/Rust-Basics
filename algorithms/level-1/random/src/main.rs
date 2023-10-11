extern crate rand;
use rand::Rng;
use std::io;

fn main() {
    println!("Try to guess the word: Hint: Car brand");

    let brands = vec![
        "toyota",
        "ford",
        "honda",
        "chevrolet",
        "volkswagen",
        "bmw",
        "mercedes",
        "audi",
        "nissan",
        "hyundai",
        "subaru",
        "mazda",
        "jaguar",
        "porsche"
    ];
    let car = brands[rand::thread_rng().gen_range(0..brands.len())];
    let mut current_word: Vec<char> = vec!['_'; car.len()];

    loop {
        println!("Current word: {}", current_word.iter().collect::<String>());
        println!("Guess a letter");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        if guess.trim() == car {
            println!("You guessed the word! It's: {}", car);
            break;
        }

        let guess = guess.trim().chars().next().unwrap();

        let mut correct = false;

        for (i, c) in car.chars().enumerate() {
            if c == guess {
                current_word[i] = c;
                correct = true;
            }
        }

        if !correct {
            println!("Letter not found");
        }

        if current_word.iter().collect::<String>() == car {
            println!("You guessed the word: {}", car);
            break;
        }
    }
}
