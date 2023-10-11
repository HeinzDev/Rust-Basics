use std::io;

const BOARD_SIZE: usize = 10;

struct Ship {
    x: usize,
    y: usize,
    length: usize,
    direction: Direction,
}

enum Direction {
    Horizontal,
    Vertical,
}

fn main() {
    let mut board = [['_'; BOARD_SIZE]; BOARD_SIZE];
    let ships = vec![
        Ship { x: 2, y: 4, length: 2, direction: Direction::Horizontal },
        Ship { x: 6, y: 7, length: 4, direction: Direction::Vertical },
        Ship { x: 5, y: 9, length: 3, direction: Direction::Horizontal },
    ];

    for ship in ships.iter() {
        for i in 0..ship.length {
            let (x, y) = match ship.direction {
                Direction::Horizontal => (ship.x + i, ship.y),
                Direction::Vertical => (ship.x, ship.y + i),
            };
            board[x][y] = 'S';
        }
    }

    println!("Welcome to the game!");
    println!("Try to hit the ships before you run out of shots.");

    let mut shots = 100;
    let mut ships_left = ships.len();

    while shots > 0 && ships_left > 0 {
        println!("Current board:");
        for row in board.iter() {
            for &cell in row.iter() {
                print!("{} ", cell);
            }
            println!();
        }

        println!("\nYou have {} shots remaining.", shots);
        println!("Enter the coordinates (x y) to shoot:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess: Vec<usize> = guess.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let x = guess[0];
        let y = guess[1];

        if x >= BOARD_SIZE || y >= BOARD_SIZE {
            println!("You entered an invalid coordinate.");
            continue;
        }

        if board[x][y] == 'S' {
            println!("You hit a ship!");
            board[x][y] = 'X';
            ships_left -= 1;
        } else {
            println!("You missed the target.");
            board[x][y] = 'O';
        }
        shots -= 1;
    }

    if ships_left == 0 {
        println!("Congratulations, you won the game!");
    }

    if shots < 1 {
        println!("Out of ammunition, you lost!");
    }
}
