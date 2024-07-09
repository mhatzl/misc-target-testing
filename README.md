# misc-target-testing

Rust example project to run integration and unit tests on host (Windows, Linux, MacOS),
and on embedded devices (Cortex-M bare metal).

The built-in test harness is used to run tests on the host,
while [qemu](https://github.com/mhatzl/embedded-runner) with [defmt-test](https://crates.io/crates/defmt-test) is used to run tests on simulated embedded devices.



