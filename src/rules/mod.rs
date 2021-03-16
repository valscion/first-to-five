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
  games: PlayedGames,
}

type Play = (Player, (i128, i128));

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
    entry.insert(y, (player, (x, y)));
  }

  pub fn range(
    &self,
    (start_x, start_y): (i128, i128),
    (end_x, end_y): (i128, i128),
  ) -> impl std::iter::Iterator<Item = Play> + '_ {
    let possible_y_entries = self.0.range(start_x..end_x);
    possible_y_entries
      .filter_map(move |(_, y_map)| Some(y_map.range(start_y..end_y)))
      .flat_map(move |y_map_range| y_map_range.map(|(&_, &play)| play).collect::<Vec<Play>>())
  }

  pub fn get(&self, (x, y): &(i128, i128)) -> Option<&Play> {
    let y_range = self.0.get(x)?;
    y_range.get(y)
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
  }

  pub fn winner(&self) -> Option<Player> {
    for (first, (x, y)) in self
      .games
      .range((self.left, self.top), (self.right, self.bottom))
    {
      let horizontal_next_cells = [
        self.games.get(&(x + 1, y)),
        self.games.get(&(x + 2, y)),
        self.games.get(&(x + 3, y)),
        self.games.get(&(x + 4, y)),
      ];
      if horizontal_next_cells
        .iter()
        .all(|&item| item.map(|&play| play.0) == Some(first))
      {
        return Some(first);
      }

      let vertical_next_cells = [
        self.games.get(&(x, y + 1)),
        self.games.get(&(x, y + 2)),
        self.games.get(&(x, y + 3)),
        self.games.get(&(x, y + 4)),
      ];
      if vertical_next_cells
        .iter()
        .all(|&item| item.map(|&play| play.0) == Some(first))
      {
        return Some(first);
      }

      let diagonally_down_from_left_to_right_next_cells = [
        self.games.get(&(x + 1, y + 1)),
        self.games.get(&(x + 2, y + 2)),
        self.games.get(&(x + 3, y + 3)),
        self.games.get(&(x + 4, y + 4)),
      ];
      if diagonally_down_from_left_to_right_next_cells
        .iter()
        .all(|&item| item.map(|&play| play.0) == Some(first))
      {
        return Some(first);
      }

      let next_cells = [
        self.games.get(&(x - 1, y + 1)),
        self.games.get(&(x - 2, y + 2)),
        self.games.get(&(x - 3, y + 3)),
        self.games.get(&(x - 4, y + 4)),
      ];
      if next_cells
        .iter()
        .all(|&item| item.map(|&play| play.0) == Some(first))
      {
        return Some(first);
      }
    }
    None
  }
}

impl fmt::Display for GameArea {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "⌜")?;
    for _ in self.left..self.right {
      write!(f, "⎺")?;
    }
    writeln!(f, "⌝")?;
    for y in self.top..self.bottom {
      write!(f, "|")?;
      for x in self.left..self.right {
        let key = (x, y);
        match self.games.get(&key) {
          Some((Player::Cross, _)) => write!(f, "x")?,
          Some((Player::Naught, _)) => write!(f, "o")?,
          None => write!(f, " ")?,
        }
      }
      writeln!(f, "|")?;
    }
    write!(f, "⌞")?;
    for _ in self.left..self.right {
      write!(f, "⎽")?;
    }
    write!(f, "⌟")
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use proptest::prelude::*;

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
    let lines: Vec<&str> = template.split("\n").collect();
    let height = lines.len() as i128;
    let width = lines[0].len() as i128;
    let mut area = GameArea::default();
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
          'x' => area.mark(Player::Cross, column as i128, row as i128),
          'o' => area.mark(Player::Naught, column as i128, row as i128),
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
  fn test_full_area() {
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
    // When we add our first mark, the area should get to become 1x1 sized
    // and our first mark is the origin, regardless what x,y combination we gave.
    area.mark(Player::Cross, origin_x, origin_y);
    assert_area_formatted_to(
      &area,
      "⌜⎺⌝\n\
       |x|\n\
       ⌞⎽⌟",
    );
    // Then we push it one to the right with 1, it should grow
    area.mark(Player::Naught, origin_x + 1, origin_y);
    assert_area_formatted_to(
      &area,
      "⌜⎺⎺⌝\n\
       |xo|\n\
       ⌞⎽⎽⌟",
    );
    // Then let's go more to the left of the original left side and watch the area grow
    area.mark(Player::Cross, origin_x - 3, origin_y);
    assert_area_formatted_to(
      &area,
      "⌜⎺⎺⎺⎺⎺⌝\n\
       |x  xo|\n\
       ⌞⎽⎽⎽⎽⎽⌟",
    );
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
