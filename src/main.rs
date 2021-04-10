//! examples/init.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _;
use feather_m4::prelude::_atsamd_hal_embedded_hal_digital_v2_OutputPin;

#[rtic::app(device = feather_m4::pac, peripherals = true)]
mod app {
    use cortex_m_semihosting::hprintln;

    #[init]
    fn init(cx: init::Context) -> (init::LateResources, init::Monotonics) {
        hprintln!("init").unwrap();

        let _core = cx.core;
        let device = cx.device;
        let mut pins = feather_m4::Pins::new(device.PORT);
        let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
        red_led.set_high().unwrap();

        (init::LateResources {}, init::Monotonics())
    }
}
