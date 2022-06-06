use criterion::{
    measurement::{Measurement, ValueFormatter, WallTime},
    Throughput,
};
pub struct DecimalByteMeasurement(WallTime);

impl Default for DecimalByteMeasurement {
    fn default() -> Self {
        Self::new()
    }
}

impl DecimalByteMeasurement {
    pub fn new() -> Self {
        DecimalByteMeasurement(WallTime)
    }
}

impl Measurement for DecimalByteMeasurement {
    type Intermediate = <WallTime as Measurement>::Intermediate;

    type Value = <WallTime as Measurement>::Value;

    fn start(&self) -> Self::Intermediate {
        self.0.start()
    }

    fn end(&self, i: Self::Intermediate) -> Self::Value {
        self.0.end(i)
    }

    fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
        self.0.add(v1, v2)
    }

    fn zero(&self) -> Self::Value {
        self.0.zero()
    }

    fn to_f64(&self, value: &Self::Value) -> f64 {
        self.0.to_f64(value)
    }

    fn formatter(&self) -> &dyn ValueFormatter {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Multiple {
    One,
    Kilo,
    Mega,
    Giga,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Unit {
    Byte,
    Elem,
}

impl Multiple {
    fn denominator(&self) -> f64 {
        match *self {
            Multiple::One => 1.0,
            Multiple::Kilo => 1_000.0,
            Multiple::Mega => 1_000_000.0,
            Multiple::Giga => 1_000_000_000.0,
        }
    }
}

impl ValueFormatter for DecimalByteMeasurement {
    fn scale_values(&self, typical_value: f64, values: &mut [f64]) -> &'static str {
        self.0.formatter().scale_values(typical_value, values)
    }

    fn scale_throughputs(
        &self,
        typical_value: f64,
        throughput: &criterion::Throughput,
        values: &mut [f64],
    ) -> &'static str {
        use Multiple::*;
        use Throughput::*;
        use Unit::*;

        let (value, unit) = match *throughput {
            Bytes(bytes) => (bytes as f64, Byte),
            Elements(elements) => (elements as f64, Elem),
        };
        let per_second = value * (1e9 / typical_value);
        let multiple = if per_second >= 1e9 {
            Giga
        } else if per_second >= 1e6 {
            Mega
        } else if per_second >= 1e3 {
            Kilo
        } else {
            One
        };
        let denominator = multiple.denominator();

        for val in values {
            let per_second = value * (1e9 / *val);
            *val = per_second / denominator;
        }

        match (unit, multiple) {
            (Byte, One) => " B/s",
            (Byte, Kilo) => "KB/s",
            (Byte, Mega) => "MB/s",
            (Byte, Giga) => "GB/s",
            (Elem, One) => " elem/s",
            (Elem, Kilo) => "Kelem/s",
            (Elem, Mega) => "Melem/s",
            (Elem, Giga) => "Gelem/s",
        }
    }

    fn scale_for_machines(&self, values: &mut [f64]) -> &'static str {
        self.0.formatter().scale_for_machines(values)
    }
}
