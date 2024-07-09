#![cfg_attr(feature = "use-target", no_main)]
#![cfg_attr(feature = "use-target", no_std)]

use misc_target_testing as _;

#[cfg_attr(feature = "use-target", defmt_test::tests)]
mod test {
    use misc_target_testing::unit_test_2::SomeStruct;

    #[cfg(feature = "use-target")]
    use defmt::assert_eq;

    #[test]
    fn integration_test_2() {
        let var = SomeStruct {
            field1: 1,
            field2: 2,
        };

        assert_eq!(var.field1, 1);
        assert_eq!(var.field2, 2);
    }
}
