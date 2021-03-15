mod rules;
use rules::{GameArea, Player};

fn main() {
    let mut area = GameArea::default();
    // Add one play by Naught
    area.mark(Player::Naught, 0, 0);
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
