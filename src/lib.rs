const WIDTH: usize = 10;
const HEIGHT: usize = 10;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Square {
    Snake,
    Fruit,
    Empty,
}

#[derive(PartialEq, Copy, Clone, Debug)]
struct Location
{
    x: i32,
    y: i32
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum SnakeData {
    Snake(Location),
    NoSnake,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
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
        snake[1] = SnakeData::Snake(Location{ x: center_x, y: center_y });
        snake[0] = SnakeData::Snake(Location{ x: center_x - 1, y: center_y });

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
        for i in 0..(self.snake.len() - 1) {
            let next = &self.snake[i + 1];
            let current = &self.snake[i];

            match (current, next) {
                (SnakeData::Snake(location), SnakeData::NoSnake) => {
                    println!("Moving head");
                    match self.direction {
                        Direction::Up => {
                            self.snake[i] = SnakeData::Snake(Location{ x: location.x, y: location.y - 1});
                        }
                        Direction::Down => {}
                        Direction::Right => {
                            self.snake[i] = SnakeData::Snake(Location{ x: location.x + 1, y: location.y});
                        }
                        Direction::Left => {}
                    }
                }
                (SnakeData::Snake(_), SnakeData::Snake(_)) => {
                    println!("Moving tail");
                    self.snake[i] = *next;
                }
                (_, _) => {}
            }
        }
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

    #[test]
    fn game_is_initialized() {
        let game = Game::<FixedSizeBoard>::new(WIDTH, HEIGHT);
    }

    #[test]
    fn at_the_beginning_snake_is_in_the_middle() {
        let game = Game::<FixedSizeBoard>::new(WIDTH, HEIGHT);

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
        let mut game = Game::<FixedSizeBoard>::new(WIDTH, HEIGHT);

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
        let mut game = Game::<FixedSizeBoard>::new(WIDTH, HEIGHT);

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

                        if board.at(&Location{x: x as i32, y: y as i32}) != expected {
                            Err(format!(
                                "X:{} Y:{} should be {:?} but is {:?}",
                                x,
                                y,
                                expected,
                                board.at(&Location{x: x as i32, y: y as i32})
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
