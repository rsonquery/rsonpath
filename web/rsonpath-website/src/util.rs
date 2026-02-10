use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicU32, Ordering};
use wasm_bindgen::JsValue;

/// Thin wrapper over [`AtomicU32`] for [`f32`] values.
pub(crate) struct AtomicF32(AtomicU32);

impl AtomicF32 {
    pub const fn new(v: f32) -> Self {
        AtomicF32(AtomicU32::new(v.to_bits()))
    }

    pub fn get(&self, ordering: Ordering) -> f32 {
        f32::from_bits(self.0.load(ordering))
    }

    pub fn store(&self, v: f32, ordering: Ordering) {
        self.0.store(v.to_bits(), ordering);
    }
}

pub fn error_string(err: JsValue) -> String {
    return try_to_string(&err).unwrap_or_else(|| format!("{err:?}"));

    fn try_to_string(err: &JsValue) -> Option<String> {
        err.as_string().or_else(|| {
            let Ok(val) = js_sys::JSON::stringify(err) else {
                return None;
            };
            val.as_string()
        })
    }
}

pub struct DisplaySize(pub f64);

impl Display for DisplaySize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 < 1_000.0 {
            write!(f, "{} B", self.0)
        } else if self.0 < 1_000_000.0 {
            write!(f, "{:.2} KB", self.0 / 1_000.0)
        } else if self.0 < 1_000_000_000.0 {
            write!(f, "{:.2} MB", self.0 / 1_000_000.0)
        } else {
            write!(f, "{:.2} GB", self.0 / 1_000_000_000.0)
        }
    }
}
