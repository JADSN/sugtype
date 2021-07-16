#[cfg(test)]
mod tests {
    use sugtype::sugtype::Sugtype;

    #[test]
    fn min_i8_max_i8() -> Result<(), Box<dyn std::error::Error>> {
        let input_min_i8 = i8::MIN.to_string();
        let input_max_i8 = i8::MAX.to_string();

        let better_type_input_min_i8 = Sugtype::obtain_better_type(input_min_i8)?;
        let better_type_input_max_i8 = Sugtype::obtain_better_type(input_max_i8)?;

        assert_eq!(better_type_input_min_i8, "i8");
        assert_eq!(better_type_input_max_i8, "u8");

        Ok(())
    }

    #[test]
    fn min_i16_max_i16() -> Result<(), Box<dyn std::error::Error>> {
        let input_min_i16 = i16::MIN.to_string();
        let input_max_i16 = i16::MAX.to_string();

        let better_type_input_min_i16 = Sugtype::obtain_better_type(input_min_i16)?;
        let better_type_input_max_i16 = Sugtype::obtain_better_type(input_max_i16)?;

        assert_eq!(better_type_input_min_i16, "i16");
        assert_eq!(better_type_input_max_i16, "u16");

        Ok(())
    }

    #[test]
    fn min_i32_max_i32() -> Result<(), Box<dyn std::error::Error>> {
        let input_min_i32 = i32::MIN.to_string();
        let input_max_i32 = i32::MAX.to_string();

        let better_type_input_min_i32 = Sugtype::obtain_better_type(input_min_i32)?;
        let better_type_input_max_i32 = Sugtype::obtain_better_type(input_max_i32)?;

        assert_eq!(better_type_input_min_i32, "i32");
        assert_eq!(better_type_input_max_i32, "u32");

        Ok(())
    }

    #[test]
    fn min_i64_max_i64() -> Result<(), Box<dyn std::error::Error>> {
        let input_min_i64 = i64::MIN.to_string();
        let input_max_i64 = i64::MAX.to_string();

        let better_type_input_min_i64 = Sugtype::obtain_better_type(input_min_i64)?;
        let better_type_input_max_i64 = Sugtype::obtain_better_type(input_max_i64)?;

        assert_eq!(better_type_input_min_i64, "i64");
        assert_eq!(better_type_input_max_i64, "u64");

        Ok(())
    }

    #[test]
    fn min_i128_max_i128() -> Result<(), Box<dyn std::error::Error>> {
        let input_min_i128 = i128::MIN.to_string();
        let input_max_i128 = i128::MAX.to_string();

        let better_type_input_min_i128 = Sugtype::obtain_better_type(input_min_i128)?;
        let better_type_input_max_i128 = Sugtype::obtain_better_type(input_max_i128)?;

        assert_eq!(better_type_input_min_i128, "i128");
        assert_eq!(better_type_input_max_i128, "u128");

        Ok(())
    }
}
