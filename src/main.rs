use libc_print::libc_println;

fn main() {
    // Allow passing an exponent, ie. the no. of zeros
    // Defaults to 9 = 1_000_000_000 iterations
    let pow: u32 = std::env::args()
        .nth(1)
        .unwrap_or("9".to_string())
        .parse()
        .expect("You must pass a valid integer");
    let iterations = 10u64.pow(pow);
    let results = monty_rs::play_threaded(iterations);
    libc_println!("{}", results);
}
