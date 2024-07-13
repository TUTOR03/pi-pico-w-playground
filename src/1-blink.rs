#![deny(unsafe_code)]
#![no_std]
#![no_main]

use rp_pico as bsp;

use bsp::entry;
use bsp::hal::{
  clocks::{init_clocks_and_plls, Clock},
  pac,
  sio::Sio,
  watchdog::Watchdog,
};
use embedded_hal::digital::OutputPin;
use panic_halt as _;

const EXTERNAL_FREQ_HZ: u32 = 12_000_000;

#[entry]
fn main() -> ! {
  let mut pac = pac::Peripherals::take().unwrap();
  let core = pac::CorePeripherals::take().unwrap();
  let mut watchdog = Watchdog::new(pac.WATCHDOG);
  let sio = Sio::new(pac.SIO);

  let clocks = init_clocks_and_plls(
    EXTERNAL_FREQ_HZ,
    pac.XOSC,
    pac.CLOCKS,
    pac.PLL_SYS,
    pac.PLL_USB,
    &mut pac.RESETS,
    &mut watchdog,
  )
  .ok()
  .unwrap();

  let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

  let pins = bsp::Pins::new(
    pac.IO_BANK0,
    pac.PADS_BANK0,
    sio.gpio_bank0,
    &mut pac.RESETS,
  );

  let mut led_pin = pins.gpio22.into_push_pull_output();
  loop {
    led_pin.set_high().unwrap();
    delay.delay_ms(1000);
    led_pin.set_low().unwrap();
    delay.delay_ms(1000);
  }
}
