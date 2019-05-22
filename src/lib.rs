const WIDTH: usize = 10;
const HEIGHT: usize = 10;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Square {
    Snake,
    Fruit,
    Empty,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

impl Location {
    ///
    /// ```
    /// use self::snake::{Location, Direction};
    /// let mut l = Location{x: 10, y: 10};
    ///
    /// l = l.move_in(Direction::Up);
    /// assert_eq!(10, l.x);
    /// assert_eq!(9, l.y);
    ///
    /// l = l.move_in(Direction::Down);
    /// assert_eq!(10, l.x);
    /// assert_eq!(10, l.y);
    ///
    /// l = l.move_in(Direction::Right);
    /// assert_eq!(11, l.x);
    /// assert_eq!(10, l.y);
    ///
    /// l = l.move_in(Direction::Left);
    /// assert_eq!(10, l.x);
    /// assert_eq!(10, l.y);
    ///
    /// ```
    ///
    pub fn move_in(self, direction: Direction) -> Location {
        match direction {
            Direction::Up => Location {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Location {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Location {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Location {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum SnakeData {
    Snake(Location),
    NoSnake,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl SnakeData {
    pub fn is_snake(&self) -> bool {
        match self {
            SnakeData::Snake(_) => true,
            SnakeData::NoSnake => false,
        }
    }
}

trait Board: Default {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn at(&self, location: &Location) -> Square;
    fn at_mut(&mut self, location: &Location) -> &mut Square;
}

struct FixedSizeBoard {
    data: [Square; WIDTH * HEIGHT],
}

impl Default for FixedSizeBoard {
    fn default() -> Self {
        Self {
            data: [Square::Empty; WIDTH * HEIGHT],
        }
    }
}

impl Board for FixedSizeBoard {
    fn width(&self) -> usize {
        WIDTH
    }
    fn height(&self) -> usize {
        HEIGHT
    }
    fn at(&self, location: &Location) -> Square {
        self.data[location.y as usize * self.width() + location.x as usize]
    }
    fn at_mut(&mut self, location: &Location) -> &mut Square {
        &mut self.data[location.y as usize * self.width() + location.x as usize]
    }
}

struct Game<B>
where
    B: Board,
{
    width: usize,
    height: usize,
    snake: [SnakeData; 10],
    direction: Direction,
    _pd: std::marker::PhantomData<B>,
}

impl<B> Game<B>
where
    B: Board,
{
    pub fn new(width: usize, height: usize) -> Game<B> {
        let center_x = (width / 2) as i32;
        let center_y = (height / 2) as i32;

        let mut snake = [SnakeData::NoSnake; 10];
        snake[1] = SnakeData::Snake(Location {
            x: center_x,
            y: center_y,
        });
        snake[0] = SnakeData::Snake(Location {
            x: center_x - 1,
            y: center_y,
        });

        Game {
            width,
            height,
            snake,
            direction: Direction::Right,
            _pd: std::marker::PhantomData::<B> {},
        }
    }

    pub fn board(&self) -> impl Board {
        let mut board = B::default();

        self.snake
            .iter()
            .take_while(|s| s.is_snake())
            .for_each(|s| {
                if let SnakeData::Snake(location) = s {
                    *board.at_mut(location) = Square::Snake;
                }
            });

        board
    }

    pub fn advance(&mut self) {
        let mut new_snake = [SnakeData::NoSnake; 10];
        let update = self.snake.windows(2).map(|w| -> SnakeData {
            match w {
                [SnakeData::Snake(location), SnakeData::NoSnake] => {
                    SnakeData::Snake(location.move_in(self.direction))
                }
                [current, next] => next.clone(),
                _ => SnakeData::NoSnake,
            }
        });

        new_snake.iter_mut().zip(update).for_each(|(new, update)| {
            *new = update;
        });

        self.snake = new_snake;
    }

    pub fn up(&mut self) {
        self.direction = Direction::Up;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn create_game() -> Game<FixedSizeBoard> {
        Game::<FixedSizeBoard>::new(WIDTH, HEIGHT)
    }

    #[test]
    fn game_is_initialized() {
        create_game();
    }

    #[test]
    fn at_the_beginning_snake_is_in_the_middle() {
        let game = create_game();

        let mut board = game.board();

        let expected = board_layout!(
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "    ◯◯    ",
            "          ",
            "          ",
            "          ",
            "          "
        );

        assert_eq!(Vec::<String>::new(), check_board(&board, &expected));
    }

    #[test]
    fn snakes_moves_forward() {
        let mut game = create_game();

        game.advance();

        let mut board = game.board();

        let expected = board_layout!(
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "     ◯◯   ",
            "          ",
            "          ",
            "          ",
            "          "
        );

        assert_eq!(Vec::<String>::new(), check_board(&board, &expected));
    }

    #[test]
    fn snakes_turns_up() {
        let mut game = create_game();

        game.up();
        game.advance();

        let mut board = game.board();

        let expected = board_layout!(
            "          ",
            "          ",
            "          ",
            "          ",
            "     ◯    ",
            "     ◯    ",
            "          ",
            "          ",
            "          ",
            "          "
        );

        assert_eq!(Vec::<String>::new(), check_board(&board, &expected));
    }

    fn check_board(board: &impl Board, expected: &Vec<String>) -> Vec<String> {
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

}
