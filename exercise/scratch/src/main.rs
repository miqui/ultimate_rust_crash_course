fn main() {
    let s = String::from("hello"); // s owns the string
    let slice = &s; // slice is a reference to the entire String

    println!("The borrowed string is: {}", slice);
}
