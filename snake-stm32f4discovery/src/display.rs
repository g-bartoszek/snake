use embedded_graphics::image::Image16BPP;
use embedded_graphics::prelude::*;
use snake::*;
use ssd1331::interface::DisplayInterface;
use ssd1331::prelude::*;

pub fn draw_rust_logo(disp: &mut GraphicsMode<impl DisplayInterface>) {
    let im = Image16BPP::new(include_bytes!("../rust_mini.raw"), 64, 64)
        .translate(Coord::new((96 - 64) / 2, 0));

    disp.draw(im.into_iter());
    disp.flush().unwrap();
}

pub fn draw_square(
    disp: &mut GraphicsMode<impl DisplayInterface>,
    size: usize,
    x: usize,
    y: usize,
    color: u16,
) {
    for py in 0..size {
        for px in 0..size {
            disp.set_pixel(((x * size) + px) as u32, ((y * size) + py) as u32, color);
        }
    }
}

pub fn draw_board(disp: &mut GraphicsMode<impl DisplayInterface>, board: &dyn Board) {
    const SIZE: usize = 4;

    for (Location{x,y}, square) in board.iter() {
        match square {
            Square::Snake => {
                draw_square(disp, SIZE, x as usize, y as usize, 31);
            }
            Square::Fruit => {
                draw_square(disp, SIZE, x as usize, y as usize, 2016);
            }
            Square::Empty => {}
        }
    }

    disp.flush().unwrap();
}
