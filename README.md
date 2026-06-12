# UwUnsafe

[![Crates.io](https://img.shields.io/crates/v/uwunsafe.svg?style=for-the-badge&logo=rust)](https://crates.io/crates/uwunsafe)
[![Documentation](https://img.shields.io/docsrs/uwunsafe?style=for-the-badge&logo=docs.rs)](https://docs.rs/uwunsafe)
[![CodeFactor](https://img.shields.io/codefactor/grade/github/c1ph3rC4t/uwunsafe/main?style=for-the-badge&logo=codefactor)](https://www.codefactor.io/repository/github/c1ph3rc4t/uwunsafe/overview/main)
[![License](https://img.shields.io/crates/l/uwunsafe.svg?style=for-the-badge)](LICENSE)

## Install

```sh
cargo add uwunsafe
```

## What it does

UwUnsafe wraps your function in unsafe blocks. Mostly for the memes, but also because in Rust 2024 `unsafe fn` doesnt make the body unsafe as well.
Me being lazy as well as a never-nester, i would much rather write:

```rs
use uwunsafe::uwu;

#[uwu]
fn some_function() {
    // Do something
}
```

than:

```rs
unsafe fn some_function() {
    unsafe {
        // Do something
    }
}
```

if i can. Thats it, thats the crate.

## Example

```rs
use uwunsafe::uwu;

#[uwu]
fn nya() {
    println!("nya :3")
}

#[uwu]
fn main() {
    let owo = 3;
    println!(":{}", owo);
    nya();
}
```

expands to

```rs
unsafe fn nya() {
    unsafe {
        println!("nya :3")
    }
}

fn main() {
    unsafe {
        let owo = 3;
        println!(":{}", owo);
        nya();
    }
}
```

## License

[MPL-2.0](LICENSE)
