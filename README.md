### Arithmetic

Arithmetic is a procedural macro for Rust enums that allows you to implement the Add/Sub trait for specific variants by adding the `#[add]`/`#[sub]`/`#[mul]`/`#[div]` attribute.

### How It Works

The Arithmetic derive macro processes your enum and generates implementations of the `Add`/`Sub`/`Mul`/`Div` trait for the variants marked with the `#[add]`/`#[sub]`/`#[mul]`/`#[div]` attribute. This allows you to perform add/sub/mul/div operations on these variants while other variants will not have this functionality.

### Example
```rust
use arithmetic_enum::Arithmetic;

#[derive(Clone, Arithmetic, Debug)]
enum MyEnum {
    #[add]
    #[mul]
    #[div]
    A(i32),
    #[add]
    #[sub]
    B(f64),
}

fn main() {
    let x = MyEnum::A(1);
    let y = MyEnum::A(2);

    let c = MyEnum::B(3.0);
    let d = MyEnum::B(3.0);

    match &x + &y {
        MyEnum::A(value) => println!("&x + &y = {}", value),
        _ => println!("Unsupported operation"),
    }

    match x + y {
        MyEnum::A(value) => println!("x + y = {}", value),
        _ => println!("Unsupported operation"),
    }

    match &c - &d {
        MyEnum::B(value) => println!("&c - &d = {}", value),
        _ => println!("Unsupported operation"),
    }

    match c + d {
        MyEnum::B(value) => println!("c + d = {}", value),
        _ => println!("Unsupported operation"),
    }
}

```
