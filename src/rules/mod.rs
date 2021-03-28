use std::collections::BTreeMap;
use std::fmt;

#[derive(fmt::Debug, PartialEq, PartialOrd, Clone, Copy)]
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

#[derive(fmt::Debug, PartialEq, PartialOrd)]
pub struct Play {
  x: i128,
  y: i128,
  player: Player,
}

/// The values selected stored in a two-layered binary tree map
/// where the first layer has keys by X-coordinate and values are
/// binary tree maps where keys are by Y-coordinate and value contains the player.
///
/// For example: BTreeMap(100 => BTreeMap(50 => Player::Naught)) would mean that
/// at location x:100 y=50, the Naught player had put a selection.
#[derive(Default)]
struct PlayedGames(BTreeMap<i128, BTreeMap<i128, Play>>);

/// The length of a line that one needs to win the game
const WINNING_LENGTH: i32 = 5;

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

  pub fn longest_consecutive_line(&self, point: &(i128, i128)) -> Option<Vec<&Play>> {
    let Play { x, y, player } = self.get(point)?;
    let mut possible_lines_of_five = vec![];

    let line_width_range = (-WINNING_LENGTH)..WINNING_LENGTH;
    let max_line_width = line_width_range.len() as i128;
    for i in line_width_range {
      let i = i as i128;
      for j in 0..max_line_width {
        // Generate all possible horizontal plays
        {
          let mut line_vec = vec![];
          for k in 0..j {
            // Horizontal plays: x grows, y stays the same
            line_vec.push((x - i + k, *y));
          }
          possible_lines_of_five.push(line_vec);
        }

        // Generate all possible vertical plays
        {
          let mut line_vec = vec![];
          for k in 0..j {
            // Vertical plays: x stays the same, y grows
            line_vec.push((*x, y - i + k));
          }
          possible_lines_of_five.push(line_vec);
        }

        // Generate all possible diagonal plays from top left to bottom right
        {
          let mut line_vec = vec![];
          for k in 0..j {
            // both x and y grow --> we're going from top left to bottom right
            line_vec.push((x - i + k, y - i + k));
          }
          possible_lines_of_five.push(line_vec);
        }

        // Generate all possible diagonal plays from top right to bottom left
        {
          let mut line_vec = vec![];
          for k in 0..j {
            // x shrinks, y grows --> we're going from top right to bottom left
            line_vec.push((x + i - k, y - i + k));
          }
          possible_lines_of_five.push(line_vec);
        }
      }
    }

    let mut longest_line: Vec<&Play> = vec![];
    // Go through all the possible lines we have generated
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
        // We found our line! Let's check if that's longest so far.
        if points_to_plays.len() > longest_line.len() {
          longest_line = points_to_plays;
        }
      }
    }
    Some(longest_line)
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
    if let Some(longest_consecutive_line) = self.games.longest_consecutive_line(&(x, y)) {
      if (longest_consecutive_line.len() as i128) >= (WINNING_LENGTH as i128) {
        self.winner = Some(player);
      }
    }
  }

  pub fn longest_consecutive_line(&self, x: i128, y: i128) -> Option<Vec<&Play>> {
    self.games.longest_consecutive_line(&(x, y))
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

  #[test]
  fn test_longest_consecutive_line_horizontal() {
    let mut area = GameArea::default();
    let player = Player::Cross;
    area.mark(player, 1, 0);
    area.mark(player, 2, 0);
    area.mark(player, 3, 0);
    area.mark(player, 4, 0);

    assert_line(
      area.longest_consecutive_line(3, 0).expect("line expected"),
      vec![
        &Play { player, x: 1, y: 0 },
        &Play { player, x: 2, y: 0 },
        &Play { player, x: 3, y: 0 },
        &Play { player, x: 4, y: 0 },
      ],
    );

    area.mark(player, 6, 0);
    area.mark(player, 7, 0);
    area.mark(player, 8, 0);
    area.mark(player, 9, 0);
    area.mark(player, 5, 0);

    assert_line(
      area.longest_consecutive_line(4, 0).expect("line expected"),
      vec![
        &Play { player, x: 1, y: 0 },
        &Play { player, x: 2, y: 0 },
        &Play { player, x: 3, y: 0 },
        &Play { player, x: 4, y: 0 },
        &Play { player, x: 5, y: 0 },
        &Play { player, x: 6, y: 0 },
        &Play { player, x: 7, y: 0 },
        &Play { player, x: 8, y: 0 },
        &Play { player, x: 9, y: 0 },
      ],
    );
  }

  #[test]
  fn test_longest_consecutive_line_vertical() {
    let mut area = GameArea::default();
    let player = Player::Cross;
    area.mark(player, 0, 1);
    area.mark(player, 0, 2);
    area.mark(player, 0, 3);
    area.mark(player, 0, 4);

    assert_line(
      area.longest_consecutive_line(0, 3).expect("line expected"),
      vec![
        &Play { player, x: 0, y: 1 },
        &Play { player, x: 0, y: 2 },
        &Play { player, x: 0, y: 3 },
        &Play { player, x: 0, y: 4 },
      ],
    );

    area.mark(player, 0, 6);
    area.mark(player, 0, 7);
    area.mark(player, 0, 8);
    area.mark(player, 0, 9);
    area.mark(player, 0, 5);

    assert_line(
      area.longest_consecutive_line(0, 4).expect("line expected"),
      vec![
        &Play { player, x: 0, y: 1 },
        &Play { player, x: 0, y: 2 },
        &Play { player, x: 0, y: 3 },
        &Play { player, x: 0, y: 4 },
        &Play { player, x: 0, y: 5 },
        &Play { player, x: 0, y: 6 },
        &Play { player, x: 0, y: 7 },
        &Play { player, x: 0, y: 8 },
        &Play { player, x: 0, y: 9 },
      ],
    );
  }

  #[test]
  fn test_longest_consecutive_line_diagonally_down_from_left_to_right() {
    let mut area = GameArea::default();
    let player = Player::Cross;
    area.mark(player, 0, 1);
    area.mark(player, 1, 2);
    area.mark(player, 2, 3);
    area.mark(player, 3, 4);

    assert_line(
      area.longest_consecutive_line(2, 3).expect("line expected"),
      vec![
        &Play { player, x: 0, y: 1 },
        &Play { player, x: 1, y: 2 },
        &Play { player, x: 2, y: 3 },
        &Play { player, x: 3, y: 4 },
      ],
    );

    area.mark(player, 5, 6);
    area.mark(player, 6, 7);
    area.mark(player, 7, 8);
    area.mark(player, 8, 9);
    area.mark(player, 4, 5);

    assert_line(
      area.longest_consecutive_line(3, 4).expect("line expected"),
      vec![
        &Play { player, x: 0, y: 1 },
        &Play { player, x: 1, y: 2 },
        &Play { player, x: 2, y: 3 },
        &Play { player, x: 3, y: 4 },
        &Play { player, x: 4, y: 5 },
        &Play { player, x: 5, y: 6 },
        &Play { player, x: 6, y: 7 },
        &Play { player, x: 7, y: 8 },
        &Play { player, x: 8, y: 9 },
      ],
    );
  }

  #[test]
  fn test_longest_consecutive_line_diagonally_down_from_right_to_left() {
    let mut area = GameArea::default();
    let player = Player::Cross;
    area.mark(player, 9, 1);
    area.mark(player, 8, 2);
    area.mark(player, 7, 3);
    area.mark(player, 6, 4);

    assert_line(
      area.longest_consecutive_line(7, 3).expect("line expected"),
      vec![
        &Play { player, x: 9, y: 1 },
        &Play { player, x: 8, y: 2 },
        &Play { player, x: 7, y: 3 },
        &Play { player, x: 6, y: 4 },
      ],
    );

    area.mark(player, 4, 6);
    area.mark(player, 3, 7);
    area.mark(player, 2, 8);
    area.mark(player, 1, 9);
    area.mark(player, 5, 5);

    assert_line(
      area.longest_consecutive_line(5, 5).expect("line expected"),
      vec![
        &Play { player, x: 9, y: 1 },
        &Play { player, x: 8, y: 2 },
        &Play { player, x: 7, y: 3 },
        &Play { player, x: 6, y: 4 },
        &Play { player, x: 5, y: 5 },
        &Play { player, x: 4, y: 6 },
        &Play { player, x: 3, y: 7 },
        &Play { player, x: 2, y: 8 },
        &Play { player, x: 1, y: 9 },
      ],
    );
  }

  fn assert_line(actual_line: Vec<&Play>, expected_line: Vec<&Play>) {
    let mut expected_line = expected_line.clone();
    let mut actual_line = actual_line.clone();
    expected_line.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    actual_line.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(actual_line, expected_line);
  }
}
