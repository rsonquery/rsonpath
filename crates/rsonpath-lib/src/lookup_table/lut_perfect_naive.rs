#[derive(Clone)]
pub enum Entry {
    Number(usize),
    Bucket(Bucket),
}
#[derive(Clone)]
pub struct LutPerfectNaive {
    buckets: Vec<Option<Entry>>,
    size: usize,
}

impl LutPerfectNaive {
    pub fn init(keys: Vec<usize>, values: Vec<usize>) -> Self {
        let size = keys.len() * 2;
        let mut helper_table = vec![vec![]; size];

        for (key, value) in keys.iter().zip(values.iter()) {
            let hash = key % size;
            helper_table[hash].push((*key, *value));
        }

        let mut buckets = vec![None; size];

        for (i, sub_table) in helper_table.into_iter().enumerate() {
            if !sub_table.is_empty() {
                if sub_table.len() == 1 {
                    let (key, value) = sub_table[0];
                    buckets[i] = Some(Entry::Number(value));
                } else {
                    let keys: Vec<usize> = sub_table.iter().map(|(k, _)| *k).collect();
                    let values: Vec<usize> = sub_table.iter().map(|(_, v)| *v).collect();
                    buckets[i] = Some(Entry::Bucket(Bucket::new(keys, values)));
                }
            }
        }

        Self { buckets, size }
    }

    pub fn get(&self, key: usize) -> Option<usize> {
        match &self.buckets[key % self.size] {
            Some(Entry::Number(v)) => Some(*v),
            Some(Entry::Bucket(bucket)) => bucket.get(key),
            None => None,
        }
    }
}

#[derive(Clone)]
pub struct Bucket {
    elements: Vec<Option<usize>>,
    size: usize,
}

impl Bucket {
    pub fn new(keys: Vec<usize>, values: Vec<usize>) -> Self {
        let mut size = keys.len() * 2;

        let elements = loop {
            let mut arr = vec![None; size];
            let mut collision = false;

            for (key, value) in keys.iter().zip(values.iter()) {
                let hash = key % size;
                if arr[hash].is_some() {
                    collision = true;
                    break;
                }
                arr[hash] = Some(*value);
            }

            if !collision {
                break arr;
            }
            size *= 2;
        };

        Self { elements, size }
    }

    pub fn get(&self, key: usize) -> Option<usize> {
        let hash = key % self.size;
        self.elements[hash]
    }
}
