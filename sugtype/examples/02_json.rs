use std::{fs::File, io::Read};

use sugtype::{better_type::BetterType, datavec::DataVec, sugtype::Sugtype};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("dataset.json")?;

    let mut content_dataset = String::new();
    file.read_to_string(&mut content_dataset)?;

    // let content_dataset = r#"
    // [
    //     {
    //         "dtBool": false,
    //         "dtU8": 255,
    //         "dtuU16": 65535,
    //         "dtuU32": 4294967295
    //     }
    // ]
    // "#;

    let _ = match serde_json::from_str::<DataVec>(&content_dataset) {
        Ok(data) => {
            let arr = data.dataset();

            let result = arr
                .iter()
                // .take(1)
                .map(|item| match item.as_object() {
                    Some(data) => data
                        .iter()
                        .map(|(key, value)| {
                            BetterType::new(
                                key,
                                &Sugtype::obtain_better_type(value.to_string()).unwrap_or_default(),
                            )
                        })
                        .collect::<Vec<_>>(),
                    None => vec![],
                })
                .flatten()
                .inspect(|x| println!("{}", x))
                .collect::<Vec<_>>();

            result
        }
        Err(_) => {
            vec![BetterType::default()]
        }
    };

    Ok(())
}
