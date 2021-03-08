mod rules;
use rules::{GameArea, Player};

fn main() {
    let mut area = GameArea::new(10, 5);
    // Mark a diagonal line in the middle of the game field
    for x in 2..=7 {
        let y = x - 2;
        area.mark(Player::Cross, x, y);
    }
    // Mark a vertical line in the end of the game field
    for y in 0..=5 {
        area.mark(Player::Naught, 9, y);
    }
    println!("{}", area);
}
