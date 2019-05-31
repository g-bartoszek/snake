use core::ops::{Index, IndexMut};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Square {
    Fruit,
    Empty,
    Snake,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum GameStatus {
    InProgress,
    Lost,
    Won,
}

impl Default for Square {
    fn default() -> Self {
        Square::Empty
    }
}

#[derive(PartialEq, Copy, Clone, Debug, Default)]
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
    /// ```rust
    /// use self::snake::Location;
    /// assert_eq!(0, Location{x: 3, y: 3}.wrap(3,3).x);
    /// assert_eq!(0, Location{x: 3, y: 3}.wrap(3,3).y);
    /// assert_eq!(2, Location{x: -1, y: 3}.wrap(3,3).x);
    /// assert_eq!(2, Location{x: -1, y: -1}.wrap(3,3).y);
    /// assert_eq!(1, Location{x: 4, y: 2}.wrap(3,3).x);
    /// assert_eq!(2, Location{x: 4, y: 2}.wrap(3,3).y);
    /// assert_eq!(1, Location{x: -2, y: 2}.wrap(3,3).x);
    /// ```
    pub fn wrap(mut self, max_width: usize, max_height: usize) -> Location {
        if self.x >= max_width as i32 {
            self.x %= max_width as i32;
        }

        if self.x < 0 as i32 {
            self.x = max_width as i32 + (self.x % max_width as i32);
        }

        if self.y >= max_height as i32 {
            self.y %= max_height as i32;
        }

        if self.y < 0 as i32 {
            self.y = max_height as i32 + (self.y % max_height as i32);
        }

        self
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

trait PreallocatedArray<T>: Default + Index<usize, Output=T> + IndexMut<usize, Output=T> {
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
    T: PreallocatedArray<Square>,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: T::default(),
            width,
            height,
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
       self.data[location.y as usize * self.width + location.x as usize]
    }
    fn at_mut(&mut self, location: &Location) -> &mut Square {
        &mut self.data[location.y as usize * self.width + location.x as usize]
    }
}

struct Game<B, S, R>
where
    B: PreallocatedArray<Square>,
    S: PreallocatedArray<Location>,
    R: RandomNumberGenerator,
{
    width: usize,
    height: usize,
    snake: S,
    snake_size: usize,
    direction: Direction,
    fruit: Location,
    status: GameStatus,
    rng: R,
    _pd: std::marker::PhantomData<B>,
}

trait RandomNumberGenerator: Default {
    fn next(&mut self) -> u32;
}

impl<B, S, R> Game<B, S, R>
where
    B: PreallocatedArray<Square>,
    S: PreallocatedArray<Location>,
    R: RandomNumberGenerator,
{
    pub fn new(width: usize, height: usize) -> Game<B, S, R> {
        assert_eq!(S::default().as_slice().len(), width * height);
        assert_eq!(B::default().as_slice().len(), width * height);

        let center_x = (width / 2) as i32;
        let center_y = (height / 2) as i32;

        let mut snake = S::default();
        snake.as_mut_slice()[1] = Location {
            x: center_x,
            y: center_y,
        };
        snake.as_mut_slice()[0] = Location {
            x: center_x - 1,
            y: center_y,
        };

        let mut game = Game {
            width,
            height,
            snake,
            snake_size: 2,
            direction: Direction::Right,
            fruit: Location { x: 0, y: 0 },
            status: GameStatus::InProgress,
            rng: R::default(),
            _pd: std::marker::PhantomData::<B> {},
        };

        game.fruit = game.place_new_fruit().unwrap();

        game
    }

    pub fn board(&self) -> impl Board {
        let mut board = FixedSizeBoard::<B>::new(self.width, self.height);

        match self.status {
            GameStatus::InProgress => {
                *board.at_mut(&self.fruit) = Square::Fruit;

                self.snake
                    .as_slice()[0..self.snake_size]
                    .iter()
                    .for_each(|l| {
                        *board.at_mut(l) = Square::Snake;
                    });
            },
            GameStatus::Won => {
                self.snake
                    .as_slice()
                    .iter()
                    .for_each(|l| {
                        *board.at_mut(l) = Square::Snake;
                    });
            },
            GameStatus::Lost => {}
        }

        board
    }

    pub fn advance(&mut self) -> GameStatus {
        if self.status == GameStatus::InProgress {
            self.status = self.advance_impl()
        }
        self.status
    }

    fn advance_impl(&mut self) -> GameStatus {
        let new_head = self.snake[self.snake_size - 1].move_in(self.direction)
            .wrap(self.width, self.height);
        if self.fruit == new_head

        {
            self.eat_the_fruit();
            if let Some(location) = self.place_new_fruit() {
                self.fruit = location;
                return GameStatus::InProgress;
            } else {
                return GameStatus::Won;
            }
        } else if self.snake.as_mut_slice()[0..self.snake_size].contains(&new_head) {
            return GameStatus::Lost
        }


        for i in 0..self.snake_size - 1 {
            self.snake[i] = self.snake[i+1];
        }

        self.snake[self.snake_size-1] = new_head;
        GameStatus::InProgress
    }

    pub fn up(&mut self) {
        if self.direction != Direction::Down {
            self.direction = Direction::Up;
        }
    }

    pub fn left(&mut self) {
        if self.direction != Direction::Right {
            self.direction = Direction::Left;
        }
    }

    pub fn down(&mut self) {
        if self.direction != Direction::Up {
            self.direction = Direction::Down;
        }
    }

    pub fn right(&mut self) {
        if self.direction != Direction::Left {
            self.direction = Direction::Right;
        }
    }

    fn place_new_fruit(&mut self) -> Option<Location> {
        let snake = &self.snake.as_slice()[0..self.snake_size];

        let fruit = Location {
            x: self.rng.next() as i32,
            y: self.rng.next() as i32,
        }.wrap(self.width, self.height);

        return place_new_fruit(fruit, self.width, self.height, snake);
    }

    fn eat_the_fruit(&mut self) {
        self.snake[self.snake_size] = self.fruit;
        self.snake_size += 1;
    }
}

fn place_new_fruit(
    expected: Location,
    width: usize,
    height: usize,
    taken: &[Location],
) -> Option<Location> {

        for y in 0..height {
            for x in 0..width {
            let l = Location{x: expected.x + x as i32, y: expected.y + y as i32}.wrap(width, height);
            if !taken.contains(&l) {
                return Some(l);
            }
        }
    }

    None
}

#[cfg(test)]
#[rustfmt::skip::macros(board_layout)]
mod tests {

    mod test_utils;

    use super::*;
    use test_utils::*;

    fn create_game() -> Game<Array5x5<Square>, Array5x5<Location>, HardcodedNumbersGenerator> {
        Game::<Array5x5<Square>, Array5x5<Location>, HardcodedNumbersGenerator>::new(WIDTH, HEIGHT)
    }

    #[test]
    fn game_is_initialized() {
        create_game();
    }

    #[test]
    fn at_the_beginning_snake_is_in_the_middle() {
        let game = create_game();

        let expected = board_layout!(
            "     ",
            "     ",
            " OO F",
            "     ",
            "     "
        );

        assert_board!(&game.board(), &expected);
    }

    #[test]
    fn snake_moves_forward() {
        let mut game = create_game();

        game.advance();

        let expected = board_layout!(
            "     ",
            "     ",
            "  OOF",
            "     ",
            "     "
        );

        assert_board!(&game.board(), &expected);
    }

    #[test]
    fn snake_turns_up() {
        let mut game = create_game();

        game.up();
        game.advance();

        let expected = board_layout!(
            "     ",
            "  O  ",
            "  O F",
            "     ",
            "     "
        );

        assert_board!(&game.board(), &expected);
    }

    #[test]
    fn snake_turns_left() {
        let mut game = create_game();

        game.up();
        game.advance();
        game.left();
        game.advance();

        let expected = board_layout!(
            "     ",
            " OO  ",
            "    F",
            "     ",
            "     "
        );

        assert_board!(&game.board(), &expected);
    }

    #[test]
    fn snake_turns_down() {
        let mut game = create_game();

        game.down();
        game.advance();

        let expected = board_layout!(
            "     ",
            "     ",
            "  O F",
            "  O  ",
            "     "
        );

        assert_board!(&game.board(), &expected);
    }

    #[test]
    fn snake_turns_right() {
        let mut game = create_game();

        game.down();
        game.advance();
        game.right();
        game.advance();

        let expected = board_layout!(
            "     ",
            "     ",
            "    F",
            "  OO ",
            "     "
        );

        assert_board!(&game.board(), &expected);
    }

    #[test]
    fn snake_cant_turn_left_when_its_moving_right() {
        let mut game = create_game();

        game.left();
        game.advance();

        let expected = board_layout!(
            "     ",
            "     ",
            "  OOF",
            "     ",
            "     "
        );

        assert_board!(&game.board(), &expected);
    }

    #[test]
    fn snake_cant_turn_right_when_its_moving_left() {
        let mut game = create_game();

        game.up();
        game.advance();
        game.left();
        game.advance();

        assert_board!(
            &game.board(),
            &board_layout!(
            "     ",
            " OO  ",
            "    F",
            "     ",
            "     "
        )
        );

        game.right();
        game.advance();

        assert_board!(
            &game.board(),
            &board_layout!(
            "     ",
            "OO   ",
            "    F",
            "     ",
            "     "
        )
        );
    }

    #[test]
    fn snake_cant_turn_up_when_its_moving_down() {
        let mut game = create_game();

        game.down();
        game.advance();

        assert_board!(
            &game.board(),
            &board_layout!(
            "     ",
            "     ",
            "  O F",
            "  O  ",
            "     "
        )
        );

        game.up();
        game.advance();

        assert_board!(
            &game.board(),
            &board_layout!(
            "     ",
            "     ",
            "    F",
            "  O  ",
            "  O  "
        )
        );
    }

    #[test]
    fn snake_cant_turn_down_when_its_moving_up() {
        let mut game = create_game();

        game.up();
        game.advance();

        assert_board!(
            &game.board(),
            &board_layout!(
            "     ",
            "  O  ",
            "  O F",
            "     ",
            "     "
        )
        );

        game.down();
        game.advance();

        assert_board!(
            &game.board(),
            &board_layout!(
            "  O  ",
            "  O  ",
            "    F",
            "     ",
            "     "
        )
        );
    }

    #[test]
    fn when_snake_eats_fruit_it_grows_and_new_fruit_is_placed() {
        let mut game = create_game();

        game.advance();
        game.advance();

        let expected = board_layout!(
            "     ",
            "     ",
            "  OOO",
            "    F",
            "     "
        );

        assert_board!(&game.board(), &expected);
    }

    #[test]
    fn when_snake_eats_another_fruit_it_grows_even_more() {
        let mut game = create_game();

        game.advance();
        game.advance();

        assert_board!(
            &game.board(),
            &board_layout!(
            "     ",
            "     ",
            "  OOO",
            "    F",
            "     "
        )
        );

        game.down();
        game.advance();

        assert_board!(
            &game.board(),
            &board_layout!(
            "     ",
            "     ",
            "  OOO",
            "    O",
            "    F"
        )
        );
    }

    #[test]
    fn when_place_for_fruit_is_taken_first_empty_square_is_used() {
        let mut game =
            Game::<Array3x3<Square>, Array3x3<Location>, HardcodedNumbersGenerator>::new(3, 3);

        assert_board!(
            &game.board(),
            &board_layout!(
            "   ",
            "OO ",
            " F "
        )
        );

        game.down();
        game.advance();

        assert_board!(
            &game.board(),
            &board_layout!(
            " F ",
            "OO ",
            " O "
        )
        );

        game.down();
        game.advance();

        assert_board!(
            &game.board(),
            &board_layout!(
            " O ",
            "OOF",
            " O "
        )
        );
    }

    #[test]
    fn when_snake_bites_itself_the_game_is_lost() {
        let mut game =
            Game::<Array3x3<Square>, Array3x3<Location>, HardcodedNumbersGenerator>::new(3, 3);

        assert_board!(
            &game.board(),
            &board_layout!(
            "   ",
            "OO ",
            " F "
        )
        );

        game.down();
        game.advance();

        assert_board!(
            &game.board(),
            &board_layout!(
            " F ",
            "OO ",
            " O "
        )
        );

        game.down();
        assert_eq!(GameStatus::InProgress, game.advance());
        assert_eq!(GameStatus::Lost, game.advance());

        assert_board!(
            &game.board(),
            &board_layout!(
            "   ",
            "   ",
            "   "
        )
        );
    }

    #[test]
    fn when_there_is_no_place_for_new_fruit_the_game_is_won() {
        let mut game =
            Game::<Array3x3<Square>, Array3x3<Location>, HardcodedNumbersGenerator>::new(3, 3);

        assert_board!(
            &game.board(),
            &board_layout!(
            "   ",
            "OO ",
            " F "
        )
        );

        game.down();
        game.advance();
        game.advance();
        game.right();
        game.advance();
        game.down();
        game.advance();
        game.advance();
        game.right();
        game.advance();
        game.down();
        game.advance();
        game.advance();
        game.right();
        game.advance();
        game.down();
        assert_eq!(GameStatus::Won, game.advance());
        assert_eq!(GameStatus::Won, game.advance());

        assert_board!(
            &game.board(),
            &board_layout!(
            "OOO",
            "OOO",
            "OOO"
        )
        );


    }

    #[test]
    fn place_new_fruit_takes_first_free_location() {
        let expected_location = Location { x: 0, y: 0 };
        let taken_locations = [Location { x: 0, y: 0 }];

        assert_eq!(
            Some(Location { x: 1, y: 0 }),
            place_new_fruit(expected_location, 2, 2, &taken_locations)
        );

        let taken_locations = [Location { x: 0, y: 0 }, Location { x: 1, y: 0 }];

        assert_eq!(
            Some(Location { x: 0, y: 1 }),
            place_new_fruit(expected_location, 2, 2, &taken_locations)
        );

        let expected_location = Location { x: 1, y: 0 };
        let taken_locations = [Location { x: 1, y: 0 }];

        assert_eq!(
            Some(Location { x: 0, y: 0 }),
            place_new_fruit(expected_location, 2, 2, &taken_locations)
        );

        let expected_location = Location { x: 4, y: 2 };
        let taken_locations = [];

        assert_eq!(
            Some(Location { x: 1, y: 2 }),
            place_new_fruit(expected_location, 3, 3, &taken_locations)
        );
    }

}
