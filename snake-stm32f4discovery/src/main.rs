#![no_std]
#![no_main]

extern crate embedded_hal;
extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

//use cortex_m_semihosting::hprintln;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{Mode, Phase, Polarity};
use embedded_hal::timer::CountDown;

use ssd1331::interface::DisplayInterface;
use ssd1331::prelude::*;
use ssd1331::Builder;

use stm32f4xx_hal::adc::{config::AdcConfig, config::*, Adc};
use stm32f4xx_hal::gpio::*;
use stm32f4xx_hal::rcc::RccExt;
use stm32f4xx_hal::stm32::interrupt;
use stm32f4xx_hal::stm32::TIM2;
use stm32f4xx_hal::time::U32Ext;
use stm32f4xx_hal::timer;
use stm32f4xx_hal::{delay::Delay, spi};

use core::cell::RefCell;

use snake::*;

mod display;
mod joystick;
mod simple_rng;

use joystick::Joystick;
use simple_rng::SimpleRNG;

type Width = generic_array::typenum::U22;
type Height = generic_array::typenum::U16;
type Array<T> = generic_array::GenericArray<T, <Width as core::ops::Mul<Height>>::Output>;
type SnakeType = Game<Array<Square>, Array<Location>, SimpleRNG>;

type PC0Analog = gpioc::PC0<Analog>;
type PC1Analog = gpioc::PC1<Analog>;

static MUTEX_TIM2: Mutex<RefCell<Option<timer::Timer<TIM2>>>> = Mutex::new(RefCell::new(None));
static MUTEX_GAME: Mutex<RefCell<Option<SnakeType>>> = Mutex::new(RefCell::new(None));
static MUTEX_JOY: Mutex<RefCell<Option<joystick::AdcJoystick<PC0Analog, PC1Analog>>>> =
    Mutex::new(RefCell::new(None));

#[interrupt]
fn TIM2() {
    free(|cs| {
        MUTEX_TIM2 .borrow(cs) .borrow_mut() .as_mut() .unwrap() .wait() .ok();
        let direction = MUTEX_JOY.borrow(cs).borrow_mut().as_mut().unwrap().read();
        if let Some(d) = match direction {
            joystick::Direction::Left => Some(snake::Direction::Left),
            joystick::Direction::Right => Some(snake::Direction::Right),
            joystick::Direction::Up => Some(snake::Direction::Up),
            joystick::Direction::Down => Some(snake::Direction::Down),
            joystick::Direction::Center => None,
        } {
            MUTEX_GAME
                .borrow(cs)
                .borrow_mut()
                .as_mut()
                .unwrap()
                .set_direction(d);
        }
    });
}

pub fn init() -> (Delay, GraphicsMode<impl DisplayInterface>) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f4xx_hal::stm32::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze();

    // DELAY
    let mut delay = Delay::new(cp.SYST, clocks);

    // DISPLAY
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();

    let sck = gpioa.pa5.into_alternate_af5();
    let miso = gpioa.pa6.into_alternate_af5();
    let mosi = gpioa.pa7.into_alternate_af5();

    let mut rst = gpiob.pb0.into_push_pull_output();
    let dc = gpiob.pb1.into_push_pull_output();

    let spi = spi::Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        8_u32.mhz().into(),
        clocks,
    );

    let mut disp: GraphicsMode<_> = Builder::new().connect_spi(spi, dc).into();

    disp.reset(&mut rst, &mut delay);
    disp.init().unwrap();
    disp.flush().unwrap();

    // JOYSTICK
    let adc_config = AdcConfig::default();

    adc_config
        .clock(Clock::Pclk2_div_8)
        .resolution(Resolution::Twelve)
        .align(Align::Right)
        .continuous(Continuous::Single);

    let adc = Adc::adc1(dp.ADC1, true, adc_config);

    let gpioc = dp.GPIOC.split();
    let pc0 = gpioc.pc0.into_analog();
    let pc1 = gpioc.pc1.into_analog();

    let joystick = joystick::AdcJoystick {
        adc,
        x: pc0,
        y: pc1,
    };

    // TIMER INTERRUPT
    let mut nvic = cp.NVIC;
    nvic.enable(stm32f4xx_hal::interrupt::TIM2);

    let timer = timer::Timer::<TIM2>::tim2(dp.TIM2, 20_u32.hz(), clocks);
    free(|cs| {
        MUTEX_JOY.borrow(cs).replace(Some(joystick));
        MUTEX_TIM2.borrow(cs).replace(Some(timer));
        MUTEX_TIM2
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .listen(timer::Event::TimeOut);
    });

    (delay, disp)
}

pub fn init_game(display: &mut GraphicsMode<impl DisplayInterface>, delay: &mut Delay) {
    display::draw_rust_logo(display);
    delay.delay_ms(3000_u16);
    free(|cs| {
        MUTEX_GAME
            .borrow(cs)
            .replace(Some(create_game_instance!(22, 16, SimpleRNG)));
    });
}

#[entry]
fn main() -> ! {
    let (mut delay, mut display) = init();

    init_game(&mut display, &mut delay);

    loop {
        let status = free(|cs| {
            let mut game = MUTEX_GAME.borrow(cs).borrow_mut();
            let status = game.as_mut().unwrap().advance();
            let board = game.as_mut().unwrap().board();
            display.clear();
            display::draw_board(&mut display, board);
            status
        });

        if status == GameStatus::Lost {
            init_game(&mut display, &mut delay);
        }

        delay.delay_ms(300_u16);
    }
}
