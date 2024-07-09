# misc-target-testing

Rust example project to run integration and unit tests on host (Windows, Linux, macOS),
and on embedded devices (Cortex-M bare metal).
This project is used to showcase current limitations and trade-offs
for projects trying to run tests on host and target platforms.

The built-in test harness is used to run tests on the host,
while [qemu](https://www.qemu.org/download/) with [defmt-test](https://crates.io/crates/defmt-test) is used to run tests on simulated embedded devices.

## Usage

**Note:** `qemu-system-arm` command must be available. 

1. Build the *Qemu runner* using `cargo build -p runner`

2. Optional: On Unix systems, remove `.exe` for the runner binary in `.cargo/config.toml`

3. Run target integration tests:
   - `cargo t -p misc-target-testing --test test_1 --target thumbv7m-none-eabi --features target-unit-test-1,test`
   - `cargo t -p misc-target-testing --test test_2 --target thumbv7m-none-eabi --features target-unit-test-2`

4. Disable `harness` for `lib` in `demo/Cargo.toml`

   **Note:** Needed to run unit tests on the target, because the standard harness requires `std`.

5. Run target unit tests:
   - `cargo t -p misc-target-testing --lib --target thumbv7m-none-eabi --features target-unit-test-1`
   - `cargo t -p misc-target-testing --lib --target thumbv7m-none-eabi --features target-unit-test-2`

6. Run host integration tests:
   - `cargo t -p misc-target-testing --test integration --target x86_64-pc-windows-msvc --features test`

7. Enable `harness` for `lib` in `demo/Cargo.toml`

   **Note:** Needed to run unit tests on the host with the default runner.

8. Run host unit tests:
   - `cargo t -p misc-target-testing --lib --target x86_64-pc-windows-msvc`

## Current limitations and trade-offs

- **Only one `defmt-test::tests` module can be active in the src folder**

  This is mostly because every `defmt-test::tests` module internally creates a `main` function used to execute tests.
  Try running unit tests on the target when both features `target-unit-test-1` and `target-unit-test-2` are set.

  `cargo t -p misc-target-testing --lib --target thumbv7m-none-eabi --features target-unit-test-1,target-unit-test-2`

  As far as I know, there is currently no workaround to resolve this limitation.
  
  This limitation gets even worse, because non-inline modules are unstable inside proc-macros
  (see https://github.com/rust-lang/rust/issues/54727).
  Meaning that all tests must be located inside one `defmt-test::tests` module,
  or feature flags must be used to selectively enable test modules (as shown in this demo).

  Both options are not really feasible for larger projects.

- **Turning integration tests into unit tests**

  Due to the limitations for embedded unit tests,
  a reasonable workaround is to move all tests into the `tests` folder,
  which turns all tests into integration tests.
  The problem with this change is that private members of the crate cannot be accessed inside tests anymore.
  A *not so nice* workaround is to create wrappers for all private members required in tests,
  and put those wrappers behind a `test` feature flag.
  This approach is used for `test_2`.

- **Enabling/Disabling the test harness for *lib***

  The default test harness uses `std` features and can therefore not be used for `no_std` targets.
  However, setting `harness = false` for `lib` will result in an error when trying to run tests on the host,
  because the custom runner is only usable for the embedded target.

  Because `Cargo.toml` settings cannot be customized depending on the chosen target,
  the harness must be enabled/disabled manually.

- **Integration tests structure workaround**

  Every integration test file for an embedded target must be manually added in `Cargo.toml`.
  This is needed to disable the default test harness.

  While this is a bit cumbersome, it allows to integrate a test file as a submodule for integration tests on the host, and use it as standalone integration test for the embedded target.
  To achieve this, all embedded test files must be placed in subfolders inside the `tests` folder,
  because `cargo` only considers root Rust files in the `tests` folder when looking for available integration tests.

  This workaround is quite ok, because it allows to be more selective about what tests to run on the target.
  However, it may get cumbersome to manually add 100+ test files to `Cargo.toml`.
