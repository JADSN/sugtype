use sugtype::sugtype;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_min_u8 = u8::MIN.to_string();
    let input_max_u8 = u8::MAX.to_string();

    let better_type_input_min_u8 = sugtype::obtain_better_type(&input_min_u8)?;
    let better_type_input_max_u8 = sugtype::obtain_better_type(&input_max_u8)?;

    println!();

    println!(
        "MIN: \t{} \tBetter type: {}",
        input_min_u8, better_type_input_min_u8
    );

    println!(
        "MAX: \t{} \tBetter type: {}",
        input_max_u8, better_type_input_max_u8
    );

    assert_eq!(better_type_input_min_u8, "u8");
    assert_eq!(better_type_input_max_u8, "u8");

    Ok(())
}
