use crate::*;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 10;

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
                        '◯' => Square::Snake,
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
