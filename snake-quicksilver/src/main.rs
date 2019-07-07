// Draw some multi-colored geometry to the screen
use rand;
use snake::*;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color},
    input::{ButtonState, Key},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};

struct SnakeQuicksilver(Box<dyn Snake>);

impl State for SnakeQuicksilver {
    fn new() -> Result<SnakeQuicksilver> {
        Ok(SnakeQuicksilver(Box::new(create_game_instance!(
            20, 20, RNG
        ))))
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        self.0.advance();
        Ok(())
    }

    fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        let direction = match event {
            Event::Key(Key::Left, ButtonState::Pressed) => Some(Direction::Left),
            Event::Key(Key::Right, ButtonState::Pressed) => Some(Direction::Right),
            Event::Key(Key::Up, ButtonState::Pressed) => Some(Direction::Up),
            Event::Key(Key::Down, ButtonState::Pressed) => Some(Direction::Down),
            _ => None,
        };

        if let Some(d) = direction {
            self.0.set_direction(d);
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        for (Location { x, y }, s) in self.0.board().iter() {
            match s {
                snake::Square::Snake => window.draw(
                    &Rectangle::new(((x * 20) as i32, (y * 20) as i32), (20, 20)),
                    Col(Color::BLUE),
                ),
                snake::Square::Fruit => window.draw(
                    &Rectangle::new(((x * 20) as i32, (y * 20) as i32), (20, 20)),
                    Col(Color::GREEN),
                ),
                snake::Square::Empty => {}
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
    run::<SnakeQuicksilver>("Snake Quicksilver", Vector::new(800, 600), settings);
}
