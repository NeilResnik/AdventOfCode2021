use std::convert::From;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct BoardEntry {
    value: i32,
    marked: bool,
}

impl BoardEntry {
    fn default() -> BoardEntry {
        BoardEntry {
            value: 0,
            marked: false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Board {
    entries: [[BoardEntry; 5]; 5],
}

impl Board {
    pub fn new(values: Vec<Vec<i32>>) -> Result<Board, BoardError> {
        if values.len() != 5 {
            return Err(BoardError::MalformedInput);
        }
        let mut entry_grid = [[BoardEntry::default(); 5]; 5];
        for (x, row) in values.iter().enumerate() {
            if row.len() != 5 {
                return Err(BoardError::MalformedInput);
            }
            for (y, val) in row.iter().enumerate() {
                entry_grid[x][y].value = *val;
            }
        }

        Ok(Board {
            entries: entry_grid,
        })
    }

    pub fn is_winner(&self) -> bool {
        let mut winning_col = [true; 5];
        for row in self.entries {
            let mut winning_row = true;
            for (col, val) in row.iter().enumerate() {
                winning_row = winning_row && val.marked;
                winning_col[col] = winning_col[col] && val.marked;
            }
            if winning_row {
                return true;
            }
        }

        for w_col in winning_col {
            if w_col {
                return true;
            }
        }
        false
    }

    pub fn get_sum(&self, last_num: i32) -> i32 {
        let mut sum = 0;
        for row in self.entries {
            for val in row {
                if !val.marked {
                    sum += val.value;
                }
            }
        }

        sum * last_num
    }

    pub fn mark(&mut self, number: i32) {
        for row_idx in 0..self.entries.len() {
            for col_idx in 0..self.entries[row_idx].len() {
                if self.entries[row_idx][col_idx].value == number {
                    self.entries[row_idx][col_idx].marked = true;
                }
            }
        }
    }
}

pub enum BoardError {
    MalformedInput,
}

impl From<BoardError> for String {
    fn from(error: BoardError) -> Self {
        match error {
            BoardError::MalformedInput => String::from("Malformed Input - cannot construct board"),
        }
    }
}
