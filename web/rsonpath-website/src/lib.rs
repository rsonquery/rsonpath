use eframe::Frame;
use eframe::emath::Align;
use eframe::epaint::Color32;
use egui::{Button, ComboBox, Context, Layout, RichText, ScrollArea, TextEdit, TopBottomPanel, Vec2};
use eyre::Result;
use rsonpath::automaton::{Automaton, error::CompilerError};
use rsonpath::engine::{Compiler, Engine, RsonpathEngine};
use rsonpath::input::BorrowedBytes;
use rsonpath::result::MatchWriter;
use rsonpath_syntax::error::ParseError;
use rsonpath_syntax::{JsonPathQuery, ParserBuilder};
use std::cell::{OnceCell, RefCell};
use std::io::Write;
use std::sync::{Mutex, OnceLock};
use strip_ansi_escapes::strip;
use wasm_bindgen::prelude::*;
use web_sys::window;
use web_time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResultArg {
    /// Return only the number of matches.
    Count,
    /// Return a list of all bytes at which a match occurred.
    Indices,
    /// Returns the full text of the matched nodes.
    Nodes,
}

pub struct WebsiteGui {
    json_input: String,
    query_input: String,
    json_output: String,
    console_output: String,
    is_dragging_file: bool,
    toggle_dark_mode_on: bool,
    toggle_console_on: bool,
    argument_result_arg: ResultArg,
    benchmark_repetitions: usize,
    warmup: bool,
}

impl Default for WebsiteGui {
    fn default() -> Self {
        FILE_BUFFER.with(|cell| {
            cell.get_or_init(|| RefCell::new(None));
        });

        Self {
            json_input: String::new(),
            query_input: String::new(),
            json_output: String::new(),
            console_output: String::new(),
            is_dragging_file: false,
            toggle_dark_mode_on: true,
            toggle_console_on: true,
            argument_result_arg: ResultArg::Nodes,
            benchmark_repetitions: 1,
            warmup: true,
        }
    }
}

//File select for native version
#[cfg(not(target_arch = "wasm32"))]
impl WebsiteGui {
    fn open_file(&mut self, _ctx: &egui::Context) {
        if let Some(path) = rfd::FileDialog::new().add_filter("JSON", &["json"]).pick_file() {
            if let Ok(contents) = std::fs::read_to_string(path) {
                self.json_input = contents;
            } else {
                self.console_output = "Failed to read the selected file.".to_owned();
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
//For drag and dropping files
pub static JSON_INPUT_REF: OnceLock<Mutex<Option<String>>> = OnceLock::new();
//For importing files
pub static FILE_INPUT_REF: OnceLock<Mutex<Option<String>>> = OnceLock::new();

thread_local! {
    static FILE_BUFFER: OnceCell<RefCell<Option<(String, f64)>>> = OnceCell::new();
    static FILE_START: std::cell::Cell<f64> = std::cell::Cell::new(0.0);
}

//File select for web version
#[cfg(target_arch = "wasm32")]
impl WebsiteGui {
    pub fn open_file() {
        use wasm_bindgen::JsCast;
        use web_sys::{FileReader, HtmlInputElement, window};

        let document = window().unwrap().document().unwrap();
        let body = document.body().unwrap();

        let file_input = document
            .create_element("input")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        file_input.set_type("file");
        file_input.set_accept(".json");

        let reader = FileReader::new().unwrap();
        let reader_clone = reader.clone();

        // reader.onloadend callback
        let onloadend_cb = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            if let Ok(result) = reader_clone.result() {
                if let Some(text) = result.as_string() {
                    // Get elapsed time saved earlier in FILE_START
                    let elapsed = FILE_START.with(|start_cell| {
                        let start = start_cell.get();
                        window().unwrap().performance().unwrap().now() - start
                    });

                    FILE_BUFFER.with(|cell| {
                        if let Some(buf) = cell.get() {
                            *buf.borrow_mut() = Some((text, elapsed));
                        }
                    });
                }
            }
        }) as Box<dyn FnMut(_)>);
        reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
        onloadend_cb.forget();

        // Save the start time right before reading the file
        let file_input_clone = file_input.clone();
        let reader_clone = reader.clone();
        let onchange_cb = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            let files = file_input_clone.files().unwrap();
            if let Some(file) = files.get(0) {
                // Start the timer *right before* reading the file
                FILE_START.with(|cell| {
                    cell.set(window().unwrap().performance().unwrap().now());
                });

                reader_clone.read_as_text(&file).unwrap();
            }
        }) as Box<dyn FnMut(_)>);
        file_input.set_onchange(Some(onchange_cb.as_ref().unchecked_ref()));
        onchange_cb.forget();

        body.append_child(&file_input).unwrap();
        file_input.click();
    }
}

impl eframe::App for WebsiteGui {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Set image loaders.
        egui_extras::install_image_loaders(ctx);
        //Sets the font size for buttons and menu bodies, as well as the padding
        ctx.set_style({
            let mut style = (*ctx.style()).clone();
            style.spacing.button_padding = egui::vec2(12.0, 8.0);
            style.text_styles = [
                (
                    egui::TextStyle::Button,
                    egui::FontId::new(15.0, egui::FontFamily::Proportional),
                ),
                (
                    egui::TextStyle::Body,
                    egui::FontId::new(15.0, egui::FontFamily::Proportional),
                ),
            ]
            .into();
            style
        });

        //Checks for drag & dropped files
        #[cfg(target_arch = "wasm32")]
        if let Some(cell) = JSON_INPUT_REF.get() {
            if let Some(contents) = cell.lock().unwrap().take() {
                self.json_input = contents;
            }
        }

        if self.toggle_dark_mode_on {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        TopBottomPanel::top("menu bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
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
                        self.json_output.clear();
                        self.console_output.clear();

                        ui.close();
                    }

                    //Opens JSON file from computer
                    if ui.button("Open...").clicked() {
                        WebsiteGui::open_file();
                        ui.close();
                    }

                    ui.separator();

                    //Exports input text into a JSON file
                    if ui.button("Export to JSON").clicked() {
                        export_to_json(&self.json_input);
                        ui.close();
                    }
                });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    //Console toggle checkbox
                    let toggle_console_text = RichText::new("Toggle Console").color(Color32::GRAY).size(15.0).strong();

                    ui.checkbox(&mut self.toggle_console_on, toggle_console_text);

                    ui.add_space(10.0);

                    //Dark mode checkbox
                    let toggle_console_text = RichText::new("Toggle Dark Mode")
                        .color(Color32::GRAY)
                        .size(15.0)
                        .strong();

                    ui.checkbox(&mut self.toggle_dark_mode_on, toggle_console_text)
                });
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                let screen_width = ctx.screen_rect().width();
                let screen_height = ctx.screen_rect().height();

                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.vertical(|ui| {
                        let hint_text = RichText::new("Enter your json code here...")
                            .size(15.0)
                            .color(Color32::GRAY)
                            .strong();

                        let left_side_width = screen_width * 0.5;

                        let json_box_height = screen_height * 0.620;

                        //Json input window
                        ScrollArea::vertical()
                            .id_salt("Json input window")
                            .max_height(json_box_height)
                            .show(ui, |ui| {

                                //Truncates the input to the first 10,000 characters if imported file is > 10MB
                                if self.json_input.len() > 10_000_000 {
                                    let preview: String = "<JSON file too large to render>".to_string();
                                    ui.add_sized(
                                        [left_side_width, json_box_height],
                                        TextEdit::multiline(&mut preview.clone()).hint_text(hint_text).interactive(false),
                                    );
                                }
                                else {
                                    ui.add_sized(
                                        [left_side_width, json_box_height],
                                        TextEdit::multiline(&mut self.json_input).hint_text(hint_text)
                                    );
                                }
                            });

                        ui.add_space(15.0);

                        // Query input + arguments
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                let query_label = RichText::new("Enter your query here...")
                                    .size(15.0)
                                    .color(Color32::GRAY)
                                    .strong();

                                //Query input
                                ScrollArea::vertical()
                                    .id_salt("query_input_scroll")
                                    .max_height(ui.available_height())
                                    .show(ui, |ui| {
                                        ui.add_sized(
                                            [left_side_width * 0.75, screen_height * 0.077],
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
                                            .selected_text(format!("{:?}", self.argument_result_arg))
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(
                                                    &mut self.argument_result_arg,
                                                    ResultArg::Nodes,
                                                    "Nodes",
                                                );
                                                ui.selectable_value(
                                                    &mut self.argument_result_arg,
                                                    ResultArg::Count,
                                                    "Count",
                                                );
                                                ui.selectable_value(
                                                    &mut self.argument_result_arg,
                                                    ResultArg::Indices,
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
                                        .prefix("Repetitions: ")
                                );
                            });
                        });

                        ui.add_space(20.0);

                        let button_text = RichText::new("Run Query").size(20.0).color(Color32::WHITE).strong();

                        //Run button
                        if ui
                            .add_sized(
                                [left_side_width, screen_height * 0.144],
                                Button::new(button_text).fill(Color32::from_rgb(227, 60, 38)),
                            )
                            .clicked()
                        {
                            let mut total_parse = std::time::Duration::ZERO;
                            let mut total_compile = std::time::Duration::ZERO;
                            let mut total_run = std::time::Duration::ZERO;

                            let mut last_stdout = String::new();

                            for _ in 0..self.benchmark_repetitions {

                                if self.warmup {
                                    let _ = run(&self.query_input, &self.json_input, self.argument_result_arg);
                                    self.warmup = false;
                                }

                                match run(&self.query_input, &self.json_input, self.argument_result_arg) {
                                    Ok(run_output) => {
                                        last_stdout = run_output.stdout.clone();

                                        if let Some(benchmarks) = run_output.benchmark_stats {
                                            total_parse += benchmarks.parse_time;
                                            total_compile += benchmarks.compile_time;
                                            total_run += benchmarks.run_time;
                                        }
                                    }
                                    Err(e) => {
                                        self.json_output.clear();

                                        let report = format!("{:?}", eyre::Report::from(e));
                                        let stripped_bytes = strip(report.as_bytes());
                                        let cleaned_report =
                                            String::from_utf8_lossy(&stripped_bytes).into_owned();
                                        let no_location = strip_location_paragraph(&cleaned_report);

                                        self.console_output = strip_last_paragraph(&no_location);
                                        return;
                                    }
                                };
                            }

                            self.json_output = last_stdout;

                            if self.json_output.is_empty() {
                                self.console_output = String::from(
                                    "ERROR: No result found. Please make sure all the variable names in your query are correct."
                                );
                            } else {
                                self.console_output = format!(
                                    "Benchmark Stats{}:\n\n\t- Parse time: {:?}\n\t- Compile time: {:?}\n\t- Run time: {:?}",
                                    if self.benchmark_repetitions > 1 {
                                        format!(" (averaged over {} runs)", self.benchmark_repetitions)
                                    } else {
                                        "".to_string()
                                    },
                                    total_parse / self.benchmark_repetitions as u32,
                                    total_compile / self.benchmark_repetitions as u32,
                                    total_run / self.benchmark_repetitions as u32,
                                );
                            }
                        }
                    });

                    ui.add_space(10.0);

                    ui.vertical(|ui| {
                        //Output window
                        let output_window_height = if self.toggle_console_on {
                            screen_height * 0.50
                        } else {
                            screen_height
                        };
                        let output_window_width = screen_width * 0.48;

                        let query_output_hint_text = RichText::new("The result of your query will appear here...")
                            .size(15.0)
                            .color(Color32::GRAY)
                            .strong();

                        ScrollArea::vertical()
                            .id_salt("Query output window")
                            .max_height(output_window_height)
                            .show(ui, |ui| {
                                ui.add_sized(
                                    [output_window_width, output_window_height],
                                    TextEdit::multiline(&mut self.json_output)
                                        .desired_rows(5)
                                        .interactive(false)
                                        .hint_text(query_output_hint_text),
                                );
                            });

                        //Console window
                        if self.toggle_console_on {
                            ui.add_space(15.0);

                            let console_output_hint_text = RichText::new("Console output will appear here...")
                                .size(15.0)
                                .color(Color32::GRAY)
                                .strong();

                            ScrollArea::vertical()
                                .id_salt("Console output window")
                                .max_height(output_window_height)
                                .show(ui, |ui| {
                                    ui.add_sized(
                                        [output_window_width, screen_height * 0.40],
                                        TextEdit::multiline(&mut self.console_output)
                                            .interactive(false)
                                            .hint_text(console_output_hint_text),
                                    );
                                });
                        }
                    });
                })
            });
        });

        for file in &ctx.input(|i| i.raw.dropped_files.clone()) {
            if let Some(path_buf) = &file.path {
                if let Ok(contents) = std::fs::read_to_string(path_buf) {
                    self.json_input = contents;
                }
            }
        }

        FILE_BUFFER.with(|cell| {
            if let Some(buf) = cell.get() {
                if let Some((text, elapsed)) = buf.borrow_mut().take() {
                    self.json_input = text;
                    self.console_output = format!("File opened in {:.2} ms", elapsed);
                }
            }
        });

        //Checks if any files are dragged over and if yes, darkens the screen and displays a piece of text to confirm something happened
        let input = ctx.input(|i| i.clone());
        self.is_dragging_file = input.raw.hovered_files.len() > 0;

        if self.is_dragging_file {
            use egui::{Align2, Color32};

            let screen_rect = ctx.screen_rect();
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
}

//Removes the backtrace message from error
fn strip_last_paragraph(error: &str) -> String {
    let mut paragraphs: Vec<&str> = error.split("\n\n").collect();
    if paragraphs.len() > 1 {
        paragraphs.pop();
    }
    paragraphs.join("\n\n")
}

//Removes the location of the error (security risk)
fn strip_location_paragraph(error: &str) -> String {
    let mut result = String::new();
    let mut skip = false;

    for line in error.lines() {
        if line.trim().starts_with("Location:") {
            skip = true;
            continue;
        }

        if skip {
            if line.trim().is_empty() {
                skip = false;
            }
            continue;
        }

        result.push_str(line);
        result.push('\n');
    }

    result.trim_end().to_owned()
}

//Export to JSON for native version
#[cfg(not(target_arch = "wasm32"))]
fn export_to_json(json_input: &str) {
    if let Some(path) = rfd::FileDialog::new()
        .set_title("Save JSON File")
        .add_filter("JSON", &["json"])
        .save_file()
    {
        std::fs::write(path, json_input).unwrap_or_else(|err| {
            eprintln!("Failed to save file: {err}");
        });
    }
}

//Export to JSON for web version
#[cfg(target_arch = "wasm32")]
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

//Logic for enabling drag-and-dropping files into the app
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = handle_dropped_file)]
pub fn handle_dropped_file(contents: String) {
    let cell = JSON_INPUT_REF.get_or_init(|| Mutex::new(None));
    *cell.lock().unwrap() = Some(contents);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn register_file_handler() {
    let closure = Closure::wrap(Box::new(move |text: JsValue| {
        if let Some(text_str) = text.as_string() {
            handle_dropped_file(text_str);
        }
    }) as Box<dyn FnMut(JsValue)>);

    let window = window().expect("no global window");
    let func = closure.as_ref().unchecked_ref::<js_sys::Function>();

    js_sys::Reflect::set(&window, &JsValue::from_str("handle_dropped_file"), func)
        .expect("Failed to assign handle_dropped_file to window");

    closure.forget();
}

fn run(query: &str, json: &str, result_arg: ResultArg) -> Result<RunOutput> {
    // Benchmark parsing
    let parse_start = Instant::now();
    let query = parse_query(query)?;
    let parse_time = parse_start.elapsed();

    let compile_start = Instant::now();
    let automaton = compile_query(&query)?;
    let compile_time = compile_start.elapsed();

    let mut out = Vec::new();

    let engine = RsonpathEngine::from_compiled_query(automaton);
    let input = BorrowedBytes::new(json.as_bytes());
    let start = Instant::now();
    match result_arg {
        ResultArg::Count => {
            let result = engine.count(&input)?;
            write!(&mut out, "{result}")?;
        }
        ResultArg::Indices => {
            let mut sink = MatchWriter::from(&mut out);
            engine.indices(&input, &mut sink)?;
        }
        ResultArg::Nodes => {
            let mut sink = MatchWriter::from(&mut out);
            engine.matches(&input, &mut sink)?;
        }
    };
    let run_time = start.elapsed();

    Ok(RunOutput {
        stdout: String::from_utf8(out).expect("<Invalid UTF-8 in stdout>"),
        benchmark_stats: Some(BenchmarkStats {
            parse_time,
            compile_time,
            run_time,
        }),
    })
}

fn parse_query(query_string: &str) -> Result<JsonPathQuery, ParseError> {
    let mut parser_builder = ParserBuilder::default();
    parser_builder.allow_surrounding_whitespace(true);
    let parser: rsonpath_syntax::Parser = parser_builder.into();
    parser.parse(query_string)
}

fn compile_query(query: &JsonPathQuery) -> Result<Automaton, CompilerError> {
    Automaton::new(query)
}

pub struct BenchmarkStats {
    pub parse_time: Duration,
    pub compile_time: Duration,
    pub run_time: Duration,
}

pub struct RunOutput {
    pub stdout: String,
    pub benchmark_stats: Option<BenchmarkStats>,
}
