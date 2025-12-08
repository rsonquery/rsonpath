use crate::ui::WebsiteGui;
use std::any::TypeId;
use std::borrow::Cow;
use std::ops::Range;

pub mod constants;
mod engine_run;
mod file_load;
pub mod message;
mod ui;
pub mod util;

pub fn start(cc: &eframe::CreationContext, worker: web_sys::Worker) -> WebsiteGui {
    WebsiteGui::new(cc, worker)
}

pub(crate) struct ReadOnlyTextBuffer<'a>(Cow<'a, str>);

impl<'a> ReadOnlyTextBuffer<'a> {
    pub(crate) fn empty() -> Self {
        Self(Cow::Borrowed(""))
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'a> egui::TextBuffer for ReadOnlyTextBuffer<'a> {
    fn is_mutable(&self) -> bool {
        false
    }

    fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    fn insert_text(&mut self, _text: &str, _char_index: usize) -> usize {
        0
    }

    fn delete_char_range(&mut self, _char_range: Range<usize>) {}

    fn type_id(&self) -> TypeId {
        TypeId::of::<ReadOnlyTextBuffer<'static>>()
    }
}
