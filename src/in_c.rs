pub type NumSize = u64;

extern "C" {
    fn get_all_factors(number: cty::c_ulonglong, len: &cty::c_int) -> *const cty::c_ulonglong;
}

pub fn find_prime_factors(number: u128) -> Result<Vec<u128>, String> {
    let len = 0;
    let number: NumSize = match number.try_into() {
        Ok(val) => val,
        Err(e) => {
            return Err(format!("{}", e));
        }
    };
    let pointer_to_primes = unsafe { get_all_factors(number, &len) };
    let primes_slice = unsafe { std::slice::from_raw_parts(pointer_to_primes, (len) as usize) };
    Ok(primes_slice.iter().map(|val| *val as u128).collect())
}
