#![no_std]
#![no_main]

#[macro_use]
mod pinout;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use embassy_executor::Spawner;
use embassy_nrf::{config::HfclkSource, gpio::{self, AnyPin, Pull}};
use nrf52833_pac::{Peripherals, P0};
use defmt::*;

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = embassy_nrf::config::Config::default();
    config.hfclk_source = HfclkSource::ExternalXtal;
    let p = embassy_nrf::init(config);

    info!("power on");

    spawner.must_spawn(power_button_loop(pinout!(p.pwr_btn).into()));
}

#[embassy_executor::task]
async fn power_button_loop(pin: AnyPin) {
    {
        let mut pwr_btn = gpio::Input::new(pin, Pull::Up);
        pwr_btn.wait_for_falling_edge().await;
    }

    {
        let sense_when_goes_to = gpio::Level::Low;

        // P0.11
        unsafe { &(*P0::ptr()).pin_cnf[11] }.write(|w| {
            w.dir().input();
            w.input().connect();
            w.pull().pullup();
            w.drive().s0s1();
        
            match sense_when_goes_to {
                gpio::Level::Low => w.sense().low(),
                gpio::Level::High => w.sense().high(),
            };
        
            w
        });
    }

    let core = nrf52833_hal::pac::CorePeripherals::take().unwrap();
    let mut delay = nrf52833_hal::Delay::new(core.SYST);
    delay.delay_ms(500u16);

    info!("power off");

    let peripherals = Peripherals::take().unwrap();
    peripherals.POWER.systemoff.write(|w| w.systemoff().enter());

    loop {}
}
