#![no_std]
#![no_main]

use cortex_m_rt::entry;
use lpc55_hal as hal;
use panic_halt as _; // Halts the CPU if a panic occurs

#[entry]
fn main() -> ! {
    // 1. Take ownership of all device peripherals (The Rust "Singleton" pattern)
    // This ensures no other part of the OS can accidentally re-initialize hardware.
    let dp = hal::Peripherals::take().unwrap();

    // 2. Initialize the System Controller (SYSCON) 
    // This is where you manage clocks, resets, and power.
    let mut syscon = hal::Syscon::from(dp.SYSCON);
    
    // 3. Initialize the I/O Controller (IOCON)
    // Used for pin multiplexing.
    let mut iocon = hal::Iocon::from(dp.IOCON);
    
    // 4. Configure the System Clock (FRO - Free Running Oscillator)
    // By default, the LPC55S69 boots into a 12MHz FRO. 
    // Here we can scale it to 96MHz or use an external crystal.
    let mut gpio = hal::Gpio::from(dp.GPIO).enabled(&mut syscon);
    
    // 5. Example: Setup the built-in LED (PIO1_6)
    // This proves your "Kernel" is alive and the clock is ticking.
    let pins = hal::Pins::take().unwrap();
    let mut red_led = pins
        .pio1_6
        .into_gpio_pin(&mut iocon, &mut gpio)
        .into_output(hal::alias::Level::High); // Start with LED OFF

    // --- KERNEL INITIALIZATION ZONE ---
    // This is where you will later initialize your SLOS-M components:
    // - slos_memory_init();
    // - slos_scheduler_init();
    // - slos_core1_boot(); // Waking up the second M33 core
    
    loop {
        // Toggle LED to show the idle loop is running
        red_led.toggle();
        
        // A simple delay for now (replaces a proper OS sleep)
        cortex_m::asm::delay(12_000_000); 
    }
}