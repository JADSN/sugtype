pub struct Sugtype;

impl Sugtype {
    pub fn new() -> Self {
        Self
    }

    pub fn obtain_better_type(input: String) -> Result<String, Box<dyn std::error::Error>> {
        use super::mintype::MinType;

        let input = input.as_str();
        // println!("Input: {:?}", input);

        let mut types_parsed = vec![];

        // * ======================
        // * === BEGIN: boolean ===
        // * ======================
        let is_boolean = input.parse::<bool>().is_ok();
        if is_boolean {
            types_parsed.push("bool");
        }

        // * =======================
        // * === BEGIN: unsigned ===
        // * =======================

        let u16_parsed = input.parse::<u16>();

        let is_u8 = input.parse::<u8>().is_ok();

        if is_u8 {
            types_parsed.push("u8");
        }

        let is_u16 = u16_parsed.is_ok();
        if is_u16 {
            types_parsed.push("u16");
        }

        let is_u32 = input.parse::<u32>().is_ok();
        if is_u32 {
            types_parsed.push("u32");
        }

        let is_u64 = input.parse::<u64>().is_ok();
        if is_u64 {
            types_parsed.push("u64");
        }

        let is_u128 = input.parse::<u128>().is_ok();
        if is_u128 {
            types_parsed.push("u128");
        }

        let is_usize = input.parse::<usize>().is_ok();
        if is_usize {
            types_parsed.push("usize");
        }

        // * =====================
        // * === END: unsigned ===
        // * =====================

        // * ======================
        // * === BEGIN: signed ===
        // * ======================

        let is_i8 = input.parse::<i8>().is_ok();
        if is_i8 {
            types_parsed.push("i8");
        }

        let is_i16 = input.parse::<i16>().is_ok();
        if is_i16 {
            types_parsed.push("i16");
        }

        let is_i32 = input.parse::<i32>().is_ok();
        if is_i32 {
            types_parsed.push("i32");
        }

        let is_i64 = input.parse::<i64>().is_ok();
        if is_i64 {
            types_parsed.push("i64");
        }

        let is_i128 = input.parse::<i128>().is_ok();
        if is_i128 {
            types_parsed.push("i128");
        }

        let is_isize = input.parse::<isize>().is_ok();
        if is_isize {
            types_parsed.push("isize");
        }

        // * ===================
        // * === END: signed ===
        // * ===================

        // * ==================
        // * == BEGIN: float ==
        // * ==================

        let is_f32 = input.parse::<f32>().is_ok();
        if is_f32 {
            types_parsed.push("f32");
        }

        let is_f64 = input.parse::<f64>().is_ok();
        if is_f64 {
            types_parsed.push("f64");
        }

        // * ==================
        // * === END: float ===
        // * ==================

        // * =================
        // * = BEGIN: String =
        // * =================

        let is_string = input.parse::<String>().is_ok();
        if is_string {
            types_parsed.push("String");
        }

        // * ==================
        // * === END: String ==
        // * ==================

        // #[cfg(debug_assertions)]
        // println!("{:#?}", types_parsed);

        Ok(MinType::mintype(types_parsed).to_string())
    }
}
