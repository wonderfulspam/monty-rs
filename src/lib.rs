use derive_more::{Add, AddAssign};
use num_cpus;
use num_format::{Locale, ToFormattedString};
use std::time::{SystemTime, UNIX_EPOCH};
use tinyvec::{array_vec, ArrayVec};
use xorshift::{Rng, SeedableRng, Xorshift128};

#[derive(Default, Add, AddAssign)]
pub struct Results {
    switched: ResultSet,
    stayed: ResultSet,
}

impl Results {
    pub fn calc_win_rate(&self) -> (f64, f64) {
        (self.switched.wins as f64 / (self.switched.wins + self.switched.losses) as f64 * 100.,
        self.stayed.wins as f64 / (self.stayed.wins + self.stayed.losses) as f64 * 100.)
    }

}

impl std::fmt::Display for Results {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (switched_pct, stayed_pct) = self.calc_win_rate();
        let mut result = String::from("Results\n---------------------\n");
        result.push_str(&format!("Switched:\n"));
        result.push_str(&format!(
            "{} wins, {} losses - {:.8}% win rate",
            self.switched.wins.to_formatted_string(&Locale::en),
            self.switched.losses.to_formatted_string(&Locale::en),
            switched_pct
        ));
        result.push_str("\nStayed:\n");
        result.push_str(&format!(
            "{} wins, {} losses - {:.8}% win rate",
            self.stayed.wins.to_formatted_string(&Locale::en),
            self.stayed.losses.to_formatted_string(&Locale::en),
            stayed_pct
        ));
        result.push_str(&format!(
            "\n{} games played",
            (self.switched.wins + self.switched.losses + self.stayed.wins + self.stayed.losses)
                .to_formatted_string(&Locale::en)
        ));
        f.write_str(&result)
    }
}
#[derive(Default, Add, AddAssign)]
struct ResultSet {
    wins: u64,
    losses: u64,
}

fn now() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    since_the_epoch.subsec_nanos() as u64
}

pub fn play_one(rng: &mut impl Rng) -> (bool, bool) {
    let mut doors: ArrayVec<[i8; 3]> = array_vec![0, 1, 2];
    let correct_door = rng.gen_range(0, 3);
    let choice = rng.gen_range(0, 3);

    let removable_doors: ArrayVec<[i8; 2]> = doors
        .clone()
        .into_iter()
        .filter(|&x| x != correct_door && x != choice)
        .collect();
    let removed_door = {
        if removable_doors.len() == 1 {
            removable_doors[0]
        } else {
            removable_doors[rng.gen_range(0, removable_doors.len())]
        }
    };

    doors.retain(|&x| x != removed_door);

    let final_choice: i8 = doors[rng.gen_range(0, 2)];

    (final_choice == correct_door, final_choice != choice)
}

pub fn play_simple(rng: &mut impl Rng) -> (bool, bool) {
    let mut doors: ArrayVec<[i8; 3]> = array_vec![0, 1, 2];
    let correct_door = rng.gen_range(0, 3);
    let choice: i8 = rng.gen_range(0, 3);
    let switch_doors = rng.gen_weighted_bool(2);

    doors
        .iter()
        .position(|&x| x != correct_door && x != choice)
        .map(|e| doors.remove(e));

    let final_choice: i8 = {
        if switch_doors {
            *doors.iter().find(|&&x| x != choice).unwrap()
        } else {
            choice
        }
    };

    (final_choice == correct_door, switch_doors)
}

pub fn play_threaded(iterations: u64) -> Results {
    let threads = num_cpus::get();

    let iterations_per_thread = iterations / threads as u64;
    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        let iters = iterations_per_thread.clone();
        handles.push(std::thread::spawn(move || play_multiple(iters)));
    }
    let mut results = Results::default();
    for handle in handles {
        results += handle.join().unwrap();
    }
    results
}

pub fn play_multiple(iterations: u64) -> Results {
    let mut results = Results::default();
    let now = now();
    let states = [now, now];
    let mut rng: Xorshift128 = SeedableRng::from_seed(&states[..]);
    for _ in 0..iterations {
        let (won, switched) = play_simple(&mut rng);
        if switched && won {
            results.switched.wins += 1;
        } else if switched {
            results.switched.losses += 1;
        } else if won {
            results.stayed.wins += 1;
        } else {
            results.stayed.losses += 1;
        }
    }
    results
}
