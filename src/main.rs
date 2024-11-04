#![no_std]
#![no_main]

#[macro_use]
mod pinout;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use embassy_executor::Spawner;
use embassy_nrf::{config::HfclkSource, gpio::{self, AnyPin, Pin as _, Pull}, interrupt::{self, InterruptExt}, Peripheral as _};
use nrf52833_pac::{p0::pin_cnf::PIN_CNF_SPEC, Peripherals, P0};
use nrf52833_pac as pac;
use defmt::*;

use {defmt_rtt as _, panic_probe as _};

fn print_reset_reasons() {
    const REASONS: [&str; 9] = [
        "Bit 0 - Reset from pin-reset detected",
        "Bit 1 - Reset from watchdog detected",
        "Bit 2 - Reset from soft reset detected",
        "Bit 3 - Reset from CPU lock-up detected",
        "Bit 16 - Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO",
        "Bit 17 - Reset due to wake up from System OFF mode when wakeup is triggered from ANADETECT signal from LPCOMP",
        "Bit 18 - Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode",
        "Bit 19 - Reset due to wake up from System OFF mode by NFC field detect",
        "Bit 20 - Reset due to wake up from System OFF mode by VBUS rising into valid range",
    ];
    let resetreas = unsafe { &*pac::POWER::ptr() }.resetreas.read().bits();
    for (bit, reason) in [0, 1, 2, 3, 16, 17, 18, 19, 20].into_iter().zip(REASONS) {
        if resetreas & (1 << bit) != 0 {
            info!("{}", reason);
        }
    }
    unsafe {
        // Clear the RESETREAS register
        (*pac::POWER::ptr()).resetreas.write(|w| w.bits(resetreas));
    }
}

#[embassy_executor::main]
async fn fake_main(spawner: Spawner) {
    main().await;

    let peripherals = Peripherals::take().unwrap();
    peripherals.POWER.systemoff.write(|w| w.systemoff().enter());
    loop {}
}

async fn main() {
    let mut config = embassy_nrf::config::Config::default();
    config.hfclk_source = HfclkSource::ExternalXtal;
    let p = embassy_nrf::init(config);

    print_reset_reasons();

    // ir led init
    let mut ir_led0 = gpio::Output::new(pinout!(p.ir_led0), embassy_nrf::gpio::Level::Low, gpio::OutputDrive::Standard);
    let mut ir_led1 = gpio::Output::new(pinout!(p.ir_led1), embassy_nrf::gpio::Level::Low, gpio::OutputDrive::Standard);

    // set gpio defaults
    let __ = gpio::Output::new(pinout!(p.ir_iset1), embassy_nrf::gpio::Level::High, gpio::OutputDrive::Standard);
    let __ = gpio::Output::new(pinout!(p.ir_iset0), embassy_nrf::gpio::Level::Low, gpio::OutputDrive::Standard);
    let __ = gpio::Output::new(pinout!(p.drivev), embassy_nrf::gpio::Level::High, gpio::OutputDrive::Standard);

    ir_led0.set_high();
    ir_led1.set_high();

    power_button_loop(pinout!(p.pwr_btn).into()).await;
}

async fn power_button_loop(pin: AnyPin) {
    let mut pwr_btn = pin.into_ref();

    {
        let mut pwr_btn = gpio::Input::new(pwr_btn.reborrow(), Pull::Up);
        pwr_btn.wait_for_falling_edge().await;
        pwr_btn.wait_for_rising_edge().await;
    }

    {
        let sense_when_goes_to = gpio::Level::Low;

        pin_cnf(pwr_btn).write(|w| {
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
}

fn pin_cnf(pin: embassy_nrf::PeripheralRef<AnyPin>) -> &'static nrf52833_pac::generic::Reg<PIN_CNF_SPEC> {
    let idx = pin.pin() as usize;
    match pin.port() {
        gpio::Port::Port0 => unsafe { &(*nrf52833_pac::P0::ptr()).pin_cnf[idx] },
        gpio::Port::Port1 => unsafe { &(*nrf52833_pac::P1::ptr()).pin_cnf[idx] },
    }
}
