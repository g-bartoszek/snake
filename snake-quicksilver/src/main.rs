// Draw some multi-colored geometry to the screen
use rand;
use snake::*;

use quicksilver::{
    Result,
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{Settings, State, Window, Event, run},
    input::{Key,ButtonState}
};


struct DrawGeometry(Box<dyn Snake>);

impl State for DrawGeometry {
    fn new() -> Result<DrawGeometry> {

        Ok(DrawGeometry(Box::new(create_game_instance!( 20, 20, RNG ))))
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        self.0.advance();
        Ok(())
    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {

        match event {
            Event::Key(Key::Left, ButtonState::Pressed) => self.0.left(),
            Event::Key(Key::Right, ButtonState::Pressed) => self.0.right(),
            Event::Key(Key::Up, ButtonState::Pressed) => self.0.up(),
            Event::Key(Key::Down, ButtonState::Pressed) => self.0.down(),
            _ => {}
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        for (Location{x, y}, s ) in self.0.board().iter() {
            match s {
                snake::Square::Snake => window.draw(&Rectangle::new(((x * 20) as i32, (y * 20) as i32), (20, 20)), Col(Color::BLUE)),
                snake::Square::Fruit => window.draw(&Rectangle::new(((x * 20) as i32, (y * 20) as i32), (20, 20)), Col(Color::GREEN)),
                snake::Square::Empty => {},
            }
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct RNG {}

impl snake::RandomNumberGenerator for RNG {
    fn next(&mut self) -> u32 {
        rand::random::<u32>()
    }
}

fn main() {
    let mut settings = Settings::default();
    settings.update_rate = 200.0;
    run::<DrawGeometry>("Draw Geometry", Vector::new(800, 600), settings);
}

