#![no_main]
#![no_std]
// This program assumes the use of an common anode rgb led. Switch high and low for use with a common cathode rgb led.

use knurling_session_20q4 as _; // global logger + panicking-behavior + memory layout

// access to functionality:
use embedded_hal::blocking::delay::DelayMs;

// access to board peripherals:
use nrf52840_hal::{
    self as hal,
    gpio::{p0::Parts as P0Parts, Level, Output, Pin, PushPull},
    pac::TIMER0,
    prelude::*,
    timer::OneShot,
    Timer,
};
pub struct LEDColor {
    r: Pin<Output<PushPull>>,
    g: Pin<Output<PushPull>>,
    b: Pin<Output<PushPull>>,
}

impl LEDColor {

    // static methods:
    // they don't need to be called by an instance.
    // they are used as constructors.
    // they don't have `self` as an argument.

    pub fn init<Mode>(led_red: Pin<Mode>, led_blue: Pin<Mode>, led_green: Pin<Mode>) -> Self {
        LEDColor {
            r: led_red.into_push_pull_output(Level::High),
            b: led_blue.into_push_pull_output(Level::High),
            g: led_green.into_push_pull_output(Level::High),
        }
    }
        
    // instance methods:
    // they are called by an instance.
    // they have a reference `self` as an argument.

    pub fn off(&mut self) {
        self.r.set_high().unwrap();
        self.b.set_high().unwrap();
        self.g.set_high().unwrap();
    }

    pub fn blue(&mut self) {
        self.r.set_high().unwrap();
        self.b.set_low().unwrap();
        self.g.set_high().unwrap();
    }

    pub fn red(&mut self) {
        self.r.set_low().unwrap();
        self.b.set_high().unwrap();
        self.g.set_high().unwrap();
    }

    pub fn green(&mut self) {
        self.r.set_high().unwrap();
        self.b.set_high().unwrap();
        self.g.set_low().unwrap();

    }

    pub fn yellow(&mut self) {
        self.r.set_low().unwrap();
        self.b.set_high().unwrap();
        self.g.set_low().unwrap();
    }

    pub fn pink(&mut self) {
        self.r.set_low().unwrap();
        self.b.set_low().unwrap();
        self.g.set_high().unwrap();
    }

    pub fn light_blue(&mut self) {
        self.r.set_high().unwrap();
        self.b.set_low().unwrap();
        self.g.set_low().unwrap();
    }

    pub fn white(&mut self) {
        self.r.set_low().unwrap();
        self.b.set_low().unwrap();
        self.g.set_low().unwrap();
    }
    // blinks between two colors
    pub fn blinky(&mut self, timer: &mut Timer<TIMER0, OneShot>) {
        self.red();
        timer.delay_ms(1000_u32);
        self.blue();
        timer.delay_ms(1000_u32);
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // take() returns all peripherals, so we can access them
    let board = hal::pac::Peripherals::take().unwrap();
    // first peripheral: initialize timer
    let mut timer = Timer::new(board.TIMER0);
    // second peripheral: access to P0 pins
    let pins = P0Parts::new(board.P0);

    let led_channel_red = pins.p0_03.degrade();
    let led_channel_blue = pins.p0_04.degrade();
    let led_channel_green = pins.p0_28.degrade();

    let mut light: LEDColor = LEDColor::init(led_channel_red, led_channel_blue, led_channel_green);

    loop {
        light.blue();
        timer.delay_ms(1000_u32);

        light.red();
        timer.delay_ms(1000_u32);
    }
}
