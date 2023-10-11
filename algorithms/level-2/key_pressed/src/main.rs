
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod key_pressed;

fn main() {
    let stdin = stdin();
    
    let mut stdout = stdout().into_raw_mode().unwrap();

    for evt in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )
        .unwrap();

        match evt {
            Ok(key) => key_pressed::handle_keypress(key),
            Err(err) => eprintln!("Error to read input: {}", err),
        }

        stdout.flush().unwrap();
    }
}
