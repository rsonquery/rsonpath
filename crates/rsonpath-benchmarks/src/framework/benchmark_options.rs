use std::time::Duration;

use criterion::{measurement::Measurement, BenchmarkGroup};

pub(crate) struct BenchmarkOptions {
    pub(crate) warm_up_time: Option<Duration>,
    pub(crate) measurement_time: Option<Duration>,
    pub(crate) sample_count: Option<usize>,
}

impl BenchmarkOptions {
    pub(crate) fn apply_to<M: Measurement>(&self, group: &mut BenchmarkGroup<'_, M>) {
        if let Some(duration) = self.warm_up_time {
            group.warm_up_time(duration);
        }
        if let Some(duration) = self.measurement_time {
            group.measurement_time(duration);
        }
        if let Some(sample_count) = self.sample_count {
            group.sample_size(sample_count);
        }
    }
}
