use std::collections::HashSet;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    num: u64,
}

fn find_primes_atkin(n: u64) -> Vec<u64> {
    let s: HashSet<u64> = vec![1, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 49, 53, 59]
        .into_iter()
        .collect();
    let mut a = vec![false; (n + 1) as usize];
    let sqrt_n = (n as f64).sqrt() as u64;

    // Step 1
    for x in 1..=sqrt_n {
        for y in (1..=sqrt_n).step_by(2) {
            let m = 4 * x * x + y * y;
            if m <= n && (m % 60 == 1 || m % 60 == 13 || m % 60 == 17 || m % 60 == 29 ||
                          m % 60 == 37 || m % 60 == 41 || m % 60 == 49 || m % 60 == 53) {
                a[m as usize] = !a[m as usize];
            }
        }
    }

    // Step 2
    for x in (1..=sqrt_n).step_by(2) {
        for y in (2..=sqrt_n).step_by(2) {
            let m = 3 * x * x + y * y;
            if m <= n && (m % 60 == 7 || m % 60 == 19 || m % 60 == 31 || m % 60 == 43) {
                a[m as usize] = !a[m as usize];
            }
        }
    }

    // Step 3
    for x in 2..=sqrt_n {
        for y in (1..x).rev().step_by(2) {
            let m = 3 * x * x - y * y;
            if m <= n && (m % 60 == 11 || m % 60 == 23 || m % 60 == 47 || m % 60 == 59) {
                a[m as usize] = !a[m as usize];
            }
        }
    }

    // Step 4
    let m: Vec<u64> = (0..=(n / 60))
        .flat_map(|w| s.iter().map(move |&s| 60 * w + s))
        .filter(|&m| m <= n)
        .collect();

    // Step 5
    for &am in &m {
        if am * am > n {
            break;
        }
        if a[am as usize] {
            let mm = am * am;
            m.iter().for_each(|&m2| {
                let c = mm * m2;
                if c > n {
                    return;
                }
                a[c as usize] = false;
            });
        }
    }

    // Collect primes
    let mut primes = vec![2, 3, 5];
    primes.extend(m.into_iter().filter(|&x| x > 5 && a[x as usize]));

    primes
}

fn main() {
    let args = Args::parse();
    let primes = find_primes_atkin(args.num);
    println!("Prime numbers up to {}: {:?}", args.num, primes);
}
