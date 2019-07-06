use stm32f4xx_hal::adc::{config::SampleTime, Adc};
use stm32f4xx_hal::stm32::ADC1;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Center,
}

pub struct AdcJoystick<PINX, PINY> {
    pub adc: Adc<ADC1>,
    pub x: PINX,
    pub y: PINY,
}

pub trait Joystick {
    fn read(&mut self) -> Direction;
}

impl<PINX, PINY> Joystick for AdcJoystick<PINX, PINY>
where
    PINX: embedded_hal::adc::Channel<ADC1, ID = u8>,
    PINY: embedded_hal::adc::Channel<ADC1, ID = u8>,
{
    fn read(&mut self) -> Direction {
        let sample_x = self.adc.convert(&self.x, SampleTime::Cycles_480);
        let x = self.adc.sample_to_millivolts(sample_x);

        let sample_y = self.adc.convert(&self.y, SampleTime::Cycles_480);
        let y = self.adc.sample_to_millivolts(sample_y);

        if x < 1000 {
            return Direction::Left;
        }

        if x > 2000 {
            return Direction::Right;
        }

        if y < 1000 {
            return Direction::Down;
        }

        if y > 2000 {
            return Direction::Up;
        }

        Direction::Center
    }
}
