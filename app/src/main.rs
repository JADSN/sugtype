fn obtain_better_type(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // * ======================
    // * === BEGIN: boolean ===
    // * ======================
    let is_boolean = input.parse::<bool>().is_ok();
    if is_boolean {
        return Ok(String::from("bool"));
    }

    // * =======================
    // * === BEGIN: unsigned ===
    // * =======================

    // ?: Check Min of u16 berfore min of u8:
    // ?: - Override min of u16 from 0 to 256;
    // ?: - Don't suggest when input is equal to 0, the type u16;

    let u16_parsed = input.parse::<u16>();

    let is_u8 = input.parse::<u8>().is_ok();
    if is_u8 {
        return Ok(String::from("u8"));
    }

    let is_u16 = u16_parsed.is_ok();
    if is_u16 {
        return Ok(String::from("u16"));
    }

    let is_u32 = input.parse::<u32>().is_ok();
    if is_u32 {
        return Ok(String::from("u32"));
    }

    let is_u64 = input.parse::<u64>().is_ok();
    if is_u64 {
        return Ok(String::from("u64"));
    }

    let is_u128 = input.parse::<u128>().is_ok();
    if is_u128 {
        return Ok(String::from("u128"));
    }

    let is_usize = input.parse::<usize>().is_ok();
    if is_usize {
        return Ok(String::from("usize"));
    }

    // * =====================
    // * === END: unsigned ===
    // * =====================

    // * ======================
    // * === BEGIN: signed ===
    // * ======================

    let is_i8 = input.parse::<i8>().is_ok();
    if is_i8 {
        return Ok(String::from("i8"));
    }

    let is_i16 = input.parse::<i16>().is_ok();
    if is_i16 {
        return Ok(String::from("i16"));
    }

    let is_i32 = input.parse::<i32>().is_ok();
    if is_i32 {
        return Ok(String::from("i32"));
    }

    let is_i64 = input.parse::<i64>().is_ok();
    if is_i64 {
        return Ok(String::from("i64"));
    }

    let is_i128 = input.parse::<i128>().is_ok();
    if is_i128 {
        return Ok(String::from("i128"));
    }

    let is_isize = input.parse::<isize>().is_ok();
    if is_isize {
        return Ok(String::from("isize"));
    }

    // * ===================
    // * === END: signed ===
    // * ===================

    // * ==================
    // * == BEGIN: float ==
    // * ==================

    let is_f32 = input.parse::<f32>().is_ok();
    if is_f32 {
        return Ok(String::from("f32"));
    }

    let is_f64 = input.parse::<f64>().is_ok();
    if is_f64 {
        return Ok(String::from("f64"));
    }

    // * ==================
    // * === END: float ===
    // * ==================

    // * =================
    // * = BEGIN: String =
    // * =================

    let is_string = input.parse::<String>().is_ok();
    if is_string {
        return Ok(String::from("String"));
    }

    // * ==================
    // * === END: String ==
    // * ==================

    Ok(String::default())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_min_u8 = u8::MIN.to_string();
    let input_max_u8 = u8::MAX.to_string();

    let better_type_input_min_u8 = obtain_better_type(&input_min_u8)?;
    let better_type_input_max_u8 = obtain_better_type(&input_max_u8)?;

    println!(
        "MIN: {} Better type: {}",
        input_min_u8, better_type_input_min_u8
    );

    println!(
        "MAX: {} Better type: {}",
        input_max_u8, better_type_input_max_u8
    );

    assert_eq!(better_type_input_min_u8, "u8");
    assert_eq!(better_type_input_max_u8, "u8");

    println!();
    println!("================================================================");
    println!();

    let input_min_u16 = u16::MIN.to_string();
    let input_max_u16 = u16::MAX.to_string();

    let better_type_input_min_u16 = obtain_better_type(&input_min_u16)?;
    let better_type_input_max_u16 = obtain_better_type(&input_max_u16)?;

    println!(
        "MIN: {} - Better type: {}",
        input_min_u16, better_type_input_min_u16
    );

    println!(
        "MAX: {} - Better type: {}",
        input_max_u16, better_type_input_max_u16
    );

    assert_eq!(better_type_input_min_u16, "u8");
    assert_eq!(better_type_input_max_u16, "u16");

    println!();
    println!("================================================================");
    println!();

    let input_min_u32 = u32::MIN.to_string();
    let input_max_u32 = u32::MAX.to_string();

    let better_type_input_min_u32 = obtain_better_type(&input_min_u32)?;
    let better_type_input_max_u32 = obtain_better_type(&input_max_u32)?;

    println!(
        "MIN: {} - Better type: {}",
        input_min_u32, better_type_input_min_u32
    );

    println!(
        "MAX: {} - Better type: {}",
        input_max_u32, better_type_input_max_u32
    );

    assert_eq!(better_type_input_min_u32, "u8");
    assert_eq!(better_type_input_max_u32, "u32");

    println!();
    println!("================================================================");
    println!();

    Ok(())
}
