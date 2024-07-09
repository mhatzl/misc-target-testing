pub struct SomeStruct {
    pub field1: usize,
    pub field2: usize,
}

#[cfg_attr(feature = "target-unit-test-2", defmt_test::tests)]
#[cfg(test)]
mod test {
    use super::SomeStruct;

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
