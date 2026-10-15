use std::error::Error;

use clap::{FromArgMatches, Subcommand};
use eframe::egui;

use crate::{
    app_error::AppError,
    service::{Service, models::Request},
};

use super::{command::Command, models::Response};

pub fn run(mut service: impl Service + 'static) -> Result<(), AppError> {
    // Build forms from the CLI definitions so both interfaces share arguments and validation.
    let parser = Command::augment_subcommands(clap::Command::new("vis"));
    let actions: Vec<_> = parser
        .get_subcommands()
        .map(|command| {
            let mut label = command.get_name().replace('-', " ");
            label[..1].make_ascii_uppercase();
            (command.get_name().to_owned(), label)
        })
        .collect();
    let mut selected = actions
        .iter()
        .position(|(name, _)| name == "all-items")
        .expect("the initial action must exist in the command definitions");
    let mut values = Vec::new();
    let mut result_title = actions[selected].1.clone();
    let mut result = service
        .execute(Request::AllItems)
        .map(Response::from)
        .map(Response::text);
    let mut validation_error = String::new();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([850.0, 600.0])
            .with_min_inner_size([500.0, 400.0])
            // The window title identifies the app without a separate bundled icon.
            .with_icon(egui::IconData::default()),
        ..Default::default()
    };
    eframe::run_ui_native("Game library", options, move |ui, _frame| {
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let previous = selected;
                egui::ComboBox::from_label("Action")
                    .selected_text(&actions[selected].1)
                    .show_ui(ui, |ui| {
                        for (index, (_, label)) in actions.iter().enumerate() {
                            ui.selectable_value(&mut selected, index, label);
                        }
                    });
                if selected != previous {
                    values.clear();
                    validation_error.clear();
                }
                let command = parser.find_subcommand(&actions[selected].0)
                    .expect("the selected action comes from the command definitions");
                values.resize_with(command.get_positionals().count(), String::new);
                ui.add_space(8.0);
                egui::Grid::new(("fields", selected)).spacing([16.0, 8.0]).show(ui, |ui| {
                    for (index, argument) in command.get_positionals().enumerate() {
                        let mut label = argument.get_id().as_str()
                            .replace("_id", "_ID").replace('_', " ");
                        label[..1].make_ascii_uppercase();
                        if argument.get_id() == "release_date" {
                            label.push_str(" (YYYY-MM-DD)");
                        }
                        let label = ui.label(label);
                        if let Some(choices) = argument.get_value_parser().possible_values() {
                            egui::ComboBox::from_id_salt(("value", selected, index))
                                .selected_text(if values[index].is_empty() {
                                    "Select a value".to_owned()
                                } else {
                                    values[index].replace('_', " ")
                                })
                                .show_ui(ui, |ui| {
                                    for choice in choices {
                                        ui.selectable_value(&mut values[index], choice.get_name().to_owned(), choice.get_name().replace('_', " "));
                                    }
                                }).response.labelled_by(label.id);
                        } else {
                            // A grid otherwise shrinks empty text fields to its minimum column width.
                            ui.add_sized([260.0, 24.0], egui::TextEdit::singleline(&mut values[index]))
                                .labelled_by(label.id);
                        }
                        ui.end_row();
                    }
                });
                ui.add_space(8.0);
                let complete = values.iter().all(|value| !value.trim().is_empty());
                if ui.add_enabled(complete, egui::Button::new("Run")).clicked() {
                    match parse_form(&parser, command.get_name(), &values) {
                        Ok(command) => {
                            validation_error.clear();
                            result_title.clone_from(&actions[selected].1);
                            result = service.execute(command.into()).map(Response::from).map(Response::text);
                        }
                        Err(error) => {
                            validation_error = "Enter whole numbers for IDs, points, and playtime. Points and playtime must be zero or greater.".to_owned();
                            if cfg!(debug_assertions) {
                                validation_error.push_str(&format!("\nDetails: {error}"));
                            }
                        }
                    }
                }
                if !validation_error.is_empty() {
                    ui.colored_label(ui.visuals().error_fg_color, &validation_error);
                }
                ui.separator();
                ui.label(format!("Results: {result_title}"));
                match &result {
                    Ok(text) => {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            // A borrowed string keeps results selectable and copyable without allowing edits.
                            ui.add(egui::TextEdit::multiline(&mut text.as_str())
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY));
                        });
                    }
                    Err(error) => {
                        ui.colored_label(ui.visuals().error_fg_color, error.to_string());
                        if cfg!(debug_assertions)
                            && let Some(source) = error.source()
                        {
                            ui.label(format!("Details: {source:?}"));
                        }
                    }
                }
            });
        });
    }).map_err(|error| AppError::Failure {
        message: "Could not open the game library window. Check your display setup and try again.".to_owned(),
        source: error.to_string().into(),
    })
}

fn parse_form(
    parser: &clap::Command,
    action: &str,
    values: &[String],
) -> Result<Command, clap::Error> {
    // End option parsing so values such as "--help" remain literal form input.
    let arguments = ["vis", action, "--"]
        .into_iter()
        .chain(values.iter().map(String::as_str));
    let matches = parser.clone().try_get_matches_from(arguments)?;
    Command::from_arg_matches(&matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forms_cover_every_action_and_preserve_literal_input() {
        let parser = Command::augment_subcommands(clap::Command::new("vis"));
        let cases: &[(&str, &[&str])] = &[
            ("all-items", &[]),
            ("add-player", &["Alex Smith", "alex@example.test"]),
            ("players", &[]),
            ("add-game", &["--help", "2020-01-02"]),
            ("games", &[]),
            ("add-genre", &["Adventure"]),
            ("genres", &[]),
            ("tag-game", &["1", "2"]),
            ("add-to-library", &["1", "2"]),
            ("library", &["1"]),
            ("set-progress", &["1", "2", "playing", "120"]),
            (
                "add-achievement",
                &["2", "First step", "Finish the tutorial", "10"],
            ),
            ("achievements", &["2"]),
            ("unlock", &["1", "3"]),
            ("unlocked", &["1"]),
        ];
        assert_eq!(parser.get_subcommands().count(), cases.len());
        for (action, fields) in cases {
            let values: Vec<_> = fields.iter().map(|value| (*value).to_owned()).collect();
            let command = parse_form(&parser, action, &values).unwrap();
            assert_eq!(
                parser
                    .find_subcommand(action)
                    .unwrap()
                    .get_positionals()
                    .count(),
                values.len()
            );
            if let Command::AddGame { title, .. } = command {
                assert_eq!(title, "--help");
            }
        }
        for fields in [
            ["1", "2", "playing", "-1"],
            ["invalid", "2", "playing", "0"],
            ["1", "2", "unknown", "0"],
            ["1", "2", "playing", "2147483648"],
        ] {
            assert!(parse_form(&parser, "set-progress", &fields.map(str::to_owned)).is_err());
        }
    }
}
