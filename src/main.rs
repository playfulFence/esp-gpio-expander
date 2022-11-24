#![no_std]
#![no_main]

#[cfg(feature="esp32")]
use esp32_hal as hal;
#[cfg(feature="esp32s2")]
use esp32s2_hal as hal;
#[cfg(feature="esp32s3")]
use esp32s3_hal as hal;
#[cfg(feature="esp32c3")]
use esp32c3_hal as hal;

use hal::{
    clock::ClockControl,
    pac::Peripherals,
    gpio_types::*,
    i2c::I2C,
    gpio::*,
    prelude::*,
    spi::{Spi, SpiMode},
    timer::TimerGroup,
    Rtc,
    IO,
    Delay,
};

use mcp230xx::*;
use embedded_hal;
use embedded_hal::blocking::i2c::{Write, WriteRead};

use esp_println::println;
use esp_backtrace as _;


pub enum Event {
    Pressed,
    Released,
    Nothing,
}
pub struct ExtendedButton
{
    pin: mcp230xx::Mcp23017,
    pressed: bool, 
}

impl ExtendedButton{
    pub fn new(pin: mcp230xx::Mcp23017) -> Self
    {
        ExtendedButton{
            pin,
            pressed: true,
        }
    }

    pub fn button_pressed<I2C, E>(&mut self, extend : &mut Mcp230xx<I2C, Mcp23017>) -> bool
    where I2C: WriteRead<Error=E> + Write<Error = E>
    {
        match extend.gpio(self.pin) {
            Ok(level) => {
                if level == mcp230xx::Level::High {true}
                else {false}
            }
            Err(lvl) => {println!("Error occured (poll_pin func)"); return false;}
            _ => {println!("Unknown state"); return false;}
        }
    }

    pub fn check<I2C, E>(&mut self, extend : &mut Mcp230xx<I2C, Mcp23017>)
    where I2C: WriteRead<Error=E> + Write<Error = E>
    {
        self.pressed = self.button_pressed(extend);
    }

    pub fn poll_pin<I2C, E>(&mut self, extend : &mut Mcp230xx<I2C, Mcp23017>, delay :&mut Delay) -> Event 
    where I2C: WriteRead<Error=E> + Write<Error = E>
    {
        let pressed_now = self.button_pressed(extend);
    
        if !self.pressed && pressed_now
        {
            delay.delay_ms(30 as u32);
            self.check(extend);
            if self.button_pressed(extend) {
                Event::Pressed
            }
            else {
                Event::Nothing
            }
        }
        else if self.pressed && !pressed_now{
            delay.delay_ms(30 as u32);
            self.check(extend);
            if self.button_pressed(extend)
            {
                Event::Released
            }
            else {
                Event::Nothing
            }
        }
        else{
            Event::Nothing
        }
    }
}

#[cfg(feature="xtensa-lx-rt")]
use xtensa_lx_rt::entry;
#[cfg(feature="riscv-rt")]
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

     #[cfg(any(feature = "esp32"))]
    let mut system = peripherals.DPORT.split();
    #[cfg(any(feature = "esp32s2", feature = "esp32s3", feature = "esp32c3"))]
    let mut system = peripherals.SYSTEM.split();

    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt = timer_group0.wdt;
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);

    // Disable watchdog timer
    wdt.disable();
    rtc.rwdt.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut delay = Delay::new(&clocks);
    
    #[cfg(any(feature = "esp32s3", feature = "esp32c3"))]
    let mut i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio1,
        io.pins.gpio2,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    ).unwrap();

    #[cfg(feature="esp32s2")]
    let mut i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio35,
        io.pins.gpio36,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    ).unwrap();

    #[cfg(feature="esp32")]
    let mut i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio32,
        io.pins.gpio33,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    ).unwrap();


    let mut extend = Mcp230xx::new_default(i2c).unwrap();
    extend.set_direction(Mcp23017::A1, Direction::Input).unwrap();
    extend.set_pull_up(Mcp23017::A1, mcp230xx::PullUp::Enabled);

    let mut button = ExtendedButton::new(Mcp23017::A1);

    let mut cnt = 0;
    
    loop {
        if let Event::Pressed = button.poll_pin(&mut extend, &mut delay)
        {   println!("Pressed({})",cnt); cnt += 1;  }
    }
}
