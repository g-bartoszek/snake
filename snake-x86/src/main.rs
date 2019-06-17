use rand;
use snake::*;

use cursive::traits::*;
use cursive::views::{Canvas, OnEventView};
use cursive::Cursive;

use std::thread;

#[derive(Default)]
pub struct RNG {}

impl snake::RandomNumberGenerator for RNG {
    fn next(&mut self) -> u32 {
        rand::random::<u32>()
    }
}

fn main() {
    let game = std::sync::Arc::new(std::sync::Mutex::new(snake::create_game_instance!(
        20, 20, RNG
    )));
    let mut siv = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback(cursive::event::Key::Left, {
        let g = game.clone();
        move |_| g.lock().unwrap().left()
    });
    siv.add_global_callback(cursive::event::Key::Up, {
        let g = game.clone();
        move |_| g.lock().unwrap().up()
    });
    siv.add_global_callback(cursive::event::Key::Down, {
        let g = game.clone();
        move |_| g.lock().unwrap().down()
    });
    siv.add_global_callback(cursive::event::Key::Right, {
        let g = game.clone();
        move |_| g.lock().unwrap().right()
    });

    siv.add_layer(OnEventView::new(
        Canvas::new(())
            .with_draw({
                let g = game.clone();
                move |_, p| {
                    let mut game = g.lock().unwrap();
                    let board = game.board();
                    for y in 0..board.height() {
                        for x in 0..board.width() {
                            p.print(
                                (x, y),
                                match board.at(snake::Location::new(x, y)) {
                                    snake::Square::Snake => "O",
                                    snake::Square::Fruit => "F",
                                    snake::Square::Empty => " ",
                                },
                            );
                        }
                    }
                }
            })
            .fixed_size((20, 20)),
    ));

    thread::spawn({
        let g = game.clone();
        move || loop {
            g.lock().unwrap().advance();
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    });

    siv.set_fps(60);

    siv.run();
}
