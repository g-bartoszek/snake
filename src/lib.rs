const WIDTH: usize = 10;
const HEIGHT: usize = 10;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Square {
    Snake,
    Fruit,
    Empty,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum SnakeData {
    Snake(i32, i32),
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
            SnakeData::Snake(_, _) => true,
            SnakeData::NoSnake => false,
        }
    }
}

trait Board: Default {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn at_mut(&mut self, x: usize, y: usize) -> &mut Square;
    fn at(&self, x: usize, y: usize) -> Square;
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
    fn at_mut(&mut self, x: usize, y: usize) -> &mut Square {
        x;
        y;
        &mut self.data[y * self.width() + x]
    }
    fn at(&self, x: usize, y: usize) -> Square {
        x;
        y;
        self.data[y * self.width() + x]
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
        snake[1] = SnakeData::Snake(center_x, center_y);
        snake[0] = SnakeData::Snake(center_x - 1, center_y);

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
                if let SnakeData::Snake(x, y) = s {
                    *board.at_mut(*x as usize, *y as usize) = Square::Snake;
                }
            });

        for s in &self.snake {}

        board
    }

    pub fn advance(&mut self) {
        for i in 0..(self.snake.len() - 1) {
            let next = &self.snake[i + 1];
            let current = &self.snake[i];

            match (current, next) {
                (SnakeData::Snake(x, y), SnakeData::NoSnake) => {
                    println!("Moving head");
                    match self.direction {
                        Direction::Up => {
                            self.snake[i] = SnakeData::Snake(*x, *y - 1);
                        }
                        Direction::Down => {}
                        Direction::Right => {
                            self.snake[i] = SnakeData::Snake(*x + 1, *y);
                        }
                        Direction::Left => {}
                    }
                }
                (SnakeData::Snake(_, _), SnakeData::Snake(_, _)) => {
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

                        if board.at(x, y) != expected {
                            Err(format!(
                                "X:{} Y:{} should be {:?} but is {:?}",
                                x,
                                y,
                                expected,
                                board.at(x, y)
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
