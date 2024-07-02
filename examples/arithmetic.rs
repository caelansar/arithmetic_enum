use arithmetic_enum::Arithmetic;

#[derive(Clone, Arithmetic, Debug, PartialEq)]
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

    assert_eq!(MyEnum::A(3), &x + &y);
    assert_eq!(MyEnum::A(2), &x * &y);
    assert_eq!(MyEnum::A(2), &y * &x);
    assert_eq!(MyEnum::A(3), x + y);

    assert_eq!(MyEnum::B(0.0), &c - &d);
    assert_eq!(MyEnum::B(6.0), c + d);
}
