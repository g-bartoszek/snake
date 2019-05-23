use crate::*;
use std::fmt::Write;

pub const WIDTH: usize = 5;
pub const HEIGHT: usize = 5;

pub struct CurrentWidthAndHeightArray<T>
where
    T: Default + Copy,
{
    data: [T; WIDTH * HEIGHT],
}

impl<T> PreallocatedArray<T> for CurrentWidthAndHeightArray<T>
where
    T: Default + Copy,
{
    fn as_slice(&self) -> &[T] {
        &self.data
    }
    fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<T> Default for CurrentWidthAndHeightArray<T>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            data: [T::default(); WIDTH * HEIGHT],
        }
    }
}

#[macro_export]
macro_rules! board_layout {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(String::from($x));
            )*
            temp_vec
        }
    }; }

#[macro_export]
macro_rules! assert_board {
    ( $actual:expr , $expected:expr ) => {{
        let result = check_board($actual, $expected);
        if !result.is_empty() {
            panic!(
                "\nExpected:\n{}Actual:\n{}Errors:\n{:?}\n",
                expected_to_string($expected),
                board_to_string($actual),
                result
            );
        }
    }};
}

pub fn expected_to_string(expected: &Vec<String>) -> String {
    let mut result = String::new();
    for e in expected {
        write!(result, "\"{}\"\n", e);
    }
    result
}

pub fn board_to_string(board: &Board) -> String {
    let mut result = String::new();
    for y in 0..board.height() {
        write!(result, "\"");
        for x in 0..board.width() {
            write!(
                result,
                "{}",
                match board.at(&Location {
                    x: x as i32,
                    y: y as i32
                }) {
                    Square::Snake => 'O',
                    Square::Fruit => 'F',
                    Square::Empty => ' ',
                }
            );
        }
        write!(result, "\"\n");
    }
    result
}

pub fn check_board(board: &impl Board, expected: &Vec<String>) -> Vec<String> {
    assert_eq!(board.height(), expected.len(), "Invalid height");

    expected
        .iter()
        .enumerate()
        .map(|(y, row)| -> Vec<String> {
            assert_eq!(board.width(), row.chars().count(), "Invalid width");

            row.chars()
                .enumerate()
                .map(|(x, square)| {
                    let expected = match square {
                        'O' => Square::Snake,
                        'F' => Square::Fruit,
                        _ => Square::Empty,
                    };

                    if board.at(&Location {
                        x: x as i32,
                        y: y as i32,
                    }) != expected
                    {
                        Err(format!(
                            "X:{} Y:{} should be {:?} but it's {:?}",
                            x,
                            y,
                            expected,
                            board.at(&Location {
                                x: x as i32,
                                y: y as i32
                            })
                        ))
                    } else {
                        Ok(())
                    }
                })
                .filter_map(Result::err)
                .collect()
        })
        .flatten()
        .collect()
}
