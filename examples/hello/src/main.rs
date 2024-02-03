use hello::{english, russian, spanish};

fn main() {
    english::greet();
    spanish::greet();
    russian::greet();

    let bunnies = 2;
    println!("{}",bunnies);
    const HEY_NUMBER = 42;
}
