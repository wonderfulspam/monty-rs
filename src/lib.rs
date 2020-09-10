//! # Monty-rs - A blazing fast, mostly stupid Monty Hall problem simulator
//!
//!
//! See the [Wikipedia article](https://en.wikipedia.org/wiki/Monty_Hall_problem)
//! for an explanation of the Monty Hall problem and its origins.
//!
//! This project aims to be the fastest Monty Hall simulator in existence.
//! To that end, some corners are cut:
//!
//! - Random number generation is fast rather than properly random
//! - The first option is always chosen as the initial guess
//! - When an incorrect option is removed following the initial choice, the
//! simulation does not randomly pick an option to remove, it simply removes
//! the first option. Half the time (ie. when the initial choice is not
//! correct and there is only one possible option that can be removed) this
//! makes no difference, and regardless, it doesn't really matter. What we
//! care about is whether switching is more successful than not switching.
//!
//! This is likely also the silliest Monty Hall problem simulator in existence.
//! This is a non-goal.

use derive_more::AddAssign; // Adds += overload for Results struct
use num_cpus; // Gets no. of cpus to spawn threads on
use num_format::{Locale, ToFormattedString}; // Allows printing 1000000 as 1,000,000
use rand_core::{RngCore, SeedableRng}; // Traits for generating random numbers and seeding
use rand_xorshift::XorShiftRng; // The fastest possible (?) random number generator
#[cfg(feature = "smallvec")]
use smallvec::{smallvec, SmallVec};
#[cfg(not(feature = "smallvec"))]
use tinyvec::{array_vec, ArrayVec}; // The smallest possible (?) data structure that implements removal
/// Inner struct for [Results](struct.Results.html) that tracks wins and losses for
/// a given strategy
#[derive(Default, AddAssign)]
struct ResultSet {
    wins: u64,
    losses: u64,
}

/// Tracks results for the two possible strategies: switching and staying.
#[derive(Default, AddAssign)]
pub struct Results {
    switched: ResultSet,
    stayed: ResultSet,
}

impl Results {
    /// Calculate win rates for the two strategies as percentages.
    ///
    /// ```rust
    /// use monty_rs::{Results, play_threaded};
    /// use assert_approx_eq::assert_approx_eq;
    /// let results: Results = play_threaded(1_000_000);
    /// let (switched_pct, stayed_pct) = results.calc_win_rate();
    /// // Ensure we are within 0.5 of target percentage
    /// assert_approx_eq!(switched_pct, 0.6667, 0.005);
    /// assert_approx_eq!(stayed_pct, 0.3333, 0.005);
    /// ```
    pub fn calc_win_rate(&self) -> (f64, f64) {
        (
            self.switched.wins as f64 / (self.switched.wins + self.switched.losses) as f64,
            self.stayed.wins as f64 / (self.stayed.wins + self.stayed.losses) as f64,
        )
    }
}

// Format and presents the results in a human-readable way.
impl std::fmt::Display for Results {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (switched_pct, stayed_pct) = self.calc_win_rate();
        let mut result = String::from("Results\n---------------------\n");
        result.push_str(&format!("Switched:\n"));
        result.push_str(&format!(
            "{} wins, {} losses - {:.8}% win rate",
            self.switched.wins.to_formatted_string(&Locale::en),
            self.switched.losses.to_formatted_string(&Locale::en),
            switched_pct * 100.
        ));
        result.push_str("\nStayed:\n");
        result.push_str(&format!(
            "{} wins, {} losses - {:.8}% win rate",
            self.stayed.wins.to_formatted_string(&Locale::en),
            self.stayed.losses.to_formatted_string(&Locale::en),
            stayed_pct * 100.
        ));
        result.push_str(&format!(
            "\n{} games played",
            (self.switched.wins + self.switched.losses + self.stayed.wins + self.stayed.losses)
                .to_formatted_string(&Locale::en)
        ));
        f.write_str(&result)
    }
}

pub struct MontyHall<R> {
    rng: R,
}

impl<R> MontyHall<R>
where
    R: RngCore,
{
    pub fn new_with_rng(rng: R) -> Self {
        Self { rng }
    }
    /// Play a single simulation of the Monty Hall problem.
    #[cfg(not(vecless))]
    fn play_single(&mut self, switch_doors: bool) -> bool {
        #[cfg(not(feature = "smallvec"))]
        let mut doors: ArrayVec<[i8; 3]> = array_vec![0, 1, 2];
        #[cfg(feature = "smallvec")]
        let mut doors: SmallVec<[i8; 3]> = smallvec![0, 1, 2];
        let correct_door = (self.rng.next_u32() % 3) as i8;
        let mut choice: i8 = 0; // https://xkcd.com/221/, sort of

        // Find the first non-correct, non-chosen door and remove it
        doors
            .iter()
            .position(|&x| x != correct_door && x != choice)
            .map(|e| doors.remove(e));

        if switch_doors {
            // Unwrapping is safe; we know there will always be at least one viable option left
            choice = *doors.iter().find(|&&x| x != choice).unwrap();
        }

        choice == correct_door
    }

    #[cfg(vecless)]
    fn play_single(&mut self, switch_doors: bool) -> bool {
        let correct_door = (self.rng.next_u32() % 3) as i8;
        let mut choice: i8 = 0; // https://xkcd.com/221/, sort of

        if switch_doors {
            let non_correct = (1i8..=2).into_iter().find(|&x| x != correct_door).unwrap();
            choice = (1i8..=2).into_iter().find(|&x| x != non_correct).unwrap();
        }

        choice == correct_door
    }

    /// Play a number of simulations.
    ///
    /// Half of the simulations use the switching strategy, the other half do not.
    ///
    /// ```rust
    /// use monty_rs::{MontyHall, Results};
    /// let mut monty = MontyHall::default();
    /// let results: Results = monty.play_multiple(1_000_000);
    /// ```
    pub fn play_multiple(&mut self, iterations: u64) -> Results {
        let half = iterations / 2;
        let mut results = Results::default();
        for _ in 0..half {
            let switch = true;
            let won = self.play_single(switch);
            if won {
                results.switched.wins += 1;
            } else {
                results.switched.losses += 1;
            }
        }
        for _ in 0..half {
            let switch = false;
            let won = self.play_single(switch);
            if won {
                results.stayed.wins += 1;
            } else {
                results.stayed.losses += 1;
            }
        }
        results
    }
}

impl Default for MontyHall<XorShiftRng> {
    fn default() -> Self {
        Self::new_with_rng(XorShiftRng::seed_from_u64(0))
    }
}

/// A wrapper around [play_multiple](fn.play_multiple.html) that splits the work by
/// the amount of logical CPUs available.
///
/// ```rust
/// use monty_rs::{Results, play_threaded};
/// let results: Results = play_threaded(1_000_000);
/// ```
pub fn play_threaded(iterations: u64) -> Results {
    let threads = num_cpus::get();

    let iterations_per_thread = iterations / threads as u64;
    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        let iters = iterations_per_thread.clone();
        let mut monty = MontyHall::default();
        handles.push(std::thread::spawn(move || monty.play_multiple(iters)));
    }
    let mut results = Results::default();
    for handle in handles {
        results += handle.join().unwrap();
    }
    results
}
