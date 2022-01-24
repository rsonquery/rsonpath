fn reverse(x: u64) -> u64 {
    let mut res = 0u64;
    for i in 0..64 {
        let bit = (x & (1 << i)) >> i;
        res |= bit << (63 - i);
    }
    res
}

#[macro_export]
macro_rules! bin {
    ($name: expr, $e:expr) => {
        log::debug!("{: >24}: {:064b} ({})", $name, reverse($e), $e);
    };
}
