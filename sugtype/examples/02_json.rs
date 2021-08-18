use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

use sugtype::{better_type::BetterType, datavec::DataVec, sugtype::Sugtype};
// use sugtype::sugtype::Data;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("dataset.json")?;

    let mut content_dataset = String::new();
    file.read_to_string(&mut content_dataset)?;

    let _ = match serde_json::from_str::<DataVec>(&content_dataset) {
        Ok(data) => {
            let arr = data.dataset();

            // let mut hs = HashMap::new();

            let better_types = arr
                .iter()
                .take(5)
                .map(|item| match item.as_object() {
                    Some(data) => data
                        .iter()
                        .map(|(key, value)| {
                            // println!("{}:{}", key, value);
                            // Sugtype::obtain_key_type_count(key.clone(), value.clone()).unwrap()
                            // let key_deref = key.clone();

                            // let data_value = value.to_string();
                            // let better_type =
                            //     Sugtype::obtain_better_type(data_value).unwrap_or_default();

                            // dv.new(better_type, 0);
                            // dt.insert(dv.clone());

                            // data_unique.new(key_deref, dt.clone());

                            // dbg!(&data_unique);

                            let bt = BetterType::new(
                                key,
                                &Sugtype::obtain_better_type(value.to_string()).unwrap_or_default(),
                            );
                        })
                        // .flatten()
                        // .inspect(|x| println!("{:?}", x))
                        .collect::<Vec<_>>(),
                    None => vec![],
                })
                .flatten()
                .inspect(|x| println!("{:?}", x))
                .collect::<Vec<_>>();

            better_types
        }
        Err(_) => {
            vec![]
            // vec![Data::default()]
        }
    };

    Ok(())
}
