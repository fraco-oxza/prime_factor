use std::env;
use std::io::Write;

use log::{error, info, trace};
use rand::Rng;

mod determinist;
mod determinist_mt_u128;
mod probabilistic;

fn main() {
    if env::var("PF_LOG").is_err() {
        env::set_var("PF_LOG", "trace");
    }
    pretty_env_logger::init_custom_env("PF_LOG");

    let mut rng = rand::thread_rng();

    let mut find_prime_pool = determinist_mt_u128::FindPrimePool::new(num_cpus::get());

    loop {
        let mut line = String::new();
        print!("Introduzca el numero: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        let i: u128 = if line == "r" {
            rng.gen::<u64>() as u128
        } else if line == "lr" {
            rng.gen()
        } else {
            line.parse().unwrap_or_else(|error| {
                eprintln!("Error: {}", error);
                std::process::exit(127);
            })
        };

        info!("n: {}", i);

        let pti = std::time::Instant::now();
        let p = probabilistic::find_prime_factors(i);

        trace!("Checking primes");

        if p.iter().product::<u128>() == i {
            let val = p
                .iter()
                .map(|val| find_prime_pool.is_prime(*val))
                .filter(|result| !result)
                .count();

            if val == 0 {
                let pt = pti.elapsed().as_secs_f64();
                info!("Probabilist in {}s : ✓", pt);
            } else {
                let pt = pti.elapsed().as_secs_f64();
                error!("Probabilist in {}s : ✗", pt);
                std::process::exit(127);
            }
        } else {
            let pt = pti.elapsed().as_secs_f64();
            error!("Probabilist in {}s : ✗", pt);

            std::process::exit(127);
        }

        println!("{:?}", p);
    }
}
