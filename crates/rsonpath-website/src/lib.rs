use eframe::emath::Align;
use eframe::epaint::Color32;
use eframe::Frame;
use egui::{Button, Context, Layout, RichText, TextEdit, TopBottomPanel};
use wasm_bindgen::prelude::*;

pub struct WebsiteGui {
    json_input: String,
    query_input: String,
    json_output: String,
    console_output: String,
    toggle_dark_mode_on: bool,
    toggle_console_on: bool,
}

impl Default for WebsiteGui {
    fn default() -> Self {
        Self {
            json_input: String::new(),
            query_input: String::new(),
            json_output: String::new(),
            console_output: String::new(),
            toggle_dark_mode_on: true,
            toggle_console_on: true,
        }
    }
}

impl eframe::App for WebsiteGui {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        //Sets the font size for buttons and menu bodies, as well as the padding
        ctx.set_style({
            let mut style = (*ctx.style()).clone();
            style.spacing.button_padding = egui::vec2(12.0, 8.0); // Wider buttons
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

        if self.toggle_dark_mode_on {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        TopBottomPanel::top("menu bar").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                // File menu button
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        //TODO: Add functionality for New button
                        ui.close_menu();
                    }
                    if ui.button("Open...").clicked() {
                        //TODO: Add functionality for Open button
                        ui.close_menu();
                    }

                    ui.separator();

                    if ui.button("Export to JSON").clicked() {
                        //TODO: Add functionality for Export to JSON button
                        ui.close_menu();
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
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.vertical(|ui| {
                    let hint_text = RichText::new("Enter your json code here...")
                        .size(15.0)
                        .color(Color32::GRAY)
                        .strong();

                    //Json input window
                    ui.add_sized(
                        [ui.available_width() / 2.0, ui.available_height() / 1.5],
                        TextEdit::multiline(&mut self.json_input).hint_text(hint_text),
                    );

                    ui.add_space(10.0);

                    let query_field_text = RichText::new("Enter your query here...")
                        .size(15.0)
                        .color(Color32::GRAY)
                        .strong();

                    //Query input field
                    ui.add_sized(
                        [ui.available_width() / 2.0, ui.available_height() - 100.0],
                        TextEdit::singleline(&mut self.query_input).hint_text(query_field_text),
                    );

                    let button_text = RichText::new("Run Query").size(20.0).color(Color32::WHITE).strong();

                    ui.add_space(10.0);

                    //Run button
                    //TODO: Add button functionality
                    ui.add_sized(
                        [ui.available_width() / 2.0, ui.available_height()],
                        Button::new(button_text).fill(Color32::BROWN),
                    );
                });

                ui.add_space(10.0);

                ui.vertical(|ui| {
                    let mut output_window_height = if self.toggle_console_on {
                        ui.available_height() / 2.0
                    } else {
                        ui.available_height()
                    };

                    //Output window
                    ui.add_sized(
                        [ui.available_width(), output_window_height],
                        TextEdit::multiline(&mut self.json_output)
                            .desired_rows(5)
                            .interactive(false), //TODO: Change placeholder to result of query
                    );

                    //Console window
                    if self.toggle_console_on {
                        ui.add_space(10.0);

                        ui.add_sized(
                            [ui.available_width(), ui.available_height()],
                            TextEdit::multiline(&mut self.console_output).interactive(false),
                        );
                    }
                });
            })
        });
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {}
}
