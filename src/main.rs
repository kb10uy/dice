use clap::Clap;
use rand::prelude::*;
use rayon::{iter::repeat, prelude::*};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::{prelude::*, stdout, BufWriter},
    str::FromStr,
    sync::{Arc, Mutex},
};

/// Rolls dice.
#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Arguments {
    /// Specifies the dice roll with `num-d-num` style.
    rolls: DiceRoll,

    /// Shows the all results of dice rolls.
    #[clap(short = "v", long = "verbose")]
    verbose: bool,
}

/// Represents dice roll specification.
struct DiceRoll {
    /// roll repetition count.
    repetitions: u64,

    /// The number of faces of virtual dice.
    faces: u64,
}

impl FromStr for DiceRoll {
    type Err = String;

    fn from_str(s: &str) -> Result<DiceRoll, String> {
        let numbers: Vec<_> = s.split('d').collect();
        if numbers.len() != 2 {
            return Err("Invalid specification format".into());
        }

        let repetitions = numbers[0].parse::<u64>().map_err(|e| e.to_string())?;
        let faces = numbers[1].parse::<u64>().map_err(|e| e.to_string())?;

        if faces != 0 {
            Ok(DiceRoll { repetitions, faces })
        } else {
            Err("Invalid number of faces".into())
        }
    }
}

impl Display for DiceRoll {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}d{}", self.repetitions, self.faces)
    }
}

fn main() {
    let arguments = Arguments::parse();

    if arguments.verbose {
        process_verbose(arguments.rolls);
    } else {
        process_sum(arguments.rolls);
    }
}

/// Rolls dice and shows the sum of them.
fn process_sum(roll: DiceRoll) {
    let sum: u64 = repeat(())
        .take(roll.repetitions as usize)
        .map(|_| thread_rng().gen_range(1, roll.faces + 1))
        .sum();

    println!("{}", sum);
}

/// Rolls dice and shows each roll.
fn process_verbose(roll: DiceRoll) {
    let stdout = Arc::new(Mutex::new(BufWriter::new(stdout())));

    repeat(stdout)
        .take(roll.repetitions as usize)
        .for_each(|w| {
            let roll = thread_rng().gen_range(1, roll.faces + 1);
            let mut locked = w.lock().expect("Failed to lock stdout");
            write!(locked, "{} ", roll).expect("IO Error");
        });
}
