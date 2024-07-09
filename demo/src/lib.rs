#![cfg_attr(feature = "use-target", no_main)]
#![cfg_attr(feature = "use-target", no_std)]

pub mod unit_test_1;
pub mod unit_test_2;

#[cfg(feature = "use-target")]
use lm3s6965 as _;
#[cfg(feature = "use-target")]
use panic_probe as _;

#[cfg(feature = "use-target")]
#[cfg(target_os = "none")]
#[defmt::panic_handler]
fn panic() -> ! {
    panic_probe::hard_fault()
}

#[cfg(feature = "use-target")]
pub fn exit() -> ! {
    loop {
        cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_SUCCESS);
    }
}

#[cfg(feature = "use-target")]
#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_FAILURE);
    }
}

// ----------------------- DEFMT GLOBAL LOGGER FOR QEMU -------------------------
// taken from: https://github.com/knurling-rs/defmt/blob/main/firmware/defmt-semihosting/src/lib.rs

#[cfg(feature = "use-target")]
use core::sync::atomic::{AtomicBool, Ordering};
#[cfg(feature = "use-target")]
use cortex_m::{interrupt, register};
#[cfg(feature = "use-target")]
use cortex_m_semihosting::hio;

#[cfg(feature = "use-target")]
#[defmt::global_logger]
struct Logger;

#[cfg(feature = "use-target")]
static TAKEN: AtomicBool = AtomicBool::new(false);
#[cfg(feature = "use-target")]
static INTERRUPTS_ACTIVE: AtomicBool = AtomicBool::new(false);
#[cfg(feature = "use-target")]
static mut ENCODER: defmt::Encoder = defmt::Encoder::new();

#[cfg(feature = "use-target")]
unsafe impl defmt::Logger for Logger {
    fn acquire() {
        let primask = register::primask::read();
        interrupt::disable();

        if TAKEN.load(Ordering::Relaxed) {
            panic!("defmt logger taken reentrantly")
        }

        // no need for CAS because interrupts are disabled
        TAKEN.store(true, Ordering::Relaxed);

        INTERRUPTS_ACTIVE.store(primask.is_active(), Ordering::Relaxed);

        // safety: accessing the `static mut` is OK because we have disabled interrupts.
        unsafe { ENCODER.start_frame(do_write) }
    }

    unsafe fn flush() {
        // Do nothing.
        //
        // semihosting is fundamentally blocking, and does not have I/O buffers the target can control.
        // After write returns, the host has the data, so there's nothing left to flush.
    }

    unsafe fn release() {
        // safety: accessing the `static mut` is OK because we have disabled interrupts.
        ENCODER.end_frame(do_write);

        TAKEN.store(false, Ordering::Relaxed);
        if INTERRUPTS_ACTIVE.load(Ordering::Relaxed) {
            // re-enable interrupts
            interrupt::enable()
        }
    }

    unsafe fn write(bytes: &[u8]) {
        // safety: accessing the `static mut` is OK because we have disabled interrupts.
        ENCODER.write(bytes, do_write);
    }
}

#[cfg(feature = "use-target")]
fn do_write(bytes: &[u8]) {
    // using QEMU; it shouldn't mind us opening several handles (I hope)
    if let Ok(mut hstdout) = hio::hstdout() {
        hstdout.write_all(bytes).ok();
    }
}
