pub fn print_estimation() {
    for bit in 32..64 {
        let max_idx = 2_usize.pow(bit);
        let max_json_size_in_bytes = max_idx;

        let human_readable = format_size(max_json_size_in_bytes);

        println!(
            "bit = {}, idx = {:>15}, json = {}",
            bit,
            format_number(max_idx),
            human_readable
        );
    }
}

/// Formats a byte size into KB, MB, GB, TB, etc.
fn format_size(bytes: usize) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];
    let mut size = bytes as f64;
    let mut unit = "B";

    for u in &units {
        unit = *u;
        if size < 1024.0 {
            break;
        }
        size /= 1024.0;
    }

    format!("{:.2} {}", size, unit)
}

/// Adds thousands separators to large numbers
fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let mut count = 0;

    for c in s.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push('.');
        }
        result.push(c);
        count += 1;
    }

    result.chars().rev().collect()
}
