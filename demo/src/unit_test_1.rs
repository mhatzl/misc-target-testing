pub struct SomeStruct {
    field1: usize,
    field2: usize,
}

#[cfg(feature = "test")]
impl SomeStruct {
    pub fn new(var1: usize, var2: usize) -> Self {
        SomeStruct {
            field1: var1,
            field2: var2,
        }
    }

    pub fn field1(&self) -> usize {
        self.field1
    }

    pub fn field2(&self) -> usize {
        self.field2
    }
}

#[cfg_attr(feature = "target-unit-test-1", defmt_test::tests)]
#[cfg(test)]
mod test {
    use super::SomeStruct;

    #[cfg(feature = "use-target")]
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
