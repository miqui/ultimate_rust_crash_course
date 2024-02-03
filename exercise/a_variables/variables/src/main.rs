fn main() {
    //let missiles: i32 = 8;
    //let ready: i32 = 2;

    // let coords: (f32, f32) = (6.3, 15.0);

    //let stuff:i32 (missiles, ready) = (8, 2);

    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;

    let (mut missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
