#[macro_use]
extern crate lazy_static;
use std::{
    fs::File,
    io::{
        prelude::*,
        BufReader,
    },
    ops,
    path::PathBuf,
};

use clap::Parser;

fn digit_probability(digit: u32) -> f64 {
    let d = digit as f64;
    (1.0 + (1.0 / d)).log10()
}

lazy_static! {
    static ref TARGET_DIST: [f64; 9] = [
        digit_probability(1),
        digit_probability(2),
        digit_probability(3),
        digit_probability(4),
        digit_probability(5),
        digit_probability(6),
        digit_probability(7),
        digit_probability(8),
        digit_probability(9),
    ];
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum SignificanceLevel {
    Adot1,
    Adot05,
    Adot01,
}

impl SignificanceLevel {
    fn value(&self) -> f64 {
        match self {
            Self::Adot1 => 1.21,
            Self::Adot05 => 1.330,
            Self::Adot01 => 1.569,
        }
    }
    fn perc(&self) -> f64 {
        match self {
            Self::Adot1 => 0.1,
            Self::Adot05 => 0.05,
            Self::Adot01 => 0.01,
        }
    }
}

#[derive(Default, Debug)]
struct DigitCount {
    total: usize,
    d1: usize,
    d2: usize,
    d3: usize,
    d4: usize,
    d5: usize,
    d6: usize,
    d7: usize,
    d8: usize,
    d9: usize,
}

impl DigitCount {
    fn init(number: &str) -> Self {
        let mut dc = DigitCount::default();
        if let Some(digit) = number.chars().next() {
            dc.total += 1;
            match digit {
                '1' => dc.d1 += 1,
                '2' => dc.d2 += 1,
                '3' => dc.d3 += 1,
                '4' => dc.d4 += 1,
                '5' => dc.d5 += 1,
                '6' => dc.d6 += 1,
                '7' => dc.d7 += 1,
                '8' => dc.d8 += 1,
                '9' => dc.d9 += 1,
                _ => dc.total -= 1,
            }
        }
        dc
    }
    fn distribution(&self) -> [f64; 9] {
        [
            self.d1 as f64 / self.total as f64,
            self.d2 as f64 / self.total as f64,
            self.d3 as f64 / self.total as f64,
            self.d4 as f64 / self.total as f64,
            self.d5 as f64 / self.total as f64,
            self.d6 as f64 / self.total as f64,
            self.d7 as f64 / self.total as f64,
            self.d8 as f64 / self.total as f64,
            self.d9 as f64 / self.total as f64,
        ]
    }
    fn score(&self) -> f64 {
        // Based on this https://en.wikipedia.org/wiki/Benford's_law#Statistical_tests
        let distribution = self.distribution();
        let diff = vec![
            (distribution[0] - TARGET_DIST[0]).powi(2),
            (distribution[1] - TARGET_DIST[1]).powi(2),
            (distribution[2] - TARGET_DIST[2]).powi(2),
            (distribution[3] - TARGET_DIST[3]).powi(2),
            (distribution[4] - TARGET_DIST[4]).powi(2),
            (distribution[5] - TARGET_DIST[5]).powi(2),
            (distribution[6] - TARGET_DIST[6]).powi(2),
            (distribution[7] - TARGET_DIST[7]).powi(2),
            (distribution[8] - TARGET_DIST[8]).powi(2),
        ];
        ((self.total as f64) * diff.iter().sum::<f64>()).sqrt()
    }

    fn pass(&self, significance: SignificanceLevel) -> bool {
        self.score() < significance.value()
    }
}
impl ops::Add<DigitCount> for DigitCount {
    type Output = DigitCount;

    fn add(self, other: DigitCount) -> DigitCount {
        DigitCount {
            total: self.total + other.total,
            d1: self.d1 + other.d1,
            d2: self.d2 + other.d2,
            d3: self.d3 + other.d3,
            d4: self.d4 + other.d4,
            d5: self.d5 + other.d5,
            d6: self.d6 + other.d6,
            d7: self.d7 + other.d7,
            d8: self.d8 + other.d8,
            d9: self.d9 + other.d9,
        }
    }
}

fn get_counts(file: PathBuf) -> Result<DigitCount, std::io::Error> {
    let file = File::open(file)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents
        .split("\n")
        .map(|x| DigitCount::init(x))
        .fold(DigitCount::default(), |a, b| a + b))
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// csv to verify
    file: PathBuf,
    /// significance
    #[arg(value_enum, long, default_value_t = SignificanceLevel::Adot05)]
    signficance: SignificanceLevel,
}
fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let counts = get_counts(args.file)?;
    let dist = counts.distribution();

    println!("Leading digit distribution\n");
    println!("      |  1  |  2  |  3  |  4  |  5  |  6  |  7  |  8  |  9");
    println!(
        "TARGET|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}",
        TARGET_DIST[0],
        TARGET_DIST[1],
        TARGET_DIST[2],
        TARGET_DIST[3],
        TARGET_DIST[4],
        TARGET_DIST[5],
        TARGET_DIST[6],
        TARGET_DIST[7],
        TARGET_DIST[8]
    );

    println!(
        "FOUND |{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}",
        dist[0], dist[1], dist[2], dist[3], dist[4], dist[5], dist[6], dist[7], dist[8]
    );

    println!("\nSignificance level: {}", args.signficance.perc());
    println!("Found distance score: {}\n", counts.score());
    if counts.pass(args.signficance) {
        println!("✅ PASS!! data set seems to be natural!")
    } else {
        println!("❌ FAIL!! data set seems to be tampered with!")
    }

    println!("\nwarning: Make sure the category of data your testing follows Benford's law.");
    println!("https://en.wikipedia.org/wiki/Benford's_law");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fail() {
        let dc = get_counts(PathBuf::from("./data/random.txt")).unwrap();
        assert!(!dc.pass(SignificanceLevel::Adot01));
    }
    #[test]
    fn test_pass() {
        let dc = get_counts(PathBuf::from("./data/poweroftwo.txt")).unwrap();
        assert!(dc.pass(SignificanceLevel::Adot01));
    }
}
