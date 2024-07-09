#![cfg_attr(feature = "use-target", no_main)]
#![cfg_attr(feature = "use-target", no_std)]

use misc_target_testing as _;

#[cfg(feature = "use-target")]
#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("Hello world!");

    misc_target_testing::exit()
}

#[cfg(not(feature = "use-target"))]
fn main() {
    println!("Hello world!");
}
