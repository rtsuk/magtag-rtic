`cargo check`

~~~
/Users/rob/.cargo/bin/cargo check --manifest-path /Users/rob/projects/nhl/magtag-rtic/Cargo.toml
    Checking magtag-rtic v0.1.0 (/Users/rob/projects/nhl/magtag-rtic)
error[E0599]: no method named `set_high` found for struct `feather_m4::gpio::Pin>` in the current scope

  --> src/main.rs:24:17

   |
24 |         red_led.set_high().unwrap();
   |                 ^^^^^^^^ method not found in `feather_m4::gpio::Pin>`
   |
   = help: items from traits can only be used if the trait is in scope
   = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
           `use feather_m4::prelude::_atsamd_hal_embedded_hal_digital_v2_OutputPin;`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
error: could not compile `magtag-rtic`

To learn more, run the command again with --verbose.
~~~