use termion::event::{Key};

pub fn handle_keypress(key: Key){
    match key {
        Key::Ctrl('c') => {
            std::process::exit(0);
        }
        Key::Char(c) => {
            print!("{}", c);
        }
        Key::Left => {
            print!("LEFT_KEY");
        }
        Key::Right => {
            print!("RIGHT_KEY");
        }
        Key::Up => {
            print!("UP_KEY");
        }
        Key::Down => {
            print!("DOWN_KEY");
        }
        _ => (),
    }
}