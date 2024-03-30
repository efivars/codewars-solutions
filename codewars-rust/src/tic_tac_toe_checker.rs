#![allow(dead_code)]
/**
 * (5 kyu)
 * https://www.codewars.com/kata/525caa5c1bf619d28c000335/rust
 *
 * If we were to set up a Tic-Tac-Toe game, we would want to know whether the board's
 * current state is solved, wouldn't we? Our goal is to create a function that will check that for us!
 */
const WIN_COMBINATIONS: [[usize; 3]; 8] = [
  // row-win
  [0, 1, 2],
  [3, 4, 5],
  [6, 7, 8],
  // col-win
  [0, 3, 6],
  [1, 4, 7],
  [2, 5, 8],
  // diag-win
  [0, 4, 8],
  [2, 4, 6],
];

enum GameState {
  Draw,
  XWon,
  OWon,
  InProgress,
}
impl GameState {
  fn to_raw(&self) -> i8 {
    match &self {
      GameState::Draw => 0,
      GameState::XWon => 1,
      GameState::OWon => 2,
      GameState::InProgress => -1,
    }
  }
}

#[derive(PartialEq)]
enum CellValue {
  Empty,
  X,
  O,
}
impl CellValue {
  fn from_raw(raw: &u8) -> CellValue {
    match raw {
      0 => CellValue::Empty,
      1 => CellValue::X,
      2 => CellValue::O,
      _ => panic!("Impossible!"),
    }
  }

  fn to_raw(&self) -> u8 {
    match &self {
      CellValue::Empty => 0,
      CellValue::X => 1,
      CellValue::O => 2,
    }
  }
}

struct TicTacToe {
  cells: Vec<CellValue>,
}
impl TicTacToe {
  fn new(raw_rows: &[&[u8; 3]; 3]) -> TicTacToe {
    TicTacToe {
      cells: raw_rows
        .into_iter()
        .fold(Vec::with_capacity(9), |mut acc, row| {
          acc.extend(row.into_iter().map(CellValue::from_raw));

          acc
        }),
    }
  }

  fn get_state(&self) -> GameState {
    for side in [CellValue::X, CellValue::O] {
      if self.is_win(&side) {
        return match side {
          CellValue::X => GameState::XWon,
          CellValue::O => GameState::OWon,
          CellValue::Empty => panic!("Impossible"),
        };
      };
    }

    match self.has_empty_space() {
      true => GameState::Draw,
      false => GameState::InProgress,
    }
  }

  fn is_win(&self, side: &CellValue) -> bool {
    let cells = self.cells.as_slice();

    match side {
      CellValue::Empty => panic!("Empty side cannot win xD"),
      s => WIN_COMBINATIONS.iter().any(|combination| {
        combination
          .into_iter()
          .all(|index| cells[*index] == *s)
      }),
    }
  }

  fn has_empty_space(&self) -> bool {
    self.cells.iter().all(|c| *c != CellValue::Empty)
  }
}

fn is_solved(raw_rows: &[&[u8; 3]; 3]) -> i8 {
  let ttt = TicTacToe::new(raw_rows);

  ttt.get_state().to_raw()
}

#[cfg(test)]
mod tests {
  use super::is_solved;

  fn dotest(board: &[&[u8; 3]; 3], expected: i8) {
    let actual = is_solved(board);
    assert!(
      actual == expected,
      "With board = {board:?}\nExpected {expected} but got {actual}"
    )
  }

  #[test]
  fn fixed_tests() {
    for (board, expected) in [
      ([&[0, 0, 1], &[0, 1, 2], &[2, 1, 0]], -1),
      ([&[1, 1, 1], &[0, 2, 2], &[0, 0, 0]], 1),
      ([&[2, 1, 2], &[2, 1, 1], &[1, 1, 2]], 1),
      ([&[2, 1, 2], &[2, 1, 1], &[1, 2, 1]], 0),
    ] {
      dotest(&board, expected);
    }
  }
}
