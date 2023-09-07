# Simple Input
This is an easy to use input library crate. It automatically handles parsing errors and data validation with a simple syntax and good looking error messages.

### Error message:

![Error](./images/error.png)

### Array input:

![Array](images/array.png)

## Usage
Normal input:
```rust
use simple_input::input;

fn main() {
    let num: u8 = input("Enter a number: ");
    println!();
}
```

Array input:
```rust
use simple_input::input_array;

fn main() {
    let nums: Vec<u8> = input_array("Enter a list of numbers: ");
    println!();
}
```

Data validation:
```rust
use simple_input::input;

fn main() {
    let validated_num = input_with_validation::<u8>("Enter a number between 0 and 10: ", Box::new(|value| {
        if value < 0 || value > 10 {
            Err("Number must be between 0 and 10")
        } else {
            Ok(())
        }
    }));
    println!();
}
```

## Features
- Array input
- Builtin data validation with custom messages
- Pretty error messages
- Cross platform

## Dependencies
- [crossterm](https://crates.io/crates/crossterm)

