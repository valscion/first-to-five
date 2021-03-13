mod rules;
use rules::{GameArea, Player};

fn main() {
    let mut area = GameArea::from_template(
        ".o.......o.\n\
         .o.......o.\n\
         .o.......o.\n\
         .o.......o.\n\
         .o.......o.",
    );
    // Mark a diagonal line in the middle of the game field
    for x in 3..8 {
        let y = x - 3;
        area.mark(Player::Cross, x, y);
    }
    println!("{}", area);
    match area.winner() {
        Some(Player::Naught) => println!("Naught has won!"),
        Some(Player::Cross) => println!("Cross has won!"),
        None => println!("No winner yet..."),
    }
}
