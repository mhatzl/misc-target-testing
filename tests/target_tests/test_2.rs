#![cfg_attr(feature = "xmc-target", no_main)]
#![cfg_attr(feature = "xmc-target", no_std)]

use misc_target_testing as _;

#[cfg_attr(feature = "xmc-target", defmt_test::tests)]
mod test {
    use misc_target_testing::unit_test_2::SomeStruct;

    #[cfg(feature = "xmc-target")]
    use defmt::assert_eq;

    #[test]
    fn some_unit_test() {
        let var = SomeStruct {
            field1: 1,
            field2: 2,
        };

        assert_eq!(var.field1, 1);
        assert_eq!(var.field2, 2);
    }
}
