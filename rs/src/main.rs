
// Functional types with enums
enum RSEnum {
    Foo(i32),
    Bar(String),
    Baz(Vec<String>),
    Faz(Option<i32>),
    Far(fn() -> i32);
}

fn far() -> i32 {
    return 5;
}

fn main() {
    let a: Vec<i32> = vec![];
    let mut b = a;
    
    b.push(1);
    println!("{:?}", b);

    // Shadowing
    let a = "Truth";

    let mut x = 5;
    let y = &x;
    {
        let z = &mut x;

        *z = 7;
        println!("{:?}", z);
    }

    x = 9;
    println!("{:?}", x);

    let foo = RSEnum::Foo(bar);

    // Pattern matching according to type
    if let RSEnum::Foo(value) = foo { }
    match foo {
        RSEnum::Foo(value) => {

        },
        RSEnum::Faz(Some(value)) => { },
        RSEnum::Faz(None) => { },
        _ => {}
    }
}
