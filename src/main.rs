#![deny(unsafe_code)]
#![no_main]
#![no_std]

//! Initialization code

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
use stm32f4xx_hal::{delay::Delay, stm32, prelude::*};


#[entry]
fn main() -> ! {

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let gpiob = dp.GPIOB.split(/*&mut rcc.apb2*/);
    let mut led = gpiob.pb7.into_push_pull_output(/*&mut gpiob.mode*/);

    // let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(48.mhz()).freeze(/*&mut flash.acr*/);

    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        led.set_high().ok();
        delay.delay_ms(1_000_u16);
        led.set_low().ok();
        delay.delay_ms(1_000_u16);
    }
}
