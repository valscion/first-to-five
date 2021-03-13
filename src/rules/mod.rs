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

  /// Creates a new GameArea from a static template string
  ///
  /// Example creating a 4x5 area with a vertical line for
  /// Player::Cross in the second column from top to bottom:
  ///
  /// ```
  /// let area = GameArea::from_template(
  ///   ".x..\n\
  ///    .x..\n\
  ///    .x..\n\
  ///    .x..\n\
  ///    .x..",
  /// );
  /// ```
  pub fn from_template(template: &'static str) -> Self {
    let lines: Vec<&str> = template.split("\n").collect();
    let height = lines.len() as u128;
    let width = lines[0].len() as u128;
    let mut area = GameArea::new(width, height);
    for (row, line) in lines.iter().enumerate() {
      let row_width = line.chars().count();
      assert_eq!(
        row_width,
        width as usize,
        "All line should have same width. Row {row} had a width of {wrong_width} when expected was {expected_width}",
        row = row + 1,
        wrong_width = row_width,
        expected_width = width
      );
      for (column, character) in line.chars().enumerate() {
        match character {
          '.' => { /* blank, do nothing */ }
          'x' => area.mark(Player::Cross, column as u128, row as u128),
          'o' => area.mark(Player::Naught, column as u128, row as u128),
          _ => panic!("Invalid template character: '{}'", character),
        }
      }
    }
    area
  }

  pub fn mark(&mut self, player: Player, x: u128, y: u128) {
    let key = format!("{},{}", x, y);
    self.games.insert(key, player);
  }

  pub fn winner(&self) -> Option<Player> {
    // First test if anyones won horizontally
    for y in 0..self.height {
      let mut last_seen_player: Option<&Player> = None;
      let mut same_player_seen_times = 0;
      for x in 0..self.width {
        let key = format!("{},{}", x, y);
        let player = self.games.get(&key);
        match &player {
          None => {
            last_seen_player = None;
            same_player_seen_times = 0;
          }
          _ if player == last_seen_player => {
            same_player_seen_times += 1;
          }
          _ => {
            last_seen_player = player;
            same_player_seen_times = 1;
          }
        }
        if same_player_seen_times == 5 {
          match last_seen_player.unwrap() {
            Player::Cross => {
              return Some(Player::Cross);
            }
            Player::Naught => {
              return Some(Player::Naught);
            }
          }
        }
      }
    }
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

  fn assert_area_formatted_to(area: &GameArea, expected: &str) {
    let formatted_area = format!("{}", area);

    assert_eq!(
      formatted_area,
      expected,
      "{}",
      colored_diff::PrettyDifference {
        expected,
        actual: &formatted_area
      }
    );
  }

  #[test]
  fn test_format_empty_area() {
    let area = GameArea::new(0, 0);
    assert_area_formatted_to(
      &area,
      "⌜⌝\n\
       ⌞⌟",
    );
  }

  #[test]
  fn test_empty_two_by_one_area() {
    let area = GameArea::new(2, 1);
    assert_area_formatted_to(
      &area,
      "⌜⎺⎺⌝\n\
       |  |\n\
       ⌞⎽⎽⌟",
    );
  }

  #[test]
  fn test_full_area() {
    let mut area = GameArea::new(2, 2);
    area.mark(Player::Naught, 0, 0);
    area.mark(Player::Naught, 1, 0);
    area.mark(Player::Cross, 0, 1);
    area.mark(Player::Cross, 1, 1);
    assert_area_formatted_to(
      &area,
      "⌜⎺⎺⌝\n\
       |oo|\n\
       |xx|\n\
       ⌞⎽⎽⌟",
    );
  }

  #[test]
  fn test_partial_area() {
    let mut area = GameArea::new(3, 2);
    area.mark(Player::Naught, 0, 0);
    area.mark(Player::Naught, 2, 0);
    area.mark(Player::Cross, 1, 1);
    assert_area_formatted_to(
      &area,
      "⌜⎺⎺⎺⌝\n\
       |o o|\n\
       | x |\n\
       ⌞⎽⎽⎽⌟",
    );
  }

  #[test]
  fn test_area_from_template() {
    let area = GameArea::from_template(
      "....\n\
       .x..\n\
       ..o.\n\
       .xx.\n\
       x..x",
    );
    assert_area_formatted_to(
      &area,
      "⌜⎺⎺⎺⎺⌝\n\
       |    |\n\
       | x  |\n\
       |  o |\n\
       | xx |\n\
       |x  x|\n\
       ⌞⎽⎽⎽⎽⌟",
    );
  }

  #[test]
  fn test_no_winner() {
    let area = GameArea::from_template(
      ".x..oo\n\
       .x..o.\n\
       .oooo.\n\
       xoxxxx\n\
       .x..o.",
    );
    assert_eq!(area.winner(), None);
  }

  #[test]
  fn test_winner_horizontal() {
    assert_eq!(
      GameArea::from_template(
        "......\n\
         ......\n\
         .ooooo\n\
         ......\n\
         ......",
      )
      .winner()
      .unwrap(),
      Player::Naught
    );

    assert_eq!(
      GameArea::from_template(
        "......\n\
         ......\n\
         .xxxxx\n\
         ......\n\
         ......",
      )
      .winner()
      .unwrap(),
      Player::Cross
    );
  }

  #[test]
  fn test_winner_vertical() {
    assert_eq!(
      GameArea::from_template(
        "...o..\n\
         ...o..\n\
         ...o..\n\
         ...o..\n\
         ...o..\n\
         ......",
      )
      .winner()
      .unwrap(),
      Player::Naught
    );

    assert_eq!(
      GameArea::from_template(
        "......\n\
         ...x..\n\
         ...x..\n\
         ...x..\n\
         ...x..\n\
         ...x..",
      )
      .winner()
      .unwrap(),
      Player::Cross
    );
  }
}
