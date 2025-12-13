use std::fs;

use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn take_input(name: &str) -> String {
    fs::read_to_string(name).expect(name)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn clockwise(&self) -> Self {
        match self {
            Direction::Up => Self::Right,
            Direction::Right => Self::Down,
            Direction::Down => Self::Left,
            Direction::Left => Self::Up,
        }
    }

    pub fn anti_clockwise(&self) -> Self {
        match self {
            Direction::Up => Self::Left,
            Direction::Right => Self::Up,
            Direction::Down => Self::Right,
            Direction::Left => Self::Down,
        }
    }

    pub fn clockwise_delta(&self) -> (i8, i8, Self) {
        match self {
            Direction::Up => (0, 1, Self::Right),
            Direction::Right => (1, 0, Self::Down),
            Direction::Down => (0, 1, Self::Left),
            Direction::Left => (0, -1, Self::Up),
        }
    }

    pub fn clockwise_within(
        &self,
        i: usize,
        j: usize,
        cols: usize,
        rows: usize,
    ) -> Option<(usize, usize, Self)> {
        match self {
            Direction::Up => {
                if j != rows - 1 {
                    Some((i, j + 1, Self::Right))
                } else {
                    None
                }
            }
            Direction::Right => {
                if i != cols - 1 {
                    Some((i + 1, j, Self::Down))
                } else {
                    None
                }
            }
            Direction::Down => {
                if j != 0 {
                    Some((i, j - 1, Self::Left))
                } else {
                    None
                }
            }
            Direction::Left => {
                if i != 0 {
                    Some((i - 1, j, Self::Up))
                } else {
                    None
                }
            }
        }
    }

    pub fn anti_clockwise_delta(&self) -> (i8, i8, Self) {
        match self {
            Direction::Down => (0, 1, Self::Right),
            Direction::Left => (1, 0, Self::Down),
            Direction::Up => (0, 1, Self::Left),
            Direction::Right => (0, -1, Self::Up),
        }
    }

    pub fn anti_clockwise_within(
        &self,
        i: usize,
        j: usize,
        cols: usize,
        rows: usize,
    ) -> Option<(usize, usize, Self)> {
        match self {
            Direction::Down => {
                if j != rows - 1 {
                    Some((i, j + 1, Self::Right))
                } else {
                    None
                }
            }
            Direction::Left => {
                if i != cols - 1 {
                    Some((i + 1, j, Self::Down))
                } else {
                    None
                }
            }
            Direction::Up => {
                if j != 0 {
                    Some((i, j - 1, Self::Left))
                } else {
                    None
                }
            }
            Direction::Right => {
                if i != 0 {
                    Some((i - 1, j, Self::Up))
                } else {
                    None
                }
            }
        }
    }

    pub fn four_deltas() -> [(i8, i8, Self); 4] {
        [
            (-1, 0, Self::Up),
            (0, 1, Self::Right),
            (1, 0, Self::Down),
            (0, -1, Self::Left),
        ]
    }

    pub fn four_within(
        i: usize,
        j: usize,
        cols: usize,
        rows: usize,
    ) -> [Option<(usize, usize, Self)>; 4] {
        [
            if i != 0 {
                Some((i - 1, j, Self::Up))
            } else {
                None
            },
            if j != rows - 1 {
                Some((i, j + 1, Self::Right))
            } else {
                None
            },
            if i != cols - 1 {
                Some((i + 1, j, Self::Down))
            } else {
                None
            },
            if j != 0 {
                Some((i, j - 1, Self::Left))
            } else {
                None
            },
        ]
    }

    pub fn front_left_right_delats(&self) -> [(i8, i8, Self); 3] {
        match self {
            Direction::Up => [(-1, 0, Self::Up), (0, -1, Self::Left), (0, 1, Self::Right)],
            Direction::Right => [(0, 1, Self::Right), (-1, 0, Self::Up), (1, 0, Self::Down)],
            Direction::Down => [(1, 0, Self::Down), (0, 1, Self::Right), (0, -1, Self::Left)],
            Direction::Left => [(0, -1, Self::Left), (1, 0, Self::Down), (-1, 0, Self::Up)],
        }
    }

    pub fn front_left_right_within(
        &self,
        i: usize,
        j: usize,
        cols: usize,
        rows: usize,
    ) -> [Option<(usize, usize, Self)>; 3] {
        match self {
            Direction::Up => [
                if i != 0 {
                    Some((i - 1, j, Self::Up))
                } else {
                    None
                },
                if j != 0 {
                    Some((i, j - 1, Self::Left))
                } else {
                    None
                },
                if j != rows - 1 {
                    Some((i, j + 1, Self::Right))
                } else {
                    None
                },
            ],
            Direction::Right => [
                if j != rows - 1 {
                    Some((i, j + 1, Self::Right))
                } else {
                    None
                },
                if i != 0 {
                    Some((i - 1, j, Self::Up))
                } else {
                    None
                },
                if i != cols - 1 {
                    Some((i + 1, j, Self::Down))
                } else {
                    None
                },
            ],
            Direction::Down => [
                if i != 0 {
                    Some((i - 1, j, Self::Down))
                } else {
                    None
                },
                if j != rows - 1 {
                    Some((i, j + 1, Self::Right))
                } else {
                    None
                },
                if j != 0 {
                    Some((i, j - 1, Self::Left))
                } else {
                    None
                },
            ],
            Direction::Left => [
                if j != 0 {
                    Some((i, j - 1, Self::Left))
                } else {
                    None
                },
                if i != cols - 1 {
                    Some((i + 1, j, Self::Down))
                } else {
                    None
                },
                if i != 0 {
                    Some((i - 1, j, Self::Up))
                } else {
                    None
                },
            ],
        }
    }
}

pub fn matrix_from_str(s: &str, radix: u32) -> Vec<Vec<u32>> {
    s.par_lines()
        .map(|l| l.chars().map(|c| c.to_digit(radix).unwrap()).collect())
        .collect()
}
