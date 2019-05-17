const WIDTH: usize = 10;
const HEIGHT: usize = 10;


#[derive(PartialEq, Copy, Clone, Debug)]
enum Square {
    Snake,
    Fruit,
    Empty
}

trait Board: Default {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn at(&mut self, x: usize, y: usize) -> &mut Square;
}

struct FixedSizeBoard
{
    data : [Square; WIDTH * HEIGHT]
}

impl Default for FixedSizeBoard {
    fn default() -> Self {
        Self{data: [Square::Empty; WIDTH * HEIGHT]}
    }
}

impl Board for FixedSizeBoard {
    fn width(&self) -> usize { WIDTH }
    fn height(&self) -> usize { HEIGHT }
    fn at(&mut self, x: usize, y: usize) -> &mut Square {
        dbg!(x);
        dbg!(y);
        &mut self.data[dbg!(y*self.width() + x)]
    }
}

struct Game<B> where B: Board {
    width: usize,
    height: usize,
    _pd: std::marker::PhantomData<B>
}

impl<B> Game<B> where B: Board {
    pub fn new(width: usize, height: usize) -> Game<B> {
        Game{width, height, _pd: std::marker::PhantomData::<B>{}}
    }

    pub fn board(&self) -> impl Board {
        let mut board = B::default();

        let center_x = self.width / 2;
        let center_y = self.height / 2;

        *board.at(center_x, center_y) = Square::Snake;
        *board.at(center_x - 1, center_y) = Square::Snake;

        board
    }

    pub fn advance(&self) {

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_is_initialized() {
        let game = Game::<FixedSizeBoard>::new(WIDTH, HEIGHT);
    }

    #[test]
    fn at_the_beginning_snake_is_in_the_middle() {
        let game = Game::<FixedSizeBoard>::new(WIDTH, HEIGHT);

        let center_x = WIDTH / 2;
        let center_y = HEIGHT / 2;

        let mut board = game.board();

        assert_eq!(Square::Snake, *board.at(center_x, center_y));
        assert_eq!(Square::Snake, *board.at(center_x+1, center_y));
    }

    #[test]
    fn snakes_moves_forward() {
        let game = Game::<FixedSizeBoard>::new(WIDTH, HEIGHT);

        let center_x = WIDTH / 2;
        let center_y = HEIGHT / 2;

        game.advance();

        let mut board = game.board();

        assert_eq!(Square::Empty, *board.at(center_x+0, center_y));
        assert_eq!(Square::Snake, *board.at(center_x+1, center_y));
        assert_eq!(Square::Snake, *board.at(center_x+2, center_y));
    }
}
