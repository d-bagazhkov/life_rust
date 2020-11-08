#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Cell {
    Dyaboo, Void
}

type Grid = Vec<Vec<Cell>>;

pub enum Direction {
    LeftUp, Up, RightUp,
    Left, Right,
    LeftDown, Down, RightDown
}

impl Direction {
    pub fn to_vec() -> Vec<Direction> {
        use Direction::*;

        vec![LeftUp, Up, RightUp, Left, Right, LeftDown, Down, RightDown]
    } 
}

pub struct Table {
    grid: Grid
}

impl Table {

    pub fn new(rows: usize, columns: usize) -> Table {
        use Cell::Void;
        let mut grid = Vec::new();
        for row in 0..rows {
            grid.push(Vec::new());
            for _ in 0..columns {
                grid[row].push(Void);
            }
        }
        Table {grid}
    }

    pub fn get<P: Into<Position>>(&self, pos: P) -> &Cell {
        let pos = pos.into();
        &self.grid[pos.row][pos.column]
    }

    pub fn set<P: Into<Position>>(&mut self, pos: P, value: Cell) {
        let pos = pos.into();
        self.grid[pos.row][pos.column] = value;
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

    pub fn around_count<P: Into<Position>>(&self, current_coord: P, search: Option<Cell>) -> u8 {
        let current_coord = current_coord.into();
        let (rows, columns) = self.get_size();
        Direction::to_vec().into_iter()
                .map(|d| current_coord.shift(d, Some(rows - 1), Some(columns - 1)))
                .map(|opt_c| match opt_c {
                    Some(coord) => {
                        if let Some(entry) = search { 
                            if &entry == self.get(coord) {1} else {0}
                        } else {1}
                    },
                    None => 0
                })
                .sum()
    }

}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl From<(usize, usize)> for Position {
    #[inline(always)]
    fn from(value: (usize, usize)) -> Position {
        Position {
            row: value.0,
            column: value.1,
        }
    }
}

impl From<[usize; 2]> for Position {
    #[inline(always)]
    fn from(value: [usize; 2]) -> Position {
        Position {
            row: value[0],
            column: value[1],
        }
    }
}


impl Position {
    pub fn shift(&self, direction: Direction, max_row: Option<usize>, max_column: Option<usize>) -> Option<Position> {
        use Direction::*;
        match direction {
            LeftUp => {
                if self.column as i32 - 1 >= 0 && self.row as i32 - 1 >= 0 {
                    Some(Position::from((self.row - 1, self.column - 1)))
                } else {
                    None
                }
            },
            Up => {
                if self.row as i32 - 1 >= 0 {
                    Some(Position::from((self.row - 1, self.column)))
                } else {
                    None
                }
            },
            RightUp => {
                if self.row as i32 - 1 >= 0 {
                    if let Some(mc) = max_column {
                        if mc >= self.column + 1 {
                            return Some(Position::from((self.row - 1, self.column + 1)))
                        }
                    } else {
                        return Some(Position::from((self.row - 1, self.column + 1)))
                    }
                }
                None
            },
            Right => {
                if let Some(mc) = max_column {
                    if mc >= self.column + 1 {
                        return Some(Position::from((self.row, self.column + 1)))
                    }
                } else {
                    return Some(Position::from((self.row, self.column + 1)))
                }
                None
            },
            RightDown => {
                if let Some(mr) = max_row {
                    if let Some(mc) = max_column {
                        if mc >= self.column + 1 && mr >= self.row + 1 {
                            return Some(Position::from((self.row + 1, self.column + 1)))
                        }
                    } else {
                        return Some(Position::from((self.row + 1, self.column + 1)))
                    }
                } else {
                    return Some(Position::from((self.row + 1, self.column + 1)))
                }
                None
            },
            Down => {
                if let Some(mr) = max_row {
                    if mr >= self.row + 1 {
                        return Some(Position::from((self.row + 1, self.column)))
                    }
                } else {
                    return Some(Position::from((self.row + 1, self.column)))
                }
                None
            },
            LeftDown => {
                if self.column as i32 - 1 >= 0 {
                    if let Some(mr) = max_row {
                        if mr >= self.row + 1 {
                            return Some(Position::from((self.row + 1, self.column - 1)))
                        }
                    } else {
                        return Some(Position::from((self.row + 1, self.column - 1)))
                    }
                }
                None
            },
            Left => {
                if self.column as i32 - 1 >= 0 {
                    Some(Position::from((self.row, self.column - 1)))
                } else {
                    None
                }
            },
        }
    }
}