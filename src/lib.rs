const WIDTH: usize = 10;
const HEIGHT: usize = 10;


#[derive(PartialEq, Copy, Clone, Debug)]
enum Square {
    Snake,
    Fruit,
    Empty,
}

trait Board: Default {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn at_mut(&mut self, x: usize, y: usize) -> &mut Square;
    fn at(&self, x: usize, y: usize) -> Square;
}

struct FixedSizeBoard
{
    data: [Square; WIDTH * HEIGHT]
}

impl Default for FixedSizeBoard {
    fn default() -> Self {
        Self { data: [Square::Empty; WIDTH * HEIGHT] }
    }
}

impl Board for FixedSizeBoard {
    fn width(&self) -> usize { WIDTH }
    fn height(&self) -> usize { HEIGHT }
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

struct Game<B> where B: Board {
    width: usize,
    height: usize,
    _pd: std::marker::PhantomData<B>,
}

impl<B> Game<B> where B: Board {
    pub fn new(width: usize, height: usize) -> Game<B> {
        Game { width, height, _pd: std::marker::PhantomData::<B> {} }
    }

    pub fn board(&self) -> impl Board {
        let mut board = B::default();

        let center_x = dbg!(self.width / 2);
        let center_y = dbg!(self.height / 2);

        *board.at_mut(center_x, center_y) = Square::Snake;
        *board.at_mut(center_x - 1, center_y) = Square::Snake;

        board
    }

    pub fn advance(&self) {}
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    fn check_board(board: &impl Board, expected: &Vec<String>) -> Vec<String> {
        assert_eq!(board.height(), expected.len(), "Invalid height");

        expected.iter().enumerate().map(|(y, row)| -> Vec<String> {
            assert_eq!(board.width(), row.chars().count(), "Invalid width");

            row.chars().enumerate().map(|(x, square)| {
                let expected = match square {
                    '◯' => Square::Snake,
                    'F' => Square::Fruit,
                    _ => Square::Empty,
                };

                if board.at(x, y) != expected {
                    Err(format!("X:{} Y:{} should be {:?} but is {:?}", x, y, expected, board.at(x, y)))
                } else {
                    Ok(())
                }
            }).filter_map(Result::err).collect()
        }).flatten().collect()
    }

    #[test]
    fn game_is_initialized() {
        let game = Game::<FixedSizeBoard>::new(WIDTH, HEIGHT);
    }

    #[test]
    fn at_the_beginning_snake_is_in_the_middle() {
        let game = Game::<FixedSizeBoard>::new(WIDTH, HEIGHT);

        let mut board = game.board();

        let expected = vec![
            String::from("          "),
            String::from("          "),
            String::from("          "),
            String::from("          "),
            String::from("          "),
            String::from("    ◯◯    "),
            String::from("          "),
            String::from("          "),
            String::from("          "),
            String::from("          "),
        ];

        assert_eq!(Vec::<String>::new(), check_board(&board, &expected));
    }

    #[test]
    fn snakes_moves_forward() {
        let game = Game::<FixedSizeBoard>::new(WIDTH, HEIGHT);

        let center_x = WIDTH / 2;
        let center_y = HEIGHT / 2;

        game.advance();

        let mut board = game.board();

        let expected = vec![
            String::from("          "),
            String::from("          "),
            String::from("          "),
            String::from("          "),
            String::from("          "),
            String::from("     ◯◯   "),
            String::from("          "),
            String::from("          "),
            String::from("          "),
            String::from("          "),
        ];

        assert_eq!(Vec::<String>::new(), check_board(&board, &expected));
    }
}
