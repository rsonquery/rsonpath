pub mod phf_generator;
pub mod phf_generator_double_hash;
pub mod phf_shared;

pub fn build_and_test() {
    let keys: Vec<usize> = vec![12, 231, 43213123, 321, 21232123123, 763, 213, 1, 2, 3, 453453, 53425];
    let values: Vec<&str> = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l"];

    // Generate hash state from the keys
    let hash_state = phf_generator::generate_hash(&keys);

    println!("{}", hash_state);

    println!("Index: (Key, Value)");
    for key in &keys {
        if let Some(index) = hash_state.get_index(key) {
            if index < values.len() {
                println!("  {} -> {} -> {})", index, key, values[index]);
            } else {
                println!("  Index {} out of bounds for key {}", index, key);
            }
        } else {
            println!("  Key {} not found", key);
        }
    }
}

pub fn build_and_test_large() {
    let keys: Vec<usize> = vec![
        12,
        231,
        43213123,
        321,
        21232123123,
        763,
        213,
        1,
        2,
        3,
        453453,
        53425,
        99323,
        1123,
        555,
        987654,
        87654,
        65432,
        12345,
        456789,
        987321,
        24680,
        13579,
        9999999,
    ];

    let values: Vec<bool> = vec![
        true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true,
        false, true, false, true, false, true, false,
    ];

    // Generate hash state from the keys
    let hash_state = phf_generator::generate_hash(&keys);

    println!("{}", hash_state);

    println!("Index: (Key, Value)");
    for key in &keys {
        if let Some(index) = hash_state.get_index(key) {
            if index < values.len() {
                println!("  {}: ({}, {})", index, key, values[index]);
            } else {
                println!("  Index {} out of bounds for key {}", index, key);
            }
        } else {
            println!("  Key {} not found", key);
        }
    }
}
