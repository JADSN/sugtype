# Codes

```rust

let arr: &[Value] = data.dataset();
let mut result_keys = vec![];
let mut result_values = vec![];

for item in arr.iter().take(1) {
    match item.as_object() {
        Some(data) => {
            // TODO: Tentar com for loop returnar a linha 73
            for value in data.values() {
                let value_of_key = value.to_string();
                // let value_of_key_clone = value_of_key.clone();

                let better_type =
                    Sugtype::obtain_better_type(value_of_key).unwrap_or_default();

                result_values.push(better_type);
            }

            // let mut result = vec![];
            let better_types = data
                .keys()
                .enumerate()
                .map(|(idx, key)| {
                    let value = result_values.iter().nth(idx).unwrap().deref();
                    BetterType::new(key.to_string(), value.to_string())
                })
                .collect::<Vec<_>>();

            println!("{:?}", better_types);
        }
        None => todo!(),
    }
}

```

## Inspect 
```rust

.inspect(|x| {
    #[cfg(debug_assertions)]
    println!("{:?}", x);
})

```

## All rust types
```rust

let mut all_rust_types = vec![
    "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64",
    "usize", "isize", "String",
];

```
