fn main() {
    // Declare a constant named STARTING_MISSILES and set it to 8 (the type is i32).
    const STARTING_MISSILES: i32 = 8;

    // Declare a constant named READY_AMOUNT and set it to 2 (also i32).
    const READY_AMOUNT: i32 = 2;
    // Use the constants to initialize missiles and ready

    // Declare the variable missiles and initialize it to 8
    let mut missiles: i32 = STARTING_MISSILES;
    // Declare the variable ready and initialize it to 2
    let ready: i32 = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}