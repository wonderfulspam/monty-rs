fn main() {
    let results = monty_rs::play_threaded(1_000_000_000);
    println!("{}", results);
}