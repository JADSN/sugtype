# Sugtype

[![dependency status](https://deps.rs/repo/github/JADSN/sugtype/status.svg)](https://deps.rs/repo/github/JADSN/sugtype)
https://github.com/JADSN/sugtype
### Suggest better types for a given dataset

## Precedence

1. `boolean`
1. **`unsigned`**
    1. `u8`
    1. `u16`
    1. `u32`
    1. `u64`
    1. `u128`
1. **`signed`**
    1. `i8`
    1. `i16`
    1. `i32`
    1. `i64`
    1. `i128`
1. **`float`**
    1. `f32`
    2. `f64`

1. `String`

## Baseline

1. Case **unsigned**, **signed**, **float**: max value

## Todos

- [ ] **`f32`** and **`f64`** boundaries;

## Alternatives

[quicktype](https://quicktype.io/)