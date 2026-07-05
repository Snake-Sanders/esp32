#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use core::cell::RefCell;
use critical_section::Mutex;
use defmt::{error, info};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{AnyPin, Event, Input, InputConfig, Io, Pull};
use esp_hal::{gpio, handler, main};
use esp_println as _;

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}

#[handler]
fn handler() {
    critical_section::with(|cs| {
        let binding = &BUTTON;
        let mut cell = binding.borrow_ref_mut(cs);
        let button = cell.as_mut().expect("Button not settup");

        if button.is_interrupt_set() {
            button.clear_interrupt();
            info!("Pressed");
        };
    });
}

static BUTTON: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    let delay = Delay::new();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let mut io = Io::new(peripherals.IO_MUX);
    let mut button = config_button(peripherals.GPIO0.into());

    io.set_interrupt_handler(handler);

    critical_section::with(|cs| {
        button.listen(Event::FallingEdge);
        BUTTON.borrow_ref_mut(cs).replace(button)
    });
    info!("Button configured with interrupt");

    loop {
        delay.delay_millis(500);
    }
}

fn config_button(gpio: AnyPin<'_>) -> gpio::Input<'_> {
    let config = InputConfig::default().with_pull(Pull::Up);
    Input::new(gpio, config)
}
