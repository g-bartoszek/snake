#![cfg_attr(not(feature = "std"), no_std)]

use core::convert::TryFrom;
use core::ops::DerefMut;

pub use generic_array;
pub use paste;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub trait Board {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn at(&self, location: Location) -> Square;
    fn at_mut(&mut self, location: &Location) -> &mut Square;
    fn iter(&self) -> BoardIterator;
}

pub trait Snake {
    fn board(&mut self) -> &dyn Board;
    fn advance(&mut self) -> GameStatus;
    fn set_direction(&mut self, direction: Direction);
}

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

impl Location {
    pub fn new<T>(x: T, y: T) -> Location
        where
            i32: TryFrom<T>,
    {
        Location {
            x: i32::try_from(x).ok().unwrap(),
            y: i32::try_from(y).ok().unwrap(),
        }
    }

    ///
    /// ```
    /// use self::snake::{Location, Direction};
    /// let mut l = Location::new(10, 10);
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
            Direction::Up => Location::new(self.x, self.y - 1),
            Direction::Down => Location::new(self.x, self.y + 1),
            Direction::Left => Location::new(self.x - 1, self.y),
            Direction::Right => Location::new(self.x + 1, self.y),
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


pub trait FixedSizedArray<T>: Default + DerefMut<Target = [T]> {}
impl<T, A> FixedSizedArray<T> for A where A: Default + DerefMut<Target = [T]> {}

pub struct FixedSizeBoard<T>
where
    T: FixedSizedArray<Square>,
{
    data: T,
    width: usize,
    height: usize,
}

impl<T> FixedSizeBoard<T>
where
    T: FixedSizedArray<Square>,
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
    T: FixedSizedArray<Square>,
{
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn at(&self, location: Location) -> Square {
        self.data[location.y as usize * self.width + location.x as usize]
    }
    fn at_mut(&mut self, location: &Location) -> &mut Square {
        &mut self.data[location.y as usize * self.width + location.x as usize]
    }
    fn iter<'a>(&'a self) -> BoardIterator<'a> {
        BoardIterator::<'a> {
            board: self,
            location: Location::new(0, 0)
        }
    }
}

pub struct BoardIterator<'a> {
    board: &'a dyn Board,
    location: Location
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = (Location, Square);
    fn next(&mut self) -> Option<Self::Item> {
        if self.location.y == self.board.height() as i32 {
            return None;
        }

        let result = (self.location, self.board.at(self.location));

        self.location.x += 1;

        if self.location.x == self.board.width() as i32 {
            self.location.x = 0;
            self.location.y += 1;
        }

        Some(result)
    }
}

pub struct Game<B, S, R>
where
    B: FixedSizedArray<Square>,
    S: FixedSizedArray<Location>,
    R: RandomNumberGenerator,
{
    width: usize,
    height: usize,
    snake: S,
    snake_size: usize,
    current_direction: Direction,
    next_direction: Direction,
    fruit: Location,
    status: GameStatus,
    rng: R,
    board: FixedSizeBoard<B>,
}

pub trait RandomNumberGenerator: Default {
    fn next(&mut self) -> u32;
}

impl<B, S, R> Game<B, S, R>
where
    B: FixedSizedArray<Square>,
    S: FixedSizedArray<Location>,
    R: RandomNumberGenerator,
{
    pub fn new(width: usize, height: usize) -> Game<B, S, R> {
        assert_eq!(S::default().len(), width * height);
        assert_eq!(B::default().len(), width * height);

        let center_x = (width / 2) as i32;
        let center_y = (height / 2) as i32;

        let mut snake = S::default();
        snake[1] = Location::new(center_x, center_y);
        snake[0] = Location::new(center_x - 1, center_y);

        let mut game = Game {
            width,
            height,
            snake,
            snake_size: 2,
            current_direction: Direction::Right,
            next_direction: Direction::Right,
            fruit: Location::new(0, 0),
            status: GameStatus::InProgress,
            rng: R::default(),
            board: FixedSizeBoard::<B>::new(width, height),
        };

        game.fruit = game.place_new_fruit().unwrap();

        game
    }

    fn place_new_fruit(&mut self) -> Option<Location> {
        let fruit = Location::new(self.rng.next() as i32, self.rng.next() as i32).wrap(self.width, self.height);

        place_new_fruit(fruit, self.width, self.height, self.snake())
    }

    fn eat_the_fruit(&mut self) {
        self.snake[self.snake_size] = self.fruit;
        self.snake_size += 1;
    }

    fn snake(&self) -> &[Location] {
        &self.snake[0..self.snake_size]
    }

    fn snake_mut(&mut self) -> &mut [Location] {
        &mut self.snake[0..self.snake_size]
    }

    fn move_snake_and_get_status(&mut self) -> GameStatus {
        self.change_direction();

        match self.calcualte_new_head_location() {
            new_location if self.fruit == new_location => {
                self.eat_the_fruit();

                match self.place_new_fruit() {
                    Some(location) => {
                        self.fruit = location;
                        GameStatus::InProgress
                    }
                    None => GameStatus::Won,
                }
            }
            new_location if self.snake().contains(&new_location) => GameStatus::Lost,
            new_location => {
                self.move_snake_in_current_direction(new_location);
                GameStatus::InProgress
            }
        }
    }

    fn calcualte_new_head_location(&self) -> Location {
        self.snake()
            .last()
            .unwrap()
            .move_in(self.current_direction)
            .wrap(self.width, self.height)
    }

    fn move_snake_in_current_direction(&mut self, new_head: Location) {
        for i in 0..self.snake_size - 1 {
            self.snake[i] = self.snake[i + 1];
        }

        *self.snake_mut().last_mut().unwrap() = new_head;
    }

    fn change_direction(&mut self) {
        if match (self.next_direction, self.current_direction) {
            (Direction::Left, Direction::Right) => false,
            (Direction::Right, Direction::Left) => false,
            (Direction::Up, Direction::Down) => false,
            (Direction::Down, Direction::Up) => false,
            (_, _) => true,
        } {
            self.current_direction = self.next_direction;
        }
    }
}

impl<B, S, R> Snake for Game<B, S, R>
where
    B: FixedSizedArray<Square>,
    S: FixedSizedArray<Location>,
    R: RandomNumberGenerator,
{
    fn board(&mut self) -> &dyn Board {
        let mut board = FixedSizeBoard::<B>::new(self.width, self.height);

        match self.status {
            GameStatus::InProgress => {
                *board.at_mut(&self.fruit) = Square::Fruit;

                self.snake().iter().for_each(|l| {
                    *board.at_mut(l) = Square::Snake;
                });
            }
            GameStatus::Won => {
                self.snake.iter().for_each(|l| {
                    *board.at_mut(l) = Square::Snake;
                });
            }
            GameStatus::Lost => {}
        }

        self.board = board;
        &self.board
    }

    fn advance(&mut self) -> GameStatus {
        if self.status == GameStatus::InProgress {
            self.status = self.move_snake_and_get_status()
        }
        self.status
    }

    fn set_direction(&mut self, direction: Direction) {
        self.next_direction = direction;
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
            let l = Location::new(expected.x + x as i32, expected.y + y as i32).wrap(width, height);
            if !taken.contains(&l) {
                return Some(l);
            }
        }
    }

    None
}

#[macro_export]
macro_rules! create_game_instance {
    ($width:expr, $height:expr, $rng:ty) => {
        paste::expr! {
            type Width = generic_array::typenum::[<U $width>];
            type Height = generic_array::typenum::[<U $height>];
            type Array<T> = generic_array::GenericArray<T, <Width as core::ops::Mul<Height>>::Output>;

            Game::<Array<Square>, Array<Location>, $rng>::new($width, $height)
        }
    };
}

#[cfg(feature = "std")]
#[cfg(test)]
#[rustfmt::skip::macros(board_layout)]
mod tests {

    mod test_utils;

    use super::*;
    use test_utils::*;

    fn create_game() -> impl Snake {
        create_game_instance!(5, 5, HardcodedNumbersGenerator)
    }

    #[test]
    fn game_is_initialized() {
        create_game_instance!(10, 10, HardcodedNumbersGenerator);
    }

    #[test]
    fn at_the_beginning_snake_is_in_the_middle() {
        let mut game = create_game();

        let expected = board_layout!(
            "     ",
            "     ",
            " OO F",
            "     ",
            "     "
        );

        assert_board!(game.board(), &expected);
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

        assert_board!(game.board(), &expected);
    }

    #[test]
    fn snake_turns_up() {
        let mut game = create_game();

        game.set_direction(Direction::Up);
        game.advance();

        let expected = board_layout!(
            "     ",
            "  O  ",
            "  O F",
            "     ",
            "     "
        );

        assert_board!(game.board(), &expected);
    }

    #[test]
    fn snake_turns_left() {
        let mut game = create_game();

        game.set_direction(Direction::Up);
        game.advance();
        game.set_direction(Direction::Left);
        game.advance();

        let expected = board_layout!(
            "     ",
            " OO  ",
            "    F",
            "     ",
            "     "
        );

        assert_board!(game.board(), &expected);
    }

    #[test]
    fn snake_turns_down() {
        let mut game = create_game();

        game.set_direction(Direction::Down);
        game.advance();

        let expected = board_layout!(
            "     ",
            "     ",
            "  O F",
            "  O  ",
            "     "
        );

        assert_board!(game.board(), &expected);
    }

    #[test]
    fn snake_turns_right() {
        let mut game = create_game();

        game.set_direction(Direction::Down);
        game.advance();
        game.set_direction(Direction::Right);
        game.advance();

        let expected = board_layout!(
            "     ",
            "     ",
            "    F",
            "  OO ",
            "     "
        );

        assert_board!(game.board(), &expected);
    }

    #[test]
    fn snake_cant_turn_left_when_its_moving_right() {
        let mut game = create_game();

        game.set_direction(Direction::Left);
        game.advance();

        let expected = board_layout!(
            "     ",
            "     ",
            "  OOF",
            "     ",
            "     "
        );

        assert_board!(game.board(), &expected);
    }

    #[test]
    fn snake_cant_turn_right_when_its_moving_left() {
        let mut game = create_game();

        game.set_direction(Direction::Up);
        game.advance();
        game.set_direction(Direction::Left);
        game.advance();

        assert_board!(
            game.board(),
            &board_layout!(
            "     ",
            " OO  ",
            "    F",
            "     ",
            "     "
        )
        );

        game.set_direction(Direction::Right);
        game.advance();

        assert_board!(
            game.board(),
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

        game.set_direction(Direction::Down);
        game.advance();

        assert_board!(
            game.board(),
            &board_layout!(
            "     ",
            "     ",
            "  O F",
            "  O  ",
            "     "
        )
        );

        game.set_direction(Direction::Up);
        game.advance();

        assert_board!(
            game.board(),
            &board_layout!(
            "     ",
            "     ",
            "    F",
            "  O  ",
            "  O  "
        )
        );

        game.set_direction(Direction::Left);
        game.set_direction(Direction::Up);
        assert_eq!(GameStatus::InProgress, game.advance());
    }

    #[test]
    fn snake_cant_turn_down_when_its_moving_up() {
        let mut game = create_game();

        game.set_direction(Direction::Up);
        game.advance();

        assert_board!(
            game.board(),
            &board_layout!(
            "     ",
            "  O  ",
            "  O F",
            "     ",
            "     "
        )
        );

        game.set_direction(Direction::Down);
        game.advance();

        assert_board!(
            game.board(),
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

        assert_board!(game.board(), &expected);
    }

    #[test]
    fn when_snake_eats_another_fruit_it_grows_even_more() {
        let mut game = create_game();

        game.advance();
        game.advance();

        assert_board!(
            game.board(),
            &board_layout!(
            "     ",
            "     ",
            "  OOO",
            "    F",
            "     "
        )
        );

        game.set_direction(Direction::Down);
        game.advance();

        assert_board!(
            game.board(),
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
        let mut game = create_game_instance!(3, 3, HardcodedNumbersGenerator);

        assert_board!(
            game.board(),
            &board_layout!(
            "   ",
            "OO ",
            " F "
        )
        );

        game.set_direction(Direction::Down);
        game.advance();

        assert_board!(
            game.board(),
            &board_layout!(
            " F ",
            "OO ",
            " O "
        )
        );

        game.set_direction(Direction::Down);
        game.advance();

        assert_board!(
            game.board(),
            &board_layout!(
            " O ",
            "OOF",
            " O "
        )
        );
    }

    #[test]
    fn when_snake_bites_itself_the_game_is_lost() {
        let mut game = create_game_instance!(3, 3, HardcodedNumbersGenerator);

        assert_board!(
            game.board(),
            &board_layout!(
            "   ",
            "OO ",
            " F "
        )
        );

        game.set_direction(Direction::Down);
        game.advance();

        assert_board!(
            game.board(),
            &board_layout!(
            " F ",
            "OO ",
            " O "
        )
        );

        game.set_direction(Direction::Down);
        assert_eq!(GameStatus::InProgress, game.advance());
        assert_eq!(GameStatus::Lost, game.advance());
        assert_eq!(GameStatus::Lost, game.advance());

        assert_board!(
            game.board(),
            &board_layout!(
            "   ",
            "   ",
            "   "
        )
        );
    }

    #[test]
    fn when_there_is_no_place_for_new_fruit_the_game_is_won() {
        let mut game = create_game_instance!(3, 3, HardcodedNumbersGenerator);

        assert_board!(
            game.board(),
            &board_layout!(
            "   ",
            "OO ",
            " F "
        )
        );

        game.set_direction(Direction::Down);
        game.advance();
        game.advance();
        game.set_direction(Direction::Right);
        game.advance();
        game.set_direction(Direction::Down);
        game.advance();
        game.advance();
        game.set_direction(Direction::Right);
        game.advance();
        game.set_direction(Direction::Down);
        game.advance();
        game.advance();
        game.set_direction(Direction::Right);
        game.advance();
        game.set_direction(Direction::Down);
        assert_eq!(GameStatus::Won, game.advance());
        assert_eq!(GameStatus::Won, game.advance());

        assert_board!(
            game.board(),
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
