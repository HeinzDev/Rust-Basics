use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();

    let suits = vec!["Hearts", "Diamonds", "Clubs", "Spades"];
    let ranks = vec![
        "Ace", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack",
        "Queen", "King",
    ];

    let mut deck: Vec<(String, String)> = vec![];

    for suit in &suits {
        for rank in &ranks {
            deck.push((rank.to_string(), suit.to_string()));
        }
    }

    deck.shuffle(&mut rng);

    let hand: Vec<(String, String)> = deck.into_iter().take(5).collect();

    println!("Your hand:");
    for (rank, suit) in &hand {
        println!("{} of {}", rank, suit);
    }
}
