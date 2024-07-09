#![cfg_attr(feature = "use-target", no_main)]
#![cfg_attr(feature = "use-target", no_std)]

#[cfg(not(feature = "test"))]
compile_error!("Feature 'test' must be enabled for 'test_1'.");

use misc_target_testing as _;

#[cfg_attr(feature = "use-target", defmt_test::tests)]
mod test {
    use misc_target_testing::unit_test_1::SomeStruct;

    #[cfg(feature = "use-target")]
    use defmt::assert_eq;

    #[test]
    fn some_unit_test() {
        let var = SomeStruct::new(1, 2);

        assert_eq!(var.field1(), 1);
        assert_eq!(var.field2(), 2);
    }
}
