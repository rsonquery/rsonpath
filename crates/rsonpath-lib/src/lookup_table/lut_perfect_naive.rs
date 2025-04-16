use super::LookUpTable;
use crate::{
    classification::{self, simd::Simd},
    input::{self, error, Input},
    lookup_table::pair_data,
};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::io::Write;

#[derive(Clone, Serialize, Deserialize)]
pub enum Entry {
    Number(usize),
    Bucket(Bucket),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LutPerfectNaive {
    buckets: Vec<Entry>,
    size: usize,
    cutoff: usize,
}

impl LookUpTable for LutPerfectNaive {
    #[inline]
    fn build(json_path: &str, cutoff: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::File::open(json_path).expect("Failed to open file");
        // SAFETY: We keep the file open throughout the entire duration.
        let input = unsafe { input::MmapInput::map_file(&file)? };
        let simd_c = classification::simd::configure();

        let lut_perfect_naive = classification::simd::config_simd!(simd_c => |simd| {
            classification::simd::dispatch_simd!(simd; input, simd, cutoff => fn<I, V>(
                input: I,
                simd: V,
                cutoff: usize,
            ) -> Result<LutPerfectNaive, error::InputError> where
            I: Input,
            V: Simd,{
                    let (keys, values) = pair_data::find_pairs_absolute::<I, V>(&input, simd, cutoff)?;
                    Ok(LutPerfectNaive::build_single(keys, values, cutoff))
                })
        });
        lut_perfect_naive.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }

    #[inline]
    #[must_use]
    fn get(&self, key: &usize) -> Option<usize> {
        match &self.buckets[key % self.size] {
            Entry::Number(v) => Some(*v),
            Entry::Bucket(bucket) => bucket.get(key),
        }
    }

    fn get_cutoff(&self) -> usize {
        self.cutoff
    }
}

impl LutPerfectNaive {
    #[inline]
    #[must_use]
    pub fn build_single(keys: Vec<usize>, values: Vec<usize>, cutoff: usize) -> Self {
        let size = keys.len() * 2;
        let mut helper_table = vec![vec![]; size];

        for (key, value) in keys.into_iter().zip(values.into_iter()) {
            let hash = key % size;
            helper_table[hash].push((key, value));
        }

        // Initialize with a default value, e.g., Entry::Number(0)
        let mut buckets = vec![Entry::Number(0); size];

        for (i, sub_table) in helper_table.into_iter().enumerate() {
            if !sub_table.is_empty() {
                if sub_table.len() == 1 {
                    let (_key, value) = sub_table[0];
                    buckets[i] = Entry::Number(value);
                } else {
                    let keys: Vec<usize> = sub_table.iter().map(|(k, _)| *k).collect();
                    let values: Vec<usize> = sub_table.iter().map(|(_, v)| *v).collect();
                    buckets[i] = Entry::Bucket(Bucket::new(&keys, &values));
                }
            }
        }

        Self { buckets, size, cutoff }
    }

    #[inline]
    pub fn serialize(&self, path: &str) -> std::io::Result<()> {
        let serialized_data = serde_cbor::to_vec(&self).expect("Serialize failed.");
        let mut file = File::create(path)?;
        file.write_all(&serialized_data)?;
        Ok(())
    }

    #[inline]
    pub fn serialize_to_json(&self, path: &str) -> std::io::Result<()> {
        let serialized_data = serde_json::to_vec(&self).expect("Serialize failed.");
        let mut file = File::create(path)?;
        file.write_all(&serialized_data)?;
        Ok(())
    }

    #[inline]
    pub fn deserialize(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        let deserialized: Self = serde_cbor::from_slice(&contents).expect("Deserialize: Data has no CBOR format.");
        Ok(deserialized)
    }

    #[inline]
    #[must_use]
    pub fn estimate_cbor_size(&self) -> usize {
        serde_cbor::to_vec(&self).expect("Failed to serialize to JSON.").len()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Bucket {
    elements: Vec<usize>,
    size: usize,
}

impl Bucket {
    #[inline]
    #[must_use]
    pub fn new(keys: &[usize], values: &[usize]) -> Self {
        let mut size = keys.len() * 2;

        let elements = loop {
            let mut new_elements = vec![usize::MAX; size]; // Use a placeholder value, e.g., usize::MAX
            let mut collision = false;

            for (key, value) in keys.iter().zip(values.iter()) {
                let hash = key % size;
                if new_elements[hash] != usize::MAX {
                    collision = true;
                    break;
                }
                new_elements[hash] = *value;
            }

            if !collision {
                break new_elements;
            }
            size *= 2;
        };

        Self { elements, size }
    }

    #[inline]
    #[must_use]
    pub fn get(&self, key: &usize) -> Option<usize> {
        let hash = key % self.size;
        let value = self.elements[hash];

        (value != usize::MAX).then_some(value)
    }

    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        // Size of the `Bucket` struct itself
        let mut total_size = std::mem::size_of::<Self>();

        // Add the capacity of the `elements` vector
        total_size += self.elements.capacity() * std::mem::size_of::<usize>();

        total_size
    }
}
