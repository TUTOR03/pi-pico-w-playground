#![deny(unsafe_code)]
#![no_std]
#![no_main]

use rp_pico as bsp;

use panic_halt as _;

use bsp::entry;
use bsp::hal::{pac, sio::Sio};
use embedded_hal::digital::{InputPin, OutputPin};

#[entry]
fn main() -> ! {
  let mut pac = pac::Peripherals::take().unwrap();
  let sio = Sio::new(pac.SIO);

  let pins = rp_pico::Pins::new(
    pac.IO_BANK0,
    pac.PADS_BANK0,
    sio.gpio_bank0,
    &mut pac.RESETS,
  );

  let mut led_pin = pins.gpio15.into_push_pull_output();
  let mut button_pin = pins.gpio9.into_pull_up_input();
  loop {
    if button_pin.is_high().unwrap() {
      led_pin.set_high().unwrap();
    } else {
      led_pin.set_low().unwrap();
    }
  }
}
