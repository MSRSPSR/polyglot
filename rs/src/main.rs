fn main() {
    let a = vec![];
    let mut b = a;

    b.push(1);
    println!("{:?}", b);

    let x = 5;
    let y = &x;

    println!("{:?}", x + y);
}
