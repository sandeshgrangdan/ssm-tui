use std::collections::HashMap;
use std::{
    io::{self, Write},
    process::Command,
    fs::{self, File},
};
use rand::Rng;
use ratatui::widgets::ListState;
use aws_sdk_ssm::types::ParameterMetadata;

pub mod ps_list_filter;
mod aws;

use ps_list_filter::user_input::{
    PsListFilterInput,
    InputMode::{
        Normal,
        Editing
    }
};
use clap::{Parser};
use aws_sdk_ssm::Client;

// ANCHOR_END: action

#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of your AWS profile.
    #[arg(short, long, default_value_t = String::from("None"))]
    profile: String,

    /// AWS Region.
    #[arg(short, long, default_value_t = String::from("None"))]
    region: String,
}


#[derive(Debug, Clone)]
pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<String>,
    pub display_items: Vec<String>,
    pub last_selected: Option<usize>,
    pub ps_metadata: HashMap<String, ParameterMetadata>,
    pub ps_values : HashMap<String, String>
}

impl StatefulList {
    pub fn new() -> Self{
        Self { 
            state: ListState::default(), 
            items: vec![],
            display_items: vec![],
            last_selected: None,
            ps_metadata: HashMap::new(),
            ps_values: HashMap::new()
        }
    }

    pub fn next(&mut self) {

        if self.display_items.len() > 0 {
            let i = match self.state.selected() {
                Some(i) => {
                    if i >= self.display_items.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => self.last_selected.unwrap_or(0),
            };
    
            self.state.select(Some(i));
        }
    }

    pub fn previous(&mut self) {
        if self.display_items.len() > 0 {
            let i = match self.state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.display_items.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => self.last_selected.unwrap_or(0),
            };
            self.state.select(Some(i));
        }
    }

    // fn unselect(&mut self) {
    //     let offset = self.state.offset();
    //     self.last_selected = self.state.selected();
    //     self.state.select(None);
    //     *self.state.offset_mut() = offset;
    // }
}

// ANCHOR: application
/// Application.
#[derive(Debug, Clone)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// counter
    pub counter: u8,
    pub parameter_stores: StatefulList,
    pub scroll: u16,
    pub filter_ps_list : bool,
    pub ps_filter_data: PsListFilterInput,
    ssm_client: Client
}
// ANCHOR_END: application

pub enum PsMetadata<'a, 'b> {
    Data(&'a ParameterMetadata, String, &'b String),
    None
}

// ANCHOR: application_impl
impl App {
    /// Constructs a new instance of [`App`].
    pub async fn new(args: Args) -> Self {
        Self {
            parameter_stores: StatefulList::new(),
            should_quit: false,
            counter: 0,
            scroll: 0,
            filter_ps_list : false,
            ps_filter_data : PsListFilterInput::new(),
            ssm_client: aws::parameter_store::get_aws_client(args.profile, args.region).await
        }
    }

    pub async fn fetch_ps_data(&mut self){
        match aws::parameter_store::fetch_ps(&self.ssm_client).await {
            Ok((ps_metadata,ps_values,items)) => {
                self.parameter_stores.ps_values = ps_values;
                self.parameter_stores.ps_metadata = ps_metadata;
                self.parameter_stores.items = items.clone();
                self.parameter_stores.display_items = items;
            }
            Err(err) => println!("{:?}",err)
        };
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn get_selected_ps_data(&self) -> PsMetadata{
        let selected_ps_index = match self.parameter_stores.state.selected() {
            Some(metadata) => metadata,
            None => 0
        };

        if self.parameter_stores.display_items.len() > 0 {
            let ps_name = &self.parameter_stores.display_items[selected_ps_index];

            let metadata = match self.parameter_stores.ps_metadata.get(ps_name) {
                Some(ps_metadata) => ps_metadata,
                _ => panic!("")
            };
    
            let value = match self.parameter_stores.ps_values.get(ps_name) {
                Some(value) => value.to_string(),
                None => "".to_string()
            };

            return PsMetadata::Data(metadata,value,ps_name)
        }

       PsMetadata::None

    }

    pub fn get_selected_value(&mut self) -> String {

        let selected_ps_index = match self.parameter_stores.state.selected() {
            Some(metadata) => metadata,
            None => 0
        };
    
        let ps_name = &self.parameter_stores.display_items[selected_ps_index];

        match self.parameter_stores.ps_values.get(ps_name) {
            Some(value) => value.to_string(),
            None => "".to_string()
        }
    }

    pub fn increment_scrol(&mut self){
        self.scroll += 1;
    }

    pub fn decrement_scrol(&mut self){
        if self.scroll == 0 {
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

        fs::remove_file(temp_file_path)?;

        let selected_ps_index = match self.parameter_stores.state.selected() {
            Some(metadata) => metadata,
            None => 0
        };
    
        let ps_name = &self.parameter_stores.display_items[selected_ps_index];

        self.parameter_stores.ps_values.insert((ps_name).to_string(), (&edited_value).to_string());
       
        let _ = aws::parameter_store::edit_ps_value(ps_name, edited_value, &self.ssm_client).await;

        Ok(())
    }
}
// ANCHOR_END: application_impl

// ANCHOR: application_test
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_app_increment_counter() {
//         let mut app = App::default();
//         app.increment_counter();
//         assert_eq!(app.counter, 1);
//     }

//     #[test]
//     fn test_app_decrement_counter() {
//         let mut app = App::default();
//         app.decrement_counter();
//         assert_eq!(app.counter, 0);
//     }
// }
// ANCHOR_END: application_test
