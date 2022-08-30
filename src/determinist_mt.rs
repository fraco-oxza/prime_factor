use std::io::Write;
use std::ops::Range;
use std::sync::{mpsc, Arc, Barrier};
use std::thread;

use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_traits::{One, Zero};

struct SearchDivisorThread {
    handle: thread::JoinHandle<()>,
    sender: mpsc::Sender<(BigUint, Range<BigUint>)>,
    receiver: mpsc::Receiver<bool>,
    already_found: mpsc::Sender<()>,
}

impl SearchDivisorThread {
    fn new(barrier: Arc<Barrier>) -> Self {
        let (number_tx, number_rx): (
            mpsc::Sender<(BigUint, Range<BigUint>)>,
            mpsc::Receiver<(BigUint, Range<BigUint>)>,
        ) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();
        let (already_found_tx, already_found_rx) = mpsc::channel();
        let handle = thread::spawn(move || loop {
            let mut already_found_alert = match already_found_rx.try_recv() {
                Ok(_) => true,
                Err(_) => false,
            };
            let (number, range_to_verify) = number_rx.recv().unwrap();
            let mut i = range_to_verify.start;
            let end = range_to_verify.end;

            barrier.wait();
            while i < end && !already_found_alert {
                already_found_alert = match already_found_rx.try_recv() {
                    Ok(_) => true,
                    Err(_) => false,
                };
                if &number % &i == Zero::zero() && !already_found_alert {
                    result_tx.send(true).unwrap();
                    already_found_rx.recv().unwrap();
                    break;
                }
                i += 1u8;
            }
            result_tx.send(false).unwrap();
        });
        Self {
            handle,
            sender: number_tx,
            receiver: result_rx,
            already_found: already_found_tx,
        }
    }
    fn found_alert(&self) {
        self.already_found.send(()).unwrap();
    }
    fn ask_if_is_divisible_by(&self, number: BigUint, range: Range<BigUint>) {
        self.receiver.try_iter().for_each(|_| {});
        self.sender.send((number, range)).unwrap();
    }
    fn answer(&self) -> Result<bool, mpsc::TryRecvError> {
        self.receiver.try_recv()
    }
}

pub struct FindPrimePool {
    threads: Vec<SearchDivisorThread>,
    threads_number: usize,
}

impl FindPrimePool {
    pub fn new(threads_number: usize) -> Self {
        let barrier = Arc::new(Barrier::new(threads_number));
        let threads = (0..threads_number)
            .map(|_| SearchDivisorThread::new(Arc::clone(&barrier)))
            .collect();
        FindPrimePool {
            threads,
            threads_number,
        }
    }
    fn found_alert(&self) {
        for thread in &self.threads {
            thread.found_alert();
        }
    }
    pub fn is_prime(&self, number: BigUint) -> bool {
        let increment = (number.clone() / 2_u8.to_biguint().unwrap()) / self.threads_number;
        let mut start = (2_u8).to_biguint().unwrap();
        let mut end = increment.clone();
        let mut i = 0;
        for thread in &self.threads {
            if i == (self.threads_number - 1) {
                thread.ask_if_is_divisible_by(
                    number.clone(),
                    start.clone()..(number.clone() / 2_u8.to_biguint().unwrap()),
                );
            } else {
                thread.ask_if_is_divisible_by(number.clone(), start.clone()..end.clone());
                start = (i + 1).to_biguint().unwrap() * increment.clone();
                end = start.clone() + increment.clone();
            }
            i += 1;
        }

        let mut results: Vec<Option<bool>> = (&self.threads).iter().map(|_| None).collect();
        let mut left = true;
        while left {
            for (result, thread) in results.iter_mut().zip(&self.threads) {
                match thread.answer() {
                    Ok(val) => {
                        *result = Some(val);
                    }
                    Err(_) => {}
                };
            }

            for result in results.iter() {
                match result {
                    Some(val) => {
                        if *val {
                            self.found_alert();
                            return false;
                        }
                    }
                    None => break,
                }
                left = false;
            }
            // println!("{:?}", results);
        }

        self.found_alert();
        return true;
    }
}
