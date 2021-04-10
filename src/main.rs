//! examples/init.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hprintln};
use panic_semihosting as _;
use feather_m4::prelude::_atsamd_hal_embedded_hal_digital_v2_OutputPin;

#[rtic::app(device = atsamd51j, peripherals = true)]
const APP: () = {
    #[init]
    fn init(cx: init::Context) {
        static mut X: u32 = 0;

        // Cortex-M peripherals
        let _core: cortex_m::Peripherals = cx.core;

        // Device specific peripherals
        let device: atsamd51j::Peripherals = cx.device;

        // Safe access to local `static mut` variable
        let _x: &'static mut u32 = X;

        let mut pins = feather_m4::Pins::new(device.PORT);
        let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
        red_led.set_high().unwrap();

        hprintln!("init").unwrap();

        debug::exit(debug::EXIT_SUCCESS);
    }
};
