pub struct MinType;

impl MinType {
    pub fn mintype(input: Vec<&str>) -> &str {
        if input.contains(&"u8") {
            return "u8";
        }

        if input.contains(&"u16") {
            return "u16";
        }

        if input.contains(&"u32") {
            return "u32";
        }

        if input.contains(&"u64") {
            return "u64";
        }

        if input.contains(&"u128") {
            return "u128";
        }

        if input.contains(&"usize") {
            return "usize";
        }

        if input.contains(&"i8") {
            return "i8";
        }

        if input.contains(&"i16") {
            return "i16";
        }

        if input.contains(&"i32") {
            return "i32";
        }

        if input.contains(&"i64") {
            return "i64";
        }

        if input.contains(&"i128") {
            return "i128";
        }

        if input.contains(&"isize") {
            return "isize";
        }

        if input.contains(&"f32") {
            return "f32";
        }

        if input.contains(&"f64") {
            return "f64";
        }

        // TODO: Turn boolean first detection.
        if input.contains(&"bool") {
            return "bool";
        }

        return "String";
    }
}

#[cfg(test)]
mod tests {

    use super::MinType;

    #[test]
    fn all_types() -> Result<(), Box<dyn std::error::Error>> {
        let data = vec![
            "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64",
            "usize", "isize", "bool",
        ];

        let mintype = MinType::mintype(data);

        assert_eq!(mintype, "u8");

        Ok(())
    }

    #[test]
    fn signeds() -> Result<(), Box<dyn std::error::Error>> {
        let data = vec!["u8", "u16", "u32", "u64", "u128"];

        let mintype = MinType::mintype(data);

        assert_eq!(mintype, "u8");

        Ok(())
    }

    #[test]
    fn unsigneds() -> Result<(), Box<dyn std::error::Error>> {
        let data = vec!["i8", "i16", "i32", "i64", "i128", "f32"];

        let mintype = MinType::mintype(data);

        assert_eq!(mintype, "i8");

        Ok(())
    }

    #[test]
    fn signeds_and_unsigned() -> Result<(), Box<dyn std::error::Error>> {
        let data = vec!["i8", "u8"];

        let mintype = MinType::mintype(data);

        assert_eq!(mintype, "u8");

        Ok(())
    }

    #[test]
    fn floats() -> Result<(), Box<dyn std::error::Error>> {
        let data = vec!["f32", "f64"];

        let mintype = MinType::mintype(data);

        assert_eq!(mintype, "f32");

        Ok(())
    }

    #[test]
    fn bool_string() -> Result<(), Box<dyn std::error::Error>> {
        let data = vec!["bool", "String"];

        let mintype = MinType::mintype(data);

        assert_eq!(mintype, "bool");

        Ok(())
    }

    #[test]
    fn no_match() -> Result<(), Box<dyn std::error::Error>> {
        let data = vec!["String"];

        let mintype = MinType::mintype(data);

        assert_eq!(mintype, "String");

        Ok(())
    }
}
