use std::io::Lines;
use std::fs::File;
use std::io::{self};
use super::file_service;
use super::super::models::app_state::AppState;
use super::super::models::data_type::*;

pub fn process_lines(lines: Lines<io::BufReader<File>>) -> Vec<String> {
    let mut app_state = AppState::new();
    for line in lines {
        if let Ok(data) = line {
            process_line(&mut app_state, data);
        }
    }

    app_state.display_data()
}

fn process_line(app_state: &mut AppState, data: String) {
    let line_tokens: Vec<&str> = data.split(" ").collect();

    match file_service::validate_line_type(&line_tokens) {
        DataType::Driver(driver_name) => {
            app_state.add_driver(&driver_name);
        },
        DataType::Trip(driver_name) => {
            app_state.process_trip(&driver_name, line_tokens);
        },
        DataType::Unknown => {
            println!("Unknown line type found while processing data!")
        },
    }

}