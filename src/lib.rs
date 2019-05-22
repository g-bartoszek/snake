#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Square {
    Fruit,
    Empty,
    Snake,
}

impl Default for Square {
    fn default() -> Self {
        Square::Empty
    }
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

impl Default for SnakeData {
    fn default() -> Self {
        SnakeData::NoSnake
    }
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

trait PreallocatedArray<T>: Default {
    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&mut self) -> &mut [T];
}


pub trait Board {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn at(&self, location: &Location) -> Square;
    fn at_mut(&mut self, location: &Location) -> &mut Square;
}

struct FixedSizeBoard<T>
where
    T: PreallocatedArray<Square>,
{
    data: T,
    width: usize,
    height: usize,
}

impl<T> FixedSizeBoard<T>
    where
        T: PreallocatedArray<Square> {
   pub fn new(width: usize, height: usize) -> Self {
       Self {
           data: T::default(),
           width,
           height
       }
   }
}

impl<T> Board for FixedSizeBoard<T>
where
    T: PreallocatedArray<Square>,
{
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn at(&self, location: &Location) -> Square {
        self.data.as_slice()[location.y as usize * self.width + location.x as usize]
    }
    fn at_mut(&mut self, location: &Location) -> &mut Square {
        &mut self.data.as_mut_slice()[location.y as usize * self.width + location.x as usize]
    }
}

struct Game<B, S>
where
    B: PreallocatedArray<Square>,
    S: PreallocatedArray<SnakeData>,
{
    width: usize,
    height: usize,
    snake: S,
    direction: Direction,
    _pd: std::marker::PhantomData<B>,
}

impl<B, S> Game<B, S>
where
    B: PreallocatedArray<Square>,
    S: PreallocatedArray<SnakeData>,
{
    pub fn new(width: usize, height: usize) -> Game<B, S> {
        let center_x = (width / 2) as i32;
        let center_y = (height / 2) as i32;

        let mut snake = S::default();
        snake.as_mut_slice()[1] = SnakeData::Snake(Location {
            x: center_x,
            y: center_y,
        });
        snake.as_mut_slice()[0] = SnakeData::Snake(Location {
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
        let mut board = FixedSizeBoard::<B>::new(self.width, self.height);

        self.snake
            .as_slice()
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
        let mut new_snake = S::default();
        let update = self.snake.as_slice().windows(2).map(|w| -> SnakeData {
            match w {
                [SnakeData::Snake(location), SnakeData::NoSnake] => {
                    SnakeData::Snake(location.move_in(self.direction))
                }
                [current, next] => next.clone(),
                _ => SnakeData::NoSnake,
            }
        });

        new_snake
            .as_mut_slice()
            .iter_mut()
            .zip(update)
            .for_each(|(new, update)| {
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
    mod test_utils;

    use super::*;
    use test_utils::*;



    fn create_game(
    ) -> Game<CurrentWidthAndHeightArray<Square>, CurrentWidthAndHeightArray<SnakeData>> {
        Game::<CurrentWidthAndHeightArray<Square>, CurrentWidthAndHeightArray<SnakeData>>::new(
            WIDTH, HEIGHT,
        )
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


}
