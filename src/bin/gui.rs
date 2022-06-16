use eframe::App;
use egui::{FontData, FontDefinitions, FontFamily, Layout, Ui, Vec2};
use std::sync::Once;

/// Showcase [`TextEdit`].
#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct TextEdit {
    pub text: String,
}
static one: Once = Once::new();

pub struct IpRangeTextEdit {
    pub text: String,
}

impl Default for IpRangeTextEdit {
    fn default() -> Self {
        Self {
            text: "Please input IP/subnet...".to_owned(),
        }
    }
}

impl Default for TextEdit {
    fn default() -> Self {
        Self {
            text: "Edit this text".to_owned(),
        }
    }
}

impl Demo for TextEdit {
    fn name(&self) -> &'static str {
        "🖹 TextEdit"
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.name())
            .open(open)
            .resizable(false)
            .show(ctx, |ui| {
                use View as _;
                self.ui(ui);
            });
    }
}

impl View for IpRangeTextEdit {
    fn ui(&mut self, ui: &mut egui::Ui) {}
}

impl View for TextEdit {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let text = &mut self.text;
        let layout = egui::Layout::top_down(egui::Align::Center);
        // ui.allocate_ui(ui.available_size(), |ui| {
        // ui.vertical_centered(|ui| {

        ui.vertical_centered(|ui| {
            ui.allocate_ui_with_layout(
                Vec2 { x: 0f32, y: 0f32 },
                Layout::left_to_right(),
                |ui: &mut Ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Advanced usage of ");
                    ui.code("TextEdit");
                    ui.label(".");
                },
            );
        });

        // });
        // });
        // let output = egui::TextEdit::multiline(text)
        //     .hint_text("Type something!")
        //     .show(ui);

        // ui.horizontal(|ui| {
        //     ui.spacing_mut().item_spacing.x = 0.0;
        //     ui.label("Selected text: ");
        //     if let Some(text_cursor_range) = output.cursor_range {
        //         use egui::TextBuffer as _;
        //         let selected_chars = text_cursor_range.as_sorted_char_range();
        //         let selected_text = text.char_range(selected_chars);
        //         ui.code(selected_text);
        //     }
        // });

        // let anything_selected = output
        //     .cursor_range
        //     .map_or(false, |cursor| !cursor.is_empty());

        // ui.add_enabled(
        //     anything_selected,
        //     egui::Label::new("Press ctrl+T to toggle the case of selected text (cmd+T on Mac)"),
        // );

        // if ui
        //     .input_mut()
        //     .consume_key(egui::Modifiers::COMMAND, egui::Key::T)
        // {
        //     if let Some(text_cursor_range) = output.cursor_range {
        //         use egui::TextBuffer as _;
        //         let selected_chars = text_cursor_range.as_sorted_char_range();
        //         let selected_text = text.char_range(selected_chars.clone());
        //         let upper_case = selected_text.to_uppercase();
        //         let new_text = if selected_text == upper_case {
        //             selected_text.to_lowercase()
        //         } else {
        //             upper_case
        //         };
        //         text.delete_char_range(selected_chars.clone());
        //         text.insert_text(&new_text, selected_chars.start);
        //     }
        // }

        // ui.horizontal(|ui| {
        //     ui.label("Move cursor to the:");

        //     if ui.button("start").clicked() {
        //         let text_edit_id = output.response.id;
        //         if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
        //             let ccursor = egui::text::CCursor::new(0);
        //             state.set_ccursor_range(Some(egui::text::CCursorRange::one(ccursor)));
        //             state.store(ui.ctx(), text_edit_id);
        //             ui.ctx().memory().request_focus(text_edit_id); // give focus back to the [`TextEdit`].
        //         }
        //     }

        //     if ui.button("end").clicked() {
        //         let text_edit_id = output.response.id;
        //         if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
        //             let ccursor = egui::text::CCursor::new(text.chars().count());
        //             state.set_ccursor_range(Some(egui::text::CCursorRange::one(ccursor)));
        //             state.store(ui.ctx(), text_edit_id);
        //             ui.ctx().memory().request_focus(text_edit_id); // give focus back to the [`TextEdit`].
        //         }
        //     }
        // });
    }
}

pub trait Demo {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}

/// Something to view in the demo windows
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

struct TestEditApp {
    text_edit: TextEdit,
    open: bool,
}

impl App for TestEditApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // self.text_edit.show(ctx, &mut true);
        // egui::SidePanel::left("my_left_panel").show(ctx, |ui| {});
        // egui::Window::new("test window")
        //     .open(&mut self.open)
        //     .resizable(true)
        //     .show(ctx, |ui| {
        //         ui.add(egui::Label::new("Hello World!"));
        //         ui.label("A shorter and more convenient way to add a label.");
        //         ui.horizontal(|ui| {
        //             ui.label("Add widgets");
        //             if ui.button("on the same row!").clicked() { /* … */ }
        //         });
        //     });
        // one.call_once(|| {
        //     let mut fonts = FontDefinitions::default();
        //     fonts.font_data.insert(
        //         "wenquan".to_owned(),
        //         FontData::from_static(include_bytes!("/tmp/wenquan.ttf")),
        //     );
        //     fonts
        //         .families
        //         .get_mut(&FontFamily::Proportional)
        //         .unwrap()
        //         .insert(0, "wenquan".to_owned());
        //     fonts
        //         .families
        //         .get_mut(&FontFamily::Monospace)
        //         .unwrap()
        //         .push("wenquan".to_owned());
        //     ctx.set_fonts(fonts);
        // });
        // egui::CentralPanel::default().show(ctx, |ui| {
        //     self.text_edit.ui(ui);
        // });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(
        "Edit Test App",
        options,
        Box::new(|_cc| {
            Box::new(TestEditApp {
                text_edit: TextEdit {
                    text: "abcdef".to_string(),
                },
                open: true,
            })
        }),
    );
}
