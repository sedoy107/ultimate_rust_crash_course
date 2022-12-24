use message::greet;

fn main() {
    let mut my_val = 16;
    my_val = 2;

    const MY_INT64: i64 = 100500;

    println!("Hello, world {my_val}, {MY_INT64}!");

    greet();
}
