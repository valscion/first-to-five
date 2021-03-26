use std::collections::BTreeMap;
use std::fmt;

#[derive(fmt::Debug, PartialEq, Clone, Copy)]
pub enum Player {
  Naught,
  Cross,
}

#[derive(Default)]
pub struct GameArea {
  left: i128,
  top: i128,
  right: i128,
  bottom: i128,
  winner: Option<Player>,
  games: PlayedGames,
}

struct Play {
  player: Player,
  x: i128,
  y: i128,
}

/// The values selected stored in a two-layered binary tree map
/// where the first layer has keys by X-coordinate and values are
/// binary tree maps where keys are by Y-coordinate and value contains the player.
///
/// For example: BTreeMap(100 => BTreeMap(50 => Player::Naught)) would mean that
/// at location x:100 y=50, the Naught player had put a selection.
#[derive(Default)]
struct PlayedGames(BTreeMap<i128, BTreeMap<i128, Play>>);

impl<'a> PlayedGames {
  pub fn mark(&mut self, player: Player, (x, y): (i128, i128)) {
    let entry = self.0.entry(x).or_insert(BTreeMap::new());
    entry.insert(y, Play { player, x, y });
  }

  pub fn get(&self, (x, y): &(i128, i128)) -> Option<&Play> {
    let y_range = self.0.get(x)?;
    let play = y_range.get(y);
    play
  }

  pub fn consecutive_line_of_five(&self, point: &(i128, i128)) -> Option<Vec<&Play>> {
    let Play { x, y, player } = self.get(point)?;
    let mut possible_lines_of_five = vec![];

    // Generate all possible horizontal plays with length of five
    for i in 0..5 {
      possible_lines_of_five.push(vec![
        // x grows, y stays the same
        (x - i + 0, *y),
        (x - i + 1, *y),
        (x - i + 2, *y),
        (x - i + 3, *y),
        (x - i + 4, *y),
      ])
    }
    // Generate all possible vertical plays with length of five
    for i in 0..5 {
      possible_lines_of_five.push(vec![
        // x stays the same, y grows
        (*x, y - i + 0),
        (*x, y - i + 1),
        (*x, y - i + 2),
        (*x, y - i + 3),
        (*x, y - i + 4),
      ])
    }
    // Generate all possible diagonal plays from top left to bottom right with length of five
    for i in 0..5 {
      possible_lines_of_five.push(vec![
        // both x and y grow --> we're going from top left to bottom right
        (x - i + 0, y - i + 0),
        (x - i + 1, y - i + 1),
        (x - i + 2, y - i + 2),
        (x - i + 3, y - i + 3),
        (x - i + 4, y - i + 4),
      ])
    }
    // Generate all possible diagonal plays from top right to bottom left with length of five
    for i in 0..5 {
      possible_lines_of_five.push(vec![
        // x shrinks, y grows --> we're going from top right to bottom left
        (x + i, y - i),
        (x - 1 + i, y + 1 - i),
        (x - 2 + i, y + 2 - i),
        (x - 3 + i, y + 3 - i),
        (x - 4 + i, y + 4 - i),
      ])
    }

    // Go through all the possible lines of five we have generated
    for points in possible_lines_of_five {
      // Get all the plays that have been played to a vector
      let points_to_plays: Vec<&Play> = points.iter().filter_map(|point| self.get(point)).collect();
      // If there were any blank spots, the line wasn't consecutive.
      if points_to_plays.len() != points.len() {
        continue;
      }
      // If we get here, there wasn't any blank spots in the line.
      // Now all we need to check is that all plays are the same as the given play.
      if points_to_plays
        .iter()
        .all(|line_play| line_play.player == *player)
      {
        // We found our line!
        return Some(points_to_plays);
      }
    }
    None
  }
}

impl GameArea {
  pub fn mark(&mut self, player: Player, x: i128, y: i128) {
    if self.left == 0 && self.right == 0 && self.top == 0 && self.bottom == 0 {
      // We need to set the origin to be the place where the first mark comes
      self.left = x;
      self.right = x + 1;
      self.top = y;
      self.bottom = y + 1;
    } else {
      if x < self.left {
        // We're going more to the left than the left side was
        self.left = x;
      } else if x >= self.right {
        // We're going more to the right than we had space
        self.right = x + 1;
      }

      if y < self.top {
        // We're going more to the top than top side was
        self.top = y;
      } else if y >= self.bottom {
        // We're going more to the bottom than we had space
        self.bottom = y + 1;
      }
    }

    self.games.mark(player, (x, y));

    // Then calculate if the marked play resulted in a win.
    if self.games.consecutive_line_of_five(&(x, y)).is_some() {
      self.winner = Some(player);
    }
  }

  pub fn winner(&self) -> Option<Player> {
    self.winner
  }

  pub fn width(&self) -> u128 {
    (self.right - self.left).abs() as u128
  }

  #[allow(dead_code)]
  pub fn height(&self) -> u128 {
    (self.bottom - self.top).abs() as u128
  }

  pub fn all_plays(&self) -> Vec<Option<Player>> {
    let mut plays = vec![];
    for y in self.top..self.bottom {
      for x in self.left..self.right {
        match self.games.get(&(x, y)) {
          None => plays.push(None),
          Some(play) => plays.push(Some(play.player)),
        }
      }
    }

    plays
  }
}

impl fmt::Display for GameArea {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let plays = self.all_plays();
    let width = self.width() as usize;

    write!(f, "⌜")?;
    f.write_str(&"⎺".repeat(width))?;
    write!(f, "⌝")?;
    for (i, maybe_player) in plays.iter().enumerate() {
      let x = i % width;
      if x == 0 {
        write!(f, "\n|")?;
      }

      match maybe_player {
        Some(Player::Cross) => write!(f, "x")?,
        Some(Player::Naught) => write!(f, "o")?,
        None => write!(f, " ")?,
      }

      if x == width - 1 {
        write!(f, "|")?;
      }
    }
    write!(f, "\n⌞")?;
    f.write_str(&"⎽".repeat(width))?;
    write!(f, "⌟")
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use proptest::prelude::*;
  use rand::seq::SliceRandom;

  /// Creates a new GameArea from a static template string
  ///
  /// Example creating a 4x5 area with a vertical line for
  /// Player::Cross in the second column from top to bottom:
  ///
  /// ```
  /// let area = create_area_from_template(
  ///   ".x..\n\
  ///    .x..\n\
  ///    .x..\n\
  ///    .x..\n\
  ///    .x..",
  /// );
  /// ```
  fn create_area_from_template(template: &'static str) -> GameArea {
    let mut rng = rand::thread_rng();
    let lines: Vec<&str> = template.split("\n").collect();
    let height = lines.len() as i128;
    let width = lines[0].len() as i128;
    let mut area = GameArea::default();
    let mut shuffled_lines = lines.iter().enumerate().collect::<Vec<(usize, &&str)>>();
    shuffled_lines.shuffle(&mut rng);
    for (row, &line) in shuffled_lines.iter() {
      let row_width = line.chars().count();
      assert_eq!(
        row_width,
        width as usize,
        "All line should have same width. Row {row} had a width of {wrong_width} when expected was {expected_width}",
        row = row + 1,
        wrong_width = row_width,
        expected_width = width
      );
      let mut shuffled_chars = line.chars().enumerate().collect::<Vec<(usize, char)>>();
      shuffled_chars.shuffle(&mut rng);
      for (column, character) in shuffled_chars {
        match character {
          '.' => { /* blank, do nothing */ }
          'x' => area.mark(Player::Cross, column as i128, *row as i128),
          'o' => area.mark(Player::Naught, column as i128, *row as i128),
          _ => panic!("Invalid template character: '{}'", character),
        }
      }
    }
    assert_eq!(
      (area.left - area.right).abs(),
      width,
      "The created area width isn't the same width as the template string. Maybe the rightmost or leftmost column was empty?"
    );
    assert_eq!(
      (area.top - area.bottom).abs(),
      height,
      "The created area height isn't the same height as the template string. Maybe the topmost or bottommost column was empty?"
    );
    area
  }

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
    let area = GameArea::default();
    assert_area_formatted_to(
      &area,
      "⌜⌝\n\
       ⌞⌟",
    );
  }

  #[test]
  fn test_format_full_area() {
    let mut area = GameArea::default();
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
    let mut area = GameArea::default();
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
    assert_eq!(area.width(), 3);
  }

  #[test]
  fn test_all_plays() {
    let mut area = GameArea::default();
    area.mark(Player::Naught, 0, 0);
    area.mark(Player::Naught, 2, 0);
    area.mark(Player::Cross, 1, 1);
    // Sanity check first
    assert_area_formatted_to(
      &area,
      "⌜⎺⎺⎺⌝\n\
       |o o|\n\
       | x |\n\
       ⌞⎽⎽⎽⌟",
    );
    assert_eq!(
      area.all_plays(),
      vec![
        Some(Player::Naught),
        None,
        Some(Player::Naught),
        None,
        Some(Player::Cross),
        None
      ]
    );
  }

  proptest! {
  #[test]
  fn test_area_enlargening(origin_x in -10..10i128, origin_y in -10..10i128) {
    let mut area = GameArea::default();
    // First the area should be empty
    assert_area_formatted_to(
      &area,
      "⌜⌝\n\
       ⌞⌟",
    );
    assert_eq!(area.width(), 0);
    assert_eq!(area.height(), 0);
    // When we add our first mark, the area should get to become 1x1 sized
    // and our first mark is the origin, regardless what x,y combination we gave.
    area.mark(Player::Cross, origin_x, origin_y);
    assert_area_formatted_to(
      &area,
      "⌜⎺⌝\n\
       |x|\n\
       ⌞⎽⌟",
    );
    assert_eq!(area.width(), 1);
    assert_eq!(area.height(), 1);
    // Then we push it one to the right with 1, it should grow
    area.mark(Player::Naught, origin_x + 1, origin_y);
    assert_area_formatted_to(
      &area,
      "⌜⎺⎺⌝\n\
       |xo|\n\
       ⌞⎽⎽⌟",
    );
    assert_eq!(area.width(), 2);
    assert_eq!(area.height(), 1);
    // Then let's go more to the left of the original left side and watch the area grow
    area.mark(Player::Cross, origin_x - 3, origin_y);
    assert_area_formatted_to(
      &area,
      "⌜⎺⎺⎺⎺⎺⌝\n\
       |x  xo|\n\
       ⌞⎽⎽⎽⎽⎽⌟",
    );
    assert_eq!(area.width(), 5);
    assert_eq!(area.height(), 1);
    // And then more to the top than originally was
    area.mark(Player::Naught, origin_x, origin_y - 4);
    assert_area_formatted_to(
      &area,
      "⌜⎺⎺⎺⎺⎺⌝\n\
       |   o |\n\
       |     |\n\
       |     |\n\
       |     |\n\
       |x  xo|\n\
       ⌞⎽⎽⎽⎽⎽⌟",
    );
    assert_eq!(area.width(), 5);
    assert_eq!(area.height(), 5);
    // And finally more to the bottom than there was space
    area.mark(Player::Cross, origin_x + 1, origin_y + 2);
    assert_area_formatted_to(
      &area,
      "⌜⎺⎺⎺⎺⎺⌝\n\
       |   o |\n\
       |     |\n\
       |     |\n\
       |     |\n\
       |x  xo|\n\
       |     |\n\
       |    x|\n\
       ⌞⎽⎽⎽⎽⎽⌟",
    );
    assert_eq!(area.width(), 5);
    assert_eq!(area.height(), 7);
  }
  }

  #[test]
  fn test_area_from_template() {
    let area = create_area_from_template(
      ".x..\n\
       ....\n\
       ..o.\n\
       .xx.\n\
       x..x",
    );
    assert_area_formatted_to(
      &area,
      "⌜⎺⎺⎺⎺⌝\n\
       | x  |\n\
       |    |\n\
       |  o |\n\
       | xx |\n\
       |x  x|\n\
       ⌞⎽⎽⎽⎽⌟",
    );
  }

  #[test]
  fn test_no_winner() {
    let area = create_area_from_template(
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
      create_area_from_template(
        "x.....\n\
         ......\n\
         .ooooo\n\
         ......\n\
         .....x",
      )
      .winner()
      .unwrap(),
      Player::Naught
    );

    assert_eq!(
      create_area_from_template(
        "o.....\n\
         ......\n\
         .xxxxx\n\
         ......\n\
         .....o",
      )
      .winner()
      .unwrap(),
      Player::Cross
    );
  }

  #[test]
  fn test_winner_vertical() {
    assert_eq!(
      create_area_from_template(
        "x..o..\n\
         ...o..\n\
         ...o..\n\
         ...o..\n\
         ...o..\n\
         .....x",
      )
      .winner()
      .unwrap(),
      Player::Naught
    );

    assert_eq!(
      create_area_from_template(
        "o.....\n\
         ...x..\n\
         ...x..\n\
         ...x..\n\
         ...x..\n\
         ...x.o",
      )
      .winner()
      .unwrap(),
      Player::Cross
    );
  }

  #[test]
  fn test_winner_diagonally_down_from_left_to_right() {
    assert_eq!(
      create_area_from_template(
        "o....x\n\
         .o....\n\
         ..o...\n\
         ...o..\n\
         ....o.\n\
         x.....",
      )
      .winner()
      .unwrap(),
      Player::Naught
    );

    assert_eq!(
      create_area_from_template(
        ".....o\n\
         .x....\n\
         ..x...\n\
         ...x..\n\
         ....x.\n\
         o....x",
      )
      .winner()
      .unwrap(),
      Player::Cross
    );
  }

  #[test]
  fn test_winner_diagonally_down_from_right_to_left() {
    assert_eq!(
      create_area_from_template(
        "x....o\n\
         ....o.\n\
         ...o..\n\
         ..o...\n\
         .o....\n\
         .....x",
      )
      .winner()
      .unwrap(),
      Player::Naught
    );

    assert_eq!(
      create_area_from_template(
        "o.....\n\
         ....x.\n\
         ...x..\n\
         ..x...\n\
         .x....\n\
         x....o",
      )
      .winner()
      .unwrap(),
      Player::Cross
    );
  }
}
