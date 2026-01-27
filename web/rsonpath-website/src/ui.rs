use crate::ReadOnlyTextBuffer;
use crate::constants;
use crate::engine_run::{EngineRun, EngineRunState};
use crate::file_load::FileLoad;
use crate::file_load::FileLoadState;
use crate::message::{RunRsonpathMessageBuilder, RunRsonpathMode};
use crate::util::DisplaySize;
use eframe::Frame;
use eframe::emath::Align;
use eframe::epaint::Color32;
use eframe::epaint::text::TextWrapMode;
use egui::{
    Button, ComboBox, Layout, ProgressBar, RichText, ScrollArea, Separator, Spinner, TextEdit, TextStyle,
    TopBottomPanel, Vec2,
};
use egui_async::EguiAsyncPlugin;
use std::any::TypeId;
use std::borrow::Cow;
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, Worker, window};

#[derive(Clone)]
pub struct WebsiteGui(Rc<RefCell<WebsiteImpl>>);

pub struct WebsiteImpl {
    json_input: String,
    query_input: String,
    console_output: Console,
    is_dragging_file: bool,
    toggle_dark_mode_on: bool,
    toggle_console_on: bool,
    result_mode: RunRsonpathMode,
    benchmark_repetitions: usize,
    runner: Worker,
    file_load: FileLoad,
    file_input_element: HtmlInputElement,
    engine_run: EngineRun,
}

struct Console {
    buffer: String,
}

impl Console {
    pub fn new() -> Self {
        Self { buffer: String::new() }
    }

    pub fn log(&mut self, message: &str) {
        self.buffer.push_str(message);
        self.buffer.push('\n');
    }

    pub fn warn(&mut self, message: &str) {
        self.buffer.push_str("WARNING: ");
        self.buffer.push_str(message);
        self.buffer.push('\n');
    }

    pub fn error(&mut self, message: &str) {
        self.buffer.push_str("ERROR: ");
        self.buffer.push_str(message);
        self.buffer.push('\n');
    }

    pub fn clear(&mut self) {
        self.buffer.clear()
    }
}

impl egui::TextBuffer for Console {
    fn is_mutable(&self) -> bool {
        false
    }

    fn as_str(&self) -> &str {
        self.buffer.as_str()
    }

    fn insert_text(&mut self, _text: &str, _char_index: usize) -> usize {
        0
    }

    fn delete_char_range(&mut self, _char_range: Range<usize>) {}

    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

//File select for web version
#[cfg(target_arch = "wasm32")]
impl WebsiteGui {
    pub fn new(cc: &eframe::CreationContext, worker: Worker) -> Self {
        let document = window()
            .expect("window to be present")
            .document()
            .expect("document to be present");
        let file_input = document
            .get_element_by_id(constants::FILE_INPUT_ELEMENT_ID)
            .expect("file-input element not found, update FILE_INPUT_ELEMENT_ID")
            .dyn_into::<HtmlInputElement>()
            .unwrap();

        let inner = Rc::new(RefCell::new(WebsiteImpl {
            json_input: String::new(),
            query_input: String::new(),
            console_output: Console::new(),
            is_dragging_file: false,
            toggle_dark_mode_on: true,
            toggle_console_on: true,
            result_mode: RunRsonpathMode::Nodes,
            benchmark_repetitions: 1,
            runner: worker,
            file_input_element: file_input,
            file_load: FileLoad::new(),
            engine_run: EngineRun::new(),
        }));

        let inner_clone = inner.clone();
        let runner_clone = inner.borrow().runner.clone();
        let egui_ctx = cc.egui_ctx.clone();
        let onchange = Closure::wrap(Box::new(move |_: web_sys::Event| {
            let file = inner_clone.borrow().file_input_element.files().unwrap().get(0).unwrap();
            inner_clone
                .borrow()
                .file_load
                .request_async_load(file, egui_ctx.clone(), runner_clone.clone());
        }) as Box<dyn Fn(web_sys::Event)>);
        inner
            .borrow()
            .file_input_element
            .set_onchange(Some(onchange.as_ref().unchecked_ref()));
        onchange.forget();

        let canvas = document
            .get_element_by_id(constants::CANVAS_ELEMENT_ID)
            .expect("canvas element not found, update CANVAS_ELEMENT_ID")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("canvas to be a Canvas");
        let egui_ctx = cc.egui_ctx.clone();
        let ondragover = Closure::wrap(Box::new(move |e: web_sys::Event| {
            e.prevent_default();
            egui_ctx.request_repaint();
        }) as Box<dyn Fn(web_sys::Event)>);
        canvas.set_ondragover(Some(ondragover.as_ref().unchecked_ref()));
        ondragover.forget();
        let inner_clone = inner.clone();
        let runner_clone = inner.borrow().runner.clone();
        let egui_ctx = cc.egui_ctx.clone();
        let ondrop = Closure::wrap(Box::new(move |e: web_sys::DragEvent| {
            e.prevent_default();
            let file = e.data_transfer().unwrap().files().unwrap().get(0).unwrap();
            inner_clone
                .borrow()
                .file_load
                .request_async_load(file, egui_ctx.clone(), runner_clone.clone());
        }) as Box<dyn Fn(web_sys::DragEvent)>);
        canvas.set_ondrop(Some(ondrop.as_ref().unchecked_ref()));
        ondrop.forget();

        Self(inner)
    }
}

impl eframe::App for WebsiteGui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        self.0.borrow_mut().update(ctx, frame)
    }
}

impl eframe::App for WebsiteImpl {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        self.install_plugins(ctx);
        self.set_style(ctx);
        self.console_output.clear();

        self.show_menu_bar(ctx);
        self.show_central_panel(ctx);

        self.handle_file_drag(ctx);
    }
}

impl WebsiteImpl {
    fn install_plugins(&self, ctx: &egui::Context) {
        egui_extras::install_image_loaders(ctx);
        ctx.plugin_or_default::<EguiAsyncPlugin>();
    }

    fn set_style(&self, ctx: &egui::Context) {
        ctx.set_style({
            let mut style = (*ctx.style()).clone();
            style.spacing.button_padding = egui::vec2(12.0, 8.0);
            style.text_styles = [
                (
                    egui::TextStyle::Button,
                    egui::FontId::new(constants::FONT_SIZE_NORMAL, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Body,
                    egui::FontId::new(constants::FONT_SIZE_NORMAL, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Monospace,
                    egui::FontId::new(constants::FONT_SIZE_NORMAL, egui::FontFamily::Monospace),
                ),
            ]
            .into();
            style
        });

        if self.toggle_dark_mode_on {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }
    }

    fn highlight_color(&self) -> Color32 {
        if self.toggle_dark_mode_on {
            Color32::WHITE
        } else {
            Color32::DARK_GRAY
        }
    }

    fn text_on_highlight_color(&self) -> Color32 {
        if self.toggle_dark_mode_on {
            Color32::DARK_GRAY
        } else {
            Color32::WHITE
        }
    }

    fn show_menu_bar(&mut self, ctx: &egui::Context) {
        let highlight_color = self.highlight_color();
        let text_on_highlight_color = self.text_on_highlight_color();
        TopBottomPanel::top("menu bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                // rq logo
                let logo_src = egui::include_image!("../assets/rsonquery-rq-logo.svg");
                let logo_image = egui::Image::new(logo_src)
                    .max_height(100.0)
                    .fit_to_exact_size(Vec2::new(200.0, 40.0));
                ui.add(logo_image);

                // File menu button
                ui.menu_button("File", |ui| {
                    //Wipes all text windows to allow user to start anew
                    if ui.button("New").clicked() {
                        self.json_input.clear();
                        self.query_input.clear();

                        ui.close();
                    }

                    //Opens JSON file from computer
                    if ui.button("Open...").clicked() {
                        self.open_file();
                        ui.close();
                    }

                    ui.separator();

                    //Exports input text into a JSON file
                    if ui.button("Export to JSON").clicked() {
                        export_to_json(&self.json_input);
                        ui.close();
                    }
                });

                // File load status.
                let mut discard_button = None;
                self.file_load.with_state(|state| match state {
                    // When in progress, display progress bar and spinner.
                    FileLoadState::InProgress(in_progress) => {
                        let progress = in_progress.progress();
                        let percentage = (progress * 100.0) as u32;
                        let percentage_text = RichText::new(format!("{percentage}%")).color(text_on_highlight_color);
                        let progress_bar = ProgressBar::new(progress)
                            .text(percentage_text)
                            .fill(highlight_color)
                            .desired_height(30.0)
                            .desired_width(400.0)
                            .corner_radius(0.0);
                        let spinner = Spinner::new().color(highlight_color).size(30.0);
                        ui.add(progress_bar);
                        ui.add(spinner);
                    }
                    // When loaded, display file name, size, and discard button.
                    FileLoadState::Succeeded(success) => {
                        self.json_input = success.preview().to_string();
                        self.console_output
                            .log(&format!("Loading file succeeded in {:?}", success.elapsed()));

                        egui::Frame::NONE
                            .fill(highlight_color)
                            .inner_margin(Vec2::new(10.0, 0.0))
                            .outer_margin(Vec2::new(0.0, 4.0))
                            .show(ui, |ui| {
                                ui.add(
                                    egui::Label::new(
                                        RichText::new(success.file().name().to_string()).color(text_on_highlight_color),
                                    )
                                    .wrap_mode(TextWrapMode::Truncate),
                                );
                                ui.add(egui::Label::new(
                                    RichText::new(format!("({})", DisplaySize(success.file().size())))
                                        .color(text_on_highlight_color),
                                ));
                                discard_button = Some(ui.button("❌").on_hover_text("Discard file."));
                            });
                    }
                    FileLoadState::Failed(failure) => {
                        self.console_output
                            .error(&format!("Loading file failed: {}", failure.error()));
                    }
                    FileLoadState::Idle | FileLoadState::Requested(_) => (),
                    FileLoadState::None => unreachable!("FileLoadState::None must not happen"),
                });

                if discard_button.is_some_and(|r| r.clicked()) {
                    self.file_load.discard(ctx, &self.runner);
                    self.json_input.clear();
                }

                // The buttons to the right: console and dark mode.
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    // Export output button
                    self.engine_run.with_state(|state| {
                        if let EngineRunState::Succeeded(success) = state {
                            // Save results to the computer.
                            if ui.button("Download results").clicked() {
                                export_to_json(success.results());
                            }
                        }
                    });

                    //Console toggle checkbox
                    let toggle_console_text = RichText::new("Toggle Console")
                        .color(ui.visuals().text_color())
                        .size(constants::FONT_SIZE_NORMAL)
                        .strong();

                    ui.checkbox(&mut self.toggle_console_on, toggle_console_text);

                    ui.add_space(10.0);

                    //Dark mode checkbox
                    let toggle_console_text = RichText::new("Toggle Dark Mode")
                        .color(ui.visuals().text_color())
                        .size(constants::FONT_SIZE_NORMAL)
                        .strong();

                    ui.checkbox(&mut self.toggle_dark_mode_on, toggle_console_text)
                });
            })
        });
    }

    fn show_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                let screen_width = ctx.content_rect().width();
                let screen_height = ctx.content_rect().height();

                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.vertical(|ui| {
                        let hint_text = RichText::new("Enter the JSON here...")
                            .size(constants::FONT_SIZE_NORMAL)
                            .color(ui.visuals().weak_text_color())
                            .strong();

                        let left_side_width = screen_width * 0.5;

                        let json_box_height = screen_height * 0.620;

                        //Json input window
                        ScrollArea::vertical()
                            .id_salt("Json input window")
                            .max_height(json_box_height)
                            .show(ui, |ui| {
                                self.file_load.with_state(|state| match state {
                                    FileLoadState::Requested(_)
                                    | FileLoadState::Succeeded(_)
                                    | FileLoadState::InProgress(_) => {
                                        ui.add_enabled_ui(false, |ui| {
                                            ui.add_sized(
                                                [left_side_width, json_box_height],
                                                TextEdit::multiline(&mut self.json_input)
                                                    .hint_text(hint_text)
                                                    .interactive(false),
                                            );
                                        });
                                    }
                                    FileLoadState::Failed(_) | FileLoadState::Idle | FileLoadState::None => {
                                        ui.add_sized(
                                            [left_side_width, json_box_height],
                                            TextEdit::multiline(&mut self.json_input).hint_text(hint_text),
                                        );
                                    }
                                });
                            });

                        ui.add_space(15.0);

                        // Query input + arguments
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                let query_label = RichText::new("Enter your query here...")
                                    .size(constants::FONT_SIZE_NORMAL)
                                    .color(ui.visuals().weak_text_color())
                                    .strong();

                                //Query input
                                ScrollArea::vertical()
                                    .id_salt("query_input_scroll")
                                    .max_height(ui.available_height())
                                    .show(ui, |ui| {
                                        ui.add_sized(
                                            [left_side_width * 0.75, ui.available_height()],
                                            TextEdit::multiline(&mut self.query_input)
                                                .hint_text(query_label)
                                                .desired_rows(1),
                                        );
                                    });
                            });

                            ui.add_space(10.0);

                            ui.vertical(|ui| {
                                ui.add_space(10.0);

                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ComboBox::from_label("Result Mode")
                                            .selected_text(format!("{:?}", self.result_mode))
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(
                                                    &mut self.result_mode,
                                                    RunRsonpathMode::Nodes,
                                                    "Nodes",
                                                );
                                                ui.selectable_value(
                                                    &mut self.result_mode,
                                                    RunRsonpathMode::Count,
                                                    "Count",
                                                );
                                                ui.selectable_value(
                                                    &mut self.result_mode,
                                                    RunRsonpathMode::Indices,
                                                    "Indices",
                                                );
                                            });
                                    });
                                });

                                ui.add_space(10.0);

                                //Repeat query x times
                                ui.add(
                                    egui::DragValue::new(&mut self.benchmark_repetitions)
                                        .speed(1)
                                        .range(1..=10000)
                                        .prefix("Repetitions: "),
                                );
                            });
                        });

                        ui.add_space(20.0);

                        let runnable = self.engine_run.with_state(|state| {
                            matches!(
                                state,
                                EngineRunState::Idle | EngineRunState::Succeeded(_) | EngineRunState::Failed(_)
                            )
                        });
                        let is_file_loading = self.file_load.with_state(|state| {
                            matches!(state, FileLoadState::Requested(_) | FileLoadState::InProgress(_))
                        });

                        let button = if !runnable {
                            let button_text = RichText::new("Running...").size(20.0).color(Color32::WHITE);
                            Button::new(button_text).fill(Color32::GRAY)
                        } else if is_file_loading {
                            let button_text = RichText::new("Loading...").size(20.0).color(Color32::WHITE);
                            Button::new(button_text).fill(Color32::GRAY)
                        } else {
                            let button_text = RichText::new("Run Query").size(20.0).color(Color32::WHITE).strong();
                            Button::new(button_text).fill(constants::RQ_COLOR)
                        };

                        let response = ui.add_enabled_ui(runnable, |ui| {
                            ui.add_sized([left_side_width, screen_height * 0.144], button)
                        });

                        if response.inner.clicked() {
                            assert!(runnable);
                            let file_is_loaded = self
                                .file_load
                                .with_state(|state| matches!(state, FileLoadState::Succeeded(_)));
                            let mut msg = if file_is_loaded {
                                RunRsonpathMessageBuilder::new_file(self.query_input.clone())
                            } else {
                                RunRsonpathMessageBuilder::new_inline(self.query_input.clone(), self.json_input.clone())
                            };
                            if self.benchmark_repetitions > 1 {
                                msg.benchmark(self.benchmark_repetitions);
                            }
                            msg.result_mode(self.result_mode);
                            self.engine_run
                                .request_async_run(msg.into(), ctx.clone(), self.runner.clone());
                        }
                    });
                    ui.add(Separator::default().vertical());

                    self.engine_run.with_state(|state| {
                        let mut out_buffer = ReadOnlyTextBuffer::empty();
                        match state {
                            EngineRunState::Idle | EngineRunState::InProgress(_) | EngineRunState::Requested(_) => {}
                            EngineRunState::Succeeded(success) => {
                                out_buffer = if success.results().len() > constants::MAX_OUTPUT_BYTES {
                                    ReadOnlyTextBuffer(Cow::Owned(format!(
                                        "<Result set is too large to show ({}), use the download button above>",
                                        DisplaySize(success.results().len() as f64)
                                    )))
                                } else {
                                    ReadOnlyTextBuffer(Cow::Borrowed(success.results()))
                                };
                                if out_buffer.is_empty() {
                                    self.console_output.warn("Result set is empty");
                                }
                                let mut message = "Runtime stats".to_string();
                                if self.benchmark_repetitions > 1 {
                                    message += &format!(" (averaged over {} runs)", self.benchmark_repetitions);
                                }
                                message += "\n\n";
                                message += &format!(
                                    "\t- Parse time: {:?}\n\t- Compile time: {:?}\n\t- Run time: {:?}",
                                    success.runtime().avg_parse_time(),
                                    success.runtime().avg_compile_time(),
                                    success.runtime().avg_run_time(),
                                );
                                self.console_output.log(&message);
                            }
                            EngineRunState::Failed(failure) => {
                                self.console_output.error(failure.error());
                            }
                            EngineRunState::None => unreachable!("EngineRunState::None cannot happen"),
                        }

                        ui.vertical(|ui| {
                            //Output window
                            let output_window_height = if self.toggle_console_on {
                                screen_height * 0.70
                            } else {
                                screen_height
                            };
                            let output_window_width = screen_width * 0.48;

                            let query_output_hint_text = RichText::new("The result of your query will appear here...")
                                .size(constants::FONT_SIZE_NORMAL)
                                .color(ui.visuals().weak_text_color())
                                .strong();

                            ui.label("Output");
                            ScrollArea::vertical()
                                .id_salt("Query output window")
                                .max_height(output_window_height)
                                .show(ui, |ui| {
                                    ui.add_sized(
                                        [output_window_width, output_window_height],
                                        TextEdit::multiline(&mut out_buffer)
                                            .desired_rows(5)
                                            .interactive(true)
                                            .hint_text(query_output_hint_text),
                                    );
                                });

                            //Console window
                            if self.toggle_console_on {
                                ui.add_space(15.0);
                                ui.separator();

                                let console_output_hint_text = RichText::new("Diagnostics will appear here...")
                                    .size(constants::FONT_SIZE_NORMAL)
                                    .color(ui.visuals().weak_text_color())
                                    .strong();

                                ui.label("Console");
                                ScrollArea::vertical()
                                    .id_salt("Console output window")
                                    .max_height(output_window_height)
                                    .show(ui, |ui| {
                                        ui.add_sized(
                                            [output_window_width, screen_height * 0.20],
                                            TextEdit::multiline(&mut self.console_output)
                                                .font(TextStyle::Monospace)
                                                .interactive(true)
                                                .hint_text(console_output_hint_text),
                                        );
                                    });
                            }
                        });
                    });
                })
            });
        });
    }

    fn handle_file_drag(&mut self, ctx: &egui::Context) {
        //Checks if any files are dragged over and if yes, darkens the screen and displays a piece of text to confirm something happened
        let input = ctx.input(|i| i.clone());
        self.is_dragging_file = !input.raw.hovered_files.is_empty();

        if self.is_dragging_file {
            use egui::{Align2, Color32};

            let screen_rect = ctx.content_rect();
            let painter = ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::new("drag_overlay"),
            ));
            painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(180));

            painter.text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                "📂 Drop your file here",
                egui::FontId::proportional(32.0),
                Color32::WHITE,
            );
        }
    }

    fn open_file(&self) {
        self.file_input_element.click();
    }
}

fn export_to_json(json_input: &str) {
    use wasm_bindgen::JsCast;
    use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // Create JSON blob
    let array = js_sys::Array::new();
    array.push(&JsValue::from_str(json_input));

    let bag = BlobPropertyBag::new();
    bag.set_type("application/json");

    let blob = Blob::new_with_str_sequence_and_options(&array, &bag).expect("Failed to create Blob");

    let url = Url::create_object_url_with_blob(&blob).expect("Failed to create URL");

    // Create invisible link
    let link = document
        .create_element("a")
        .unwrap()
        .dyn_into::<HtmlAnchorElement>()
        .unwrap();
    link.set_href(&url);
    link.set_download("data.json");
    link.style().set_property("display", "none").unwrap();

    // Append link to DOM *before* clicking
    body.append_child(&link).unwrap();
    link.click();
    body.remove_child(&link).unwrap(); // Clean up
    Url::revoke_object_url(&url).unwrap(); // Free memory
}
