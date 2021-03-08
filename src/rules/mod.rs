use std::collections::HashMap;
use std::fmt;

#[derive(fmt::Debug, PartialEq)]
pub enum Player {
  Naught,
  Cross,
}

pub struct GameArea {
  width: u128,
  height: u128,
  /// The values selected stored in a HashMap where the keys
  /// are specifically formatted with "X,Y" formatted strings.
  /// For example: { "100,50" => Player::Naught } would mean that
  /// at location x:100 y=50, the Naught player had put a selection.
  games: HashMap<String, Player>,
}

impl GameArea {
  pub fn new(width: u128, height: u128) -> Self {
    GameArea {
      width,
      height,
      games: HashMap::default(),
    }
  }

  pub fn mark(&mut self, player: Player, x: u128, y: u128) {
    let key = format!("{},{}", x, y);
    self.games.insert(key, player);
  }

  pub fn winner(&self) -> Option<Player> {
    // TODO: Calculate a potential winner!
    None
  }
}

impl fmt::Display for GameArea {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "⌜")?;
    for _ in 0..self.width {
      write!(f, "⎺")?;
    }
    writeln!(f, "⌝")?;
    for y in 0..self.height {
      write!(f, "|")?;
      for x in 0..self.width {
        let key = format!("{},{}", x, y);
        match self.games.get(&key) {
          Some(Player::Cross) => write!(f, "x")?,
          Some(Player::Naught) => write!(f, "o")?,
          None => write!(f, " ")?,
        }
      }
      writeln!(f, "|")?;
    }
    write!(f, "⌞")?;
    for _ in 0..self.width {
      write!(f, "⎽")?;
    }
    write!(f, "⌟")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_format_empty_area() {
    let area = GameArea::new(0, 0);
    assert_eq!(format!("{}", area), "⌜⌝\n⌞⌟");
  }

  #[test]
  fn test_empty_two_by_one_area() {
    let area = GameArea::new(2, 1);
    assert_eq!(format!("{}", area), "⌜⎺⎺⌝\n|  |\n⌞⎽⎽⌟");
  }

  #[test]
  fn test_full_area() {
    let mut area = GameArea::new(2, 2);
    area.mark(Player::Naught, 0, 0);
    area.mark(Player::Naught, 1, 0);
    area.mark(Player::Cross, 0, 1);
    area.mark(Player::Cross, 1, 1);
    assert_eq!(format!("{}", area), "⌜⎺⎺⌝\n|oo|\n|xx|\n⌞⎽⎽⌟");
  }

  #[test]
  fn test_partial_area() {
    let mut area = GameArea::new(3, 2);
    area.mark(Player::Naught, 0, 0);
    area.mark(Player::Naught, 2, 0);
    area.mark(Player::Cross, 1, 1);
    assert_eq!(format!("{}", area), "⌜⎺⎺⎺⌝\n|o o|\n| x |\n⌞⎽⎽⎽⌟");
  }

  #[test]
  fn test_winner_horizontal() {
    // Create a 6x6 initial game area
    let mut area = GameArea::new(6, 6);
    // Initially nobody should've won yet
    assert!(
      area.winner().is_none(),
      "Initially no player should have won"
    );

    // Test that four of same player horizontally won't yet give a winner
    for x in 1..5 {
      area.mark(Player::Naught, x, 2);
      assert!(area.winner().is_none(), "No player should've won yet");
    }
    // Adding the fifth element completes and a winner should be selected
    area.mark(Player::Naught, 5, 2);
    assert_eq!(area.winner().unwrap(), Player::Naught);
  }
}
