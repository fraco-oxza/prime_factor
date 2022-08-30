use std::cell::RefCell;
use std::io::Write;
use std::sync::{Arc, Barrier, Mutex};

use log::{debug, error, info, trace, warn};
use num_bigint::BigUint;
use num_bigint::ToBigUint;
use rand::Rng;

mod determinist;
mod determinist_mt_u128;
mod in_c;
mod probabilistic;

fn main() {
    pretty_env_logger::init();

    let mut rng = rand::thread_rng();

    let find_prime_pool = Arc::new(Mutex::new(RefCell::new(
        determinist_mt_u128::FindPrimePool::new(12),
    )));

    loop {
        let fpp = Arc::clone(&find_prime_pool);
        let mut line = String::new();
        print!("Introduzca el numero: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.trim().to_string();
        let i: u128 = if line == "r" {
            rng.gen::<in_c::NumSize>() as u128
        } else if line == "lr" {
            rng.gen()
        } else {
            line.parse().unwrap()
        };

        info!("n: {}", i);

        let barrier = Arc::new(Barrier::new(3));

        let b = Arc::clone(&barrier);
        let dthread = std::thread::spawn(move || {
            b.wait();
            let dti = std::time::Instant::now();
            let d = determinist::find_prime_factors(i);
            let dt = dti.elapsed().as_secs_f64();

            if d.iter().product::<u128>() == i {
                info!("Determinist in {}s : ✓", dt);
            } else {
                info!("Determinist in {}s : ✗", dt);
            }
            d
        });
        let b = Arc::clone(&barrier);
        let pthread = std::thread::spawn(move || {
            b.wait();
            let pti = std::time::Instant::now();
            let p = probabilistic::find_prime_factors(i);

            if p.iter().product::<u128>() == i {
                let val = p
                    .iter()
                    .map(|val| {
                        if *val > 16384 {
                            fpp.lock().unwrap().borrow().is_prime(*val)
                        } else {
                            determinist::is_prime(*val)
                        }
                    })
                    .filter(|result| !result)
                    .count();

                if val == 0 {
                    let pt = pti.elapsed().as_secs_f64();
                    info!("Probabilist in {}s : ✓", pt);
                } else {
                    let pt = pti.elapsed().as_secs_f64();
                    info!("Probabilist in {}s : ✗", pt);
                }
            } else {
                let pt = pti.elapsed().as_secs_f64();
                info!("Probabilist in {}s : ✗", pt);
            }
            p
        });
        let b = Arc::clone(&barrier);
        let cthread = std::thread::spawn(move || {
            b.wait();
            let cti = std::time::Instant::now();
            let c = match in_c::find_prime_factors(i) {
                Ok(val) => val,
                Err(_e) => {
                    error!("the number is too large for c");
                    return vec![];
                }
            };
            let ct = cti.elapsed().as_secs_f64();

            if c.iter().product::<u128>() == i {
                info!("Determinist C in {}s : ✓", ct);
            } else {
                info!("Determinist C in {}s : ✗", ct);
            }
            c
        });

        let d = dthread.join().unwrap();
        let p = pthread.join().unwrap();
        let c = cthread.join().unwrap();

        if (d == p) && (d == c) {
            println!("{:?}", d);
        } else {
            println!("Determinist C say {:?}", c);
            println!("Determinist say {:?}", d);
            println!("Probabilist say {:?}", p);
        }
        println!();
    }
}
