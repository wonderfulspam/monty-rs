fn main() {
    let pow: u32 = std::env::args().nth(1).unwrap_or("9".to_string()).parse().unwrap();
    let iterations: u64 = 10u64.pow(pow);
    let results = monty_rs::play_threaded(iterations);
    #[cfg(debug_assertions)]
    {
        let (switched_pct, stayed_pct) = results.calc_win_rate();
        assert_eq!("66.7", format!("{:.1}", switched_pct));
        assert_eq!("33.3", format!("{:.1}", stayed_pct));
    }
    println!("{}", results);
}