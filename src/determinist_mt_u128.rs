use std::collections::HashMap;
use std::ops::Range;
use std::sync::{mpsc, Arc, Barrier};
use std::thread;

use indicatif::style::ProgressStyle;
use indicatif::ProgressIterator;
use num::integer::Roots;

use super::determinist;

type RangeChannelU128 = (
    mpsc::Sender<(u128, Range<u128>)>,
    mpsc::Receiver<(u128, Range<u128>)>,
);

struct SearchDivisorThread {
    _handle: thread::JoinHandle<()>,
    sender: mpsc::Sender<(u128, Range<u128>)>,
    receiver: mpsc::Receiver<bool>,
    already_found: mpsc::Sender<()>,
}

impl SearchDivisorThread {
    fn new(barrier: Arc<Barrier>) -> Self {
        let (number_tx, number_rx): RangeChannelU128 = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();
        let (already_found_tx, already_found_rx) = mpsc::channel();
        let handle = thread::spawn(move || loop {
            let mut already_found_alert = already_found_rx.try_recv().is_ok();
            let (number, range_to_verify) = number_rx.recv().unwrap();
            let mut i = range_to_verify.start;
            let end = range_to_verify.end;

            barrier.wait();
            if i % 2 == 0 {
                if i == 2 && number % i == 0 {
                    result_tx.send(true).unwrap();
                    already_found_rx.recv().unwrap();
                    break;
                }
                i += 1;
            }

            while i < end && !already_found_alert {
                already_found_alert = already_found_rx.try_recv().is_ok();
                if number % i == 0 && !already_found_alert {
                    result_tx.send(true).unwrap();
                    already_found_rx.recv().unwrap();
                    break;
                }
                i += 2;
            }
            result_tx.send(false).unwrap();
        });
        Self {
            _handle: handle,
            sender: number_tx,
            receiver: result_rx,
            already_found: already_found_tx,
        }
    }
    fn found_alert(&self) {
        self.already_found.send(()).unwrap();
    }
    fn ask_if_is_divisible_by(&self, number: u128, range: Range<u128>) {
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
    is_prime_number_cache: HashMap<u128, bool>,
}

impl FindPrimePool {
    pub fn new(threads_number: usize) -> Self {
        let barrier = Arc::new(Barrier::new(threads_number));
        let threads = (0..threads_number)
            .map(|_| SearchDivisorThread::new(Arc::clone(&barrier)))
            .collect();
        let is_prime_number_cache = (0..4194304_u32)
            .progress_with_style(ProgressStyle::default_bar())
            .map(|val| (val.into(), determinist::is_prime(val.into())))
            .collect();

        FindPrimePool {
            threads,
            threads_number,
            is_prime_number_cache,
        }
    }
    fn found_alert(&self) {
        for thread in &self.threads {
            thread.found_alert();
        }
    }

    pub fn is_prime(&mut self, number: u128) -> bool {
        if self.is_prime_number_cache.contains_key(&number) {
            return *self.is_prime_number_cache.get(&number).unwrap();
        }

        let increment = ((number.sqrt()) + 1) / self.threads_number as u128;
        let mut start = 2;
        let mut end = increment;
        for (i, thread) in (0u128..).zip(self.threads.iter()) {
            if i == (self.threads_number - 1) as u128 {
                thread.ask_if_is_divisible_by(number, start..(number / 2));
            } else {
                thread.ask_if_is_divisible_by(number, start..end);
                start = (i + 1) * increment;
                end = start + increment;
            }
        }

        let mut results: Vec<Option<bool>> = (&self.threads).iter().map(|_| None).collect();
        let mut left = true;
        while left {
            for (result, thread) in results.iter_mut().zip(&self.threads) {
                if let Ok(val) = thread.answer() {
                    *result = Some(val);
                }
            }

            for result in results.iter() {
                match result {
                    Some(val) => {
                        if *val {
                            self.found_alert();
                            self.is_prime_number_cache.insert(number, false);
                            return false;
                        }
                    }
                    None => break,
                }
                left = false;
            }
        }

        self.found_alert();
        self.is_prime_number_cache.insert(number, true);
        true
    }
}
