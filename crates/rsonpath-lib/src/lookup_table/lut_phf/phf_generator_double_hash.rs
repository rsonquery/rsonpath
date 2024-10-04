use crate::lookup_table::lut_phf::phf_shared;
use crate::lookup_table::lut_phf::phf_shared::{HashKey, PhfHash};

const DEFAULT_LAMBDA: usize = 1;
pub const FIXED_SEED: u64 = 1234567890;

// Trait to convert from usize to U
pub trait FromUsize {
    fn from_usize(val: usize) -> Self;
}

impl FromUsize for usize {
    fn from_usize(val: usize) -> Self {
        val
    }
}

impl FromUsize for u16 {
    fn from_usize(val: usize) -> Self {
        val as u16
    }
}

impl FromUsize for u64 {
    fn from_usize(val: usize) -> Self {
        val as u64
    }
}

pub struct HashState<U> {
    pub hash_key: HashKey,
    pub displacements: Vec<(u32, u32)>,
    pub map: Vec<U>,
}

impl<U: Copy> HashState<U> {
    pub fn get<T: ?Sized + PhfHash>(&self, key: &T) -> Option<U> {
        let hashes = phf_shared::hash(key, &self.hash_key);
        let index = phf_shared::get_index(&hashes, &self.displacements, self.map.len()) as usize;

        Some(self.map[index])
    }
}

pub fn try_generate_hash<H: PhfHash>(entries: &[H], key: HashKey) -> Option<HashState<usize>> {
    struct Bucket {
        idx: usize,
        keys: Vec<usize>,
    }

    let hashes: Vec<_> = entries.iter().map(|entry| phf_shared::hash(entry, &key)).collect();

    let buckets_len = (hashes.len() + DEFAULT_LAMBDA - 1) / DEFAULT_LAMBDA;
    let mut buckets = (0..buckets_len)
        .map(|i| Bucket { idx: i, keys: vec![] })
        .collect::<Vec<_>>();

    for (i, hash) in hashes.iter().enumerate() {
        buckets[(hash.g % (buckets_len as u32)) as usize].keys.push(i);
    }

    // Sort descending
    buckets.sort_by(|a, b| a.keys.len().cmp(&b.keys.len()).reverse());

    let table_len = hashes.len();
    let mut map: Vec<Option<usize>> = vec![None; table_len];
    let mut disps = vec![(0u32, 0u32); buckets_len];

    let mut try_map = vec![0u64; table_len];
    let mut generation = 0u64;
    let mut values_to_add = vec![];

    'buckets: for bucket in &buckets {
        for d1 in 0..(table_len as u32) {
            'disps: for d2 in 0..(table_len as u32) {
                values_to_add.clear();
                generation += 1;

                for &key in &bucket.keys {
                    let idx =
                        (phf_shared::displace(hashes[key].f1, hashes[key].f2, d1, d2) % (table_len as u32)) as usize;
                    if map[idx].is_some() || try_map[idx] == generation {
                        continue 'disps;
                    }
                    try_map[idx] = generation;
                    values_to_add.push((idx, key));
                }

                disps[bucket.idx] = (d1, d2);
                for &(idx, key) in &values_to_add {
                    map[idx] = Some(key);
                }
                continue 'buckets;
            }
        }

        return None;
    }

    Some(HashState {
        hash_key: key,
        displacements: disps,
        map: map.into_iter().map(|i| i.unwrap()).collect(),
    })
}
