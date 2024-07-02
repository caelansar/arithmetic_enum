use arithmetic_enum::Arithmetic;

#[derive(Clone, Arithmetic, Debug)]
enum MyEnum {
    #[add]
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
