mod rules;
use rules::{GameArea, Player};

fn main() {
    let mut area = GameArea::new(11, 5);
    // Mark a diagonal line in the middle of the game field
    for x in 3..8 {
        let y = x - 3;
        area.mark(Player::Cross, x, y);
    }
    // Mark a vertical line near the start and end of the game field
    for y in 0..5 {
        area.mark(Player::Naught, 1, y);
        area.mark(Player::Naught, 9, y);
    }
    println!("{}", area);
}
