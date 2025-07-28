use std::cell::{OnceCell, RefCell};
use eframe::emath::Align;
use eframe::epaint::Color32;
use eframe::Frame;
use egui::{Button, ComboBox, Context, Layout, RichText, ScrollArea, TextEdit, TopBottomPanel};
use rsonpath::*;
use std::sync::{Mutex, OnceLock};
use strip_ansi_escapes::strip;
use wasm_bindgen::prelude::*;
use web_sys::window;

pub struct WebsiteGui {
    json_input: String,
    query_input: String,
    json_output: String,
    console_output: String,
    is_dragging_file: bool,
    toggle_dark_mode_on: bool,
    toggle_console_on: bool,
    argument_verbose: bool,
    argument_compile: bool,
    argument_result_arg: ResultArg,
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
            argument_verbose: false,
            argument_compile: false,
            argument_result_arg: ResultArg::Nodes,
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
        use web_sys::{window, FileReader, HtmlInputElement};

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

        //Checks for imported files file
        #[cfg(target_arch = "wasm32")]
        if let Some(cell) = FILE_INPUT_REF.get() {
            if let Some(contents) = cell.lock().unwrap().take() {
                self.json_input = contents;
            }
        }

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

                        let json_box_height = screen_height * 0.65;

                        //Json input window
                        ScrollArea::vertical()
                            .id_salt("Json input window")
                            .max_height(json_box_height)
                            .show(ui, |ui| {
                                ui.add_sized(
                                    [left_side_width, json_box_height],
                                    TextEdit::multiline(&mut self.json_input).hint_text(hint_text),
                                );
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
                                //Arguments
                                ui.horizontal(|ui| {

                                    ui.vertical(|ui| {
                                        ui.checkbox(&mut self.argument_compile, "Compile only");
                                    });
                                });

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
                            });
                        });

                        ui.add_space(20.0);

                        let button_text = RichText::new("Run Query").size(20.0).color(Color32::WHITE).strong();

                        //Run button
                        if ui
                            .add_sized(
                                [left_side_width, screen_height * 0.144],
                                Button::new(button_text).fill(Color32::BROWN),
                            )
                            .clicked()
                        {
                            match run_with_args(&create_args(
                                self.query_input.clone(),
                                None,
                                Option::from(self.json_input.clone()),
                                self.argument_verbose,
                                self.argument_compile,
                                self.argument_result_arg,
                                None,
                            )) {
                                Ok(run_output) => {

                                    self.json_output = run_output.stdout;

                                    if let Some(benchmarks) = run_output.benchmark_stats {
                                        self.console_output = format!(
                                            "{}Benchmark Stats:\n\n\t- Parse time: {:?}\n\t- Compile time: {:?}\n\t- Run time: {:?}",
                                            run_output.stderr,
                                            benchmarks.parse_time,
                                            benchmarks.compile_time,
                                            benchmarks.run_time
                                        );
                                    } else {
                                        self.console_output = run_output.stderr;
                                    }

                                    if self.json_output.is_empty() && !self.argument_compile {
                                        self.console_output = String::from("ERROR: No result found. Please make sure all the variable names in your query are correct.")
                                    }
                                }
                                Err(e) => {
                                    self.json_output.clear();

                                    let report = format!("{:?}", eyre::Report::from(e));
                                    let stripped_bytes = strip(report.as_bytes());
                                    let cleaned_report = String::from_utf8_lossy(&stripped_bytes).into_owned();
                                    let no_location = strip_location_paragraph(&cleaned_report);

                                    self.console_output = strip_last_paragraph(&no_location);
                                }
                            };
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
            let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Foreground, egui::Id::new("drag_overlay")));
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

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {}
}
