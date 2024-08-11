use std::fmt::Error;
use std::io;
use std::process::Command;
use rand::Rng;
use std::fs::{self, File};
use std::io::{ Write};
use ratatui::{
    widgets::{ListState}
};
use aws_sdk_ssm::{
    types::ParameterMetadata,
};

pub mod ps_list_filter;
mod aws;

use ps_list_filter::user_input::{
    PsListFilterInput,
    InputMode::{
        Normal,
        Editing
    }
};

// ANCHOR: action
pub enum Action {
    Tick,
    Increment,
    Decrement,
    Quit,
    None,
}
// ANCHOR_END: action

#[derive(Debug, Default)]
pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<String>,
    pub last_selected: Option<usize>,
    pub ps_metadata: Vec<ParameterMetadata>,
    pub ps_values : Vec<String>
    
}

impl StatefulList {
    pub fn new() -> Self{
        Self { 
            state: ListState::default(), 
            items: vec![], 
            last_selected: None,
            ps_metadata: vec![],
            ps_values: vec![]
        }
    }

    pub fn next(&mut self) {

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };

        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        let offset = self.state.offset();
        self.last_selected = self.state.selected();
        self.state.select(None);
        *self.state.offset_mut() = offset;
    }
}

// ANCHOR: application
/// Application.
#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// counter
    pub counter: u8,
    pub parameter_store_names: StatefulList,
    pub scroll: u16,
    pub filter_ps_list : bool,
    pub ps_filter_data: PsListFilterInput,
}
// ANCHOR_END: application

// ANCHOR: application_impl
impl App {
    /// Constructs a new instance of [`App`].
    pub async fn new() -> Self {
        let mut state_full_list_set = StatefulList::new();

        match aws::parameter_store::fetch_ps().await {
            Ok((ps_metadata,ps_values)) => {
                state_full_list_set.ps_values = ps_values;
                for parameter_metadata in ps_metadata.iter() {
                    state_full_list_set.ps_metadata.push(parameter_metadata.clone());
                    match &parameter_metadata.name {
                        Some(name) => state_full_list_set.items.push(name.to_string()),
                        _ => panic!("error")
                    }
                }
            }
            _ => panic!("Error")
        };
        Self {
            parameter_store_names: state_full_list_set,
            should_quit: false,
            counter: 0,
            scroll: 0,
            filter_ps_list : false,
            ps_filter_data : PsListFilterInput::new(),
        }
    }


    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    pub fn get_selected_metadata(&self) -> &ParameterMetadata {
        let selected_ps_index = match self.parameter_store_names.state.selected() {
            Some(metadata) => metadata,
            None => 0
        };
    
        &self.parameter_store_names.ps_metadata[selected_ps_index]
    }

    pub fn get_selected_value(&self) -> &String {
        let selected_ps_index = match self.parameter_store_names.state.selected() {
            Some(metadata) => metadata,
            None => 0
        };
    
        &self.parameter_store_names.ps_values[selected_ps_index]
    }

    pub fn increment_scrol(&mut self){
        self.scroll += 1;
    }

    pub fn decrement_scrol(&mut self){
        if(self.scroll == 0){
            self.scroll = 0;
        }else{
            self.scroll -= 1;
        }
    }

    pub fn clear_scrol(&mut self){
        self.scroll = 0;
    }

    pub fn toggle_search(&mut self){
        if self.filter_ps_list{
            self.ps_filter_data.input_mode = Normal;
        }else {
            self.ps_filter_data.input_mode = Editing;
        }
        self.filter_ps_list = !self.filter_ps_list
    }

    fn generate_random_file_name(&self) -> String {
        let mut rng = rand::thread_rng();
        let random_string: String = (0..10)
            .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
            .collect();
        format!("/tmp/{}.txt", random_string) // Creating the file in /tmp directory
    }

    pub async fn launch_vim(&mut self) -> io::Result<()> {
        let temp_file_path = &self.generate_random_file_name();

        let mut file = File::create(temp_file_path)?;
        file.write_all(self.get_selected_value().as_bytes())?;
        drop(file);

        Command::new("vim")
            .arg(temp_file_path) // Specify the file you want to edit with Vim
            .status()?;

        let edited_value = fs::read_to_string(temp_file_path)?;

        let ps_metadata = self.get_selected_metadata();

        let parameter_name = match &ps_metadata.name {
            Some(my_ps_description) => my_ps_description,
            None => ""
        };

        aws::parameter_store::edit_ps_value(parameter_name, edited_value).await;
        Ok(())
    }
}
// ANCHOR_END: application_impl

// ANCHOR: application_test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_app_increment_counter() {
        let mut app = App::default();
        app.increment_counter();
        assert_eq!(app.counter, 1);
    }

    #[test]
    fn test_app_decrement_counter() {
        let mut app = App::default();
        app.decrement_counter();
        assert_eq!(app.counter, 0);
    }
}
// ANCHOR_END: application_test
