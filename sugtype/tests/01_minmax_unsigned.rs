#[cfg(test)]
mod tests {
    #[test]
    fn min_u8_max_u8() -> Result<(), Box<dyn std::error::Error>> {
        use sugtype::sugtype::Sugtype;

        let input_min_u8 = u8::MIN.to_string();
        let input_max_u8 = u8::MAX.to_string();

        let better_type_input_min_u8 = Sugtype::obtain_better_type(&input_min_u8)?;
        let better_type_input_max_u8 = Sugtype::obtain_better_type(&input_max_u8)?;

        assert_eq!(better_type_input_min_u8, "u8");
        assert_eq!(better_type_input_max_u8, "u8");

        Ok(())
    }

    #[test]
    fn min_u16_max_u16() -> Result<(), Box<dyn std::error::Error>> {
        use sugtype::sugtype::Sugtype;

        let input_min_u16 = u16::MIN.to_string();
        let input_max_u16 = u16::MAX.to_string();

        let better_type_input_min_u16 = Sugtype::obtain_better_type(&input_min_u16)?;
        let better_type_input_max_u16 = Sugtype::obtain_better_type(&input_max_u16)?;

        assert_eq!(better_type_input_min_u16, "u8");
        assert_eq!(better_type_input_max_u16, "u16");

        Ok(())
    }

    #[test]
    fn min_u32_max_u32() -> Result<(), Box<dyn std::error::Error>> {
        use sugtype::sugtype::Sugtype;

        let input_min_u32 = u32::MIN.to_string();
        let input_max_u32 = u32::MAX.to_string();

        let better_type_input_min_u32 = Sugtype::obtain_better_type(&input_min_u32)?;
        let better_type_input_max_u32 = Sugtype::obtain_better_type(&input_max_u32)?;

        assert_eq!(better_type_input_min_u32, "u8");
        assert_eq!(better_type_input_max_u32, "u32");

        Ok(())
    }

    #[test]
    fn min_u64_max_u64() -> Result<(), Box<dyn std::error::Error>> {
        use sugtype::sugtype::Sugtype;

        let input_min_u64 = u64::MIN.to_string();
        let input_max_u64 = u64::MAX.to_string();

        let better_type_input_min_u64 = Sugtype::obtain_better_type(&input_min_u64)?;
        let better_type_input_max_u64 = Sugtype::obtain_better_type(&input_max_u64)?;

        assert_eq!(better_type_input_min_u64, "u8");
        assert_eq!(better_type_input_max_u64, "u64");

        Ok(())
    }

    #[test]
    fn min_u128_max_u128() -> Result<(), Box<dyn std::error::Error>> {
        use sugtype::sugtype::Sugtype;

        let input_min_u128 = u128::MIN.to_string();
        let input_max_u128 = u128::MAX.to_string();

        let better_type_input_min_u128 = Sugtype::obtain_better_type(&input_min_u128)?;
        let better_type_input_max_u128 = Sugtype::obtain_better_type(&input_max_u128)?;

        assert_eq!(better_type_input_min_u128, "u8");
        assert_eq!(better_type_input_max_u128, "u128");

        Ok(())
    }

    #[test]
    fn min_usize_max_usize() -> Result<(), Box<dyn std::error::Error>> {
        use sugtype::sugtype::Sugtype;

        let input_min_usize = usize::MIN.to_string();
        let input_max_usize = usize::MAX.to_string();

        let better_type_input_min_usize = Sugtype::obtain_better_type(&input_min_usize)?;
        let better_type_input_max_usize = Sugtype::obtain_better_type(&input_max_usize)?;

        assert_eq!(better_type_input_min_usize, "u8");

        // TODO: Check this test pass, in systems of 32Bits
        #[cfg(target_pointer_width = "64")]
        assert_eq!(better_type_input_max_usize, "u64");

        Ok(())
    }
}
