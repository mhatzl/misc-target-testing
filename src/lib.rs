#![cfg_attr(feature = "xmc-target", no_main)]
#![cfg_attr(feature = "xmc-target", no_std)]

pub mod unit_test_1;
pub mod unit_test_2;

#[cfg(feature = "xmc-target")]
use defmt_rtt as _;
#[cfg(feature = "xmc-target")]
use panic_probe as _;
#[cfg(feature = "xmc-target")]
use xmc4_hal as _;

#[cfg(feature = "xmc-target")]
#[cfg(target_os = "none")]
#[defmt::panic_handler]
fn panic() -> ! {
    // cortex_m::asm::udf()
    panic_probe::hard_fault()
}

#[cfg(feature = "xmc-target")]
pub fn exit() -> ! {
    loop {
        cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_SUCCESS);
    }
}

#[cfg(feature = "xmc-target")]
#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        cortex_m_semihosting::debug::exit(cortex_m_semihosting::debug::EXIT_FAILURE);
    }
}
