#!/home/mat/.cargo/bin/nu

def main [n: int] {
    mut i = 0
    while $i < $n {
        /tmp/rqbase '$.products[*].videoChapters' ./crates/rsonpath-benchmarks/data/pison/bestbuy_short_record.json -rcount out> /dev/null
        $i += 1
    }
}
