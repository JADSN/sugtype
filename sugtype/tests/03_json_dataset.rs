#[cfg(test)]
mod tests {
    use serde_json::{self, Value};
    use std::{fs::File, io::Read};

    use sugtype::sugtype::Sugtype;
    use sugtype::{better_type::BetterType, datavec::DataVec};

    #[test]
    fn json() -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::open("../dataset.json")?;

        let mut content_dataset = String::new();
        file.read_to_string(&mut content_dataset)?;

        let content_dataset = r#"
        [
            {
                "dtBool": false,
                "dtU8": 255,
                "dtuU16": 65535,
                "dtuU32": 4294967295
            }
        ]
        "#;

        let better_types = match serde_json::from_str::<DataVec>(&content_dataset) {
            Ok(data) => {
                let arr: &[Value] = data.dataset();

                let result = arr
                    .iter()
                    // .take(1)
                    .map(|item| match item.as_object() {
                        Some(data) => data
                            .iter()
                            .map(|(key, value)| {
                                BetterType::new(
                                    key,
                                    &Sugtype::obtain_better_type(value.to_string())
                                        .unwrap_or_default(),
                                )
                            })
                            .collect::<Vec<_>>(),
                        None => vec![],
                    })
                    .flatten()
                    // .inspect(|x| println!("{}", x))
                    .collect::<Vec<_>>();

                result
            }
            Err(_) => {
                vec![BetterType::default()]
            }
        };

        let output = vec![
            BetterType::new("dtBool", "bool"),
            BetterType::new("dtU8", "u8"),
            BetterType::new("dtuU16", "u16"),
            BetterType::new("dtuU32", "u32"),
        ];
        assert_eq!(better_types, output);

        Ok(())
    }
}
