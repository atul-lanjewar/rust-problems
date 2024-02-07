fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false; // 0 and 1 are not prime
    }
    if n <= 3 {
        return true; // 2 and 3 are prime
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false; // Even numbers except 2 and multiples of 3 are not prime
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}