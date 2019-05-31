use crate::*;
use generic_array;
use std::fmt::Write;
use std::process::Output;

pub const HEIGHT: usize = 5;
pub const WIDTH: usize = 5;

pub struct GenericArrayAdapter<T, S>
where
    T: Default + Copy,
    S: generic_array::ArrayLength<T>,
{
    data: generic_array::GenericArray<T, S>,
    pd: std::marker::PhantomData<S>,
}

impl<T, S> PreallocatedArray<T> for GenericArrayAdapter<T, S>
where
    T: Default + Copy,
    S: generic_array::ArrayLength<T>,
{
    fn as_slice(&self) -> &[T] {
        &self.data
    }
    fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<T, S> Default for GenericArrayAdapter<T, S>
where
    T: Default + Copy,
    S: generic_array::ArrayLength<T>,
{
    fn default() -> Self {
        Self {
            data: generic_array::GenericArray::<T, S>::default(),
            pd: std::marker::PhantomData::<S> {},
        }
    }
}

impl<T, S> Index<usize> for GenericArrayAdapter<T, S>
    where
        T: Default + Copy,
        S: generic_array::ArrayLength<T>,
{
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.data[index]
    }
}

impl<T, S> IndexMut<usize> for GenericArrayAdapter<T, S>
    where
        T: Default + Copy,
        S: generic_array::ArrayLength<T>,
{
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.data[index]
    }
}

pub type Array5x5<T> = GenericArrayAdapter<T, generic_array::typenum::U25>;
pub type Array3x3<T> = GenericArrayAdapter<T, generic_array::typenum::U9>;

pub struct HardcodedNumbersGenerator {
    numbers: [u32; 6],
    current: usize,
}

impl RandomNumberGenerator for HardcodedNumbersGenerator {
    fn next(&mut self) -> u32 {
        let result = self.numbers[self.current];
        self.current = (self.current + 1) % self.numbers.len();
        result
    }
}

impl Default for HardcodedNumbersGenerator {
    fn default() -> Self {
        HardcodedNumbersGenerator {
            numbers: [4, 2, 4, 3, 4, 4],
            current: 0,
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
        write!(result, "\"{}\"\n", e).unwrap();
    }
    result
}

pub fn board_to_string(board: &Board) -> String {
    let mut result = String::new();
    for y in 0..board.height() {
        write!(result, "\"").unwrap();
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
            ).unwrap();
        }
        write!(result, "\"\n").unwrap();
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
