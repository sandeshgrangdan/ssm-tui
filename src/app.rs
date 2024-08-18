use std::collections::HashMap;
use ratatui::widgets::ListState;
use rand::Rng;
use aws_sdk_ssm::types::ParameterMetadata;
use std::{
    io::{self, Write},
    process::Command,
    fs::{self, File},
};
use ps_list_filter::user_input::{
    PsListFilterInput,
    InputMode::{
        Normal,
        Editing
    }
};
use clap::Parser;
use aws_sdk_ssm::Client;

pub mod ps_list_filter;
mod aws;
// ANCHOR_END: action

/// AWS Systems Manager - Parameter Store TUI Client
#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of your AWS profile.
    #[arg(short, long, default_value_t = String::from("None"))]
    pub profile: String,

    /// AWS Region.
    #[arg(short, long, default_value_t = String::from("None"))]
    pub region: String,
}


#[derive(Debug)]
pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<String>,
    pub display_items: Vec<String>,
    pub last_selected: Option<usize>,
    pub ps_metadata: HashMap<String, ParameterMetadata>,
    pub ps_values : HashMap<String, String>,
    pub list_title: String
}

impl StatefulList {
    pub fn new() -> Self{
        Self { 
            state: ListState::default(), 
            items: vec![],
            display_items: vec![],
            last_selected: None,
            ps_metadata: HashMap::new(),
            ps_values: HashMap::new(),
            list_title: String::from("All")
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
#[derive(Debug)]
enum SsmClient{
    Client(Client),
    None
}
/// Application.
#[derive(Debug)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// counter
    pub parameter_stores: StatefulList,
    pub scroll: u16,
    pub filter_ps_list : bool,
    pub ps_filter_data: PsListFilterInput,
    ssm_client: SsmClient,
    pub args: Args
}
// ANCHOR_END: application
#[derive(Debug, Default)]
pub enum PsMetadata<'a, 'b> {
    Data(&'a ParameterMetadata, String, &'b String),
    #[default]
    None
}



// ANCHOR: application_impl
impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(args: Args) -> Self {
        Self {
            parameter_stores: StatefulList::new(),
            should_quit: false,
            scroll: 0,
            filter_ps_list : false,
            ps_filter_data : PsListFilterInput::new(),
            ssm_client: SsmClient::None,
            args
            // ssm_client: aws::parameter_store::get_aws_client(args.profile, args.region).await
        }
    }

    pub async fn set_ssm_client(&mut self){
        self.ssm_client = SsmClient::
            Client(
                aws::parameter_store::get_aws_client(
                    self.args.profile.clone(), 
                    self.args.region.clone()
                ).await
        )
    }

    pub async fn fetch_ps_data(&mut self){
        match &self.ssm_client {
            SsmClient::Client(client) => {
                match aws::parameter_store::fetch_ps(&client).await {
                    Ok((ps_metadata,ps_values,items)) => {
                        self.parameter_stores.ps_values = ps_values;
                        self.parameter_stores.ps_metadata = ps_metadata;
                        self.parameter_stores.items = items.clone();
                        self.parameter_stores.display_items = items;
                    }
                    Err(err) => println!("{:?}",err)
                };
            }
            _ => {}
        }
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

        let default_value = "".to_string();
        let selected_ps_index = match self.parameter_stores.state.selected() {
            Some(metadata) => metadata,
            None => 0
        };

        if self.parameter_stores.display_items.len() > 0 {
            let ps_name = &self.parameter_stores.display_items[selected_ps_index];

            return match self.parameter_stores.ps_values.get(ps_name) {
                Some(value) => value.to_string(),
                None => default_value
            }
        }
        default_value
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
        let selected_ps_index = match self.parameter_stores.state.selected() {
            Some(metadata) => metadata,
            None => 0
        };

        let ps_name = &self.parameter_stores.display_items[selected_ps_index];

        match &self.ssm_client {
            SsmClient::Client(client) => {
                match aws::parameter_store::get_ps_value(ps_name, client).await {
                    Ok(ps_value) => {
                        let temp_file_path = &self.generate_random_file_name();
        
                        let mut file = File::create(temp_file_path)?;
                        file.write_all(ps_value.as_bytes())?;
                        drop(file);
                
                        Command::new("vim")
                            .arg(temp_file_path) // Specify the file you want to edit with Vim
                            .status()?;
                
                        let edited_value = fs::read_to_string(temp_file_path)?;
                        let edited_value = edited_value.trim().to_string();
                
                        fs::remove_file(temp_file_path)?;
                        
                        if edited_value != ps_value.trim() {
                            self.parameter_stores.ps_values.insert((ps_name).to_string(), (&edited_value).to_string());
                            let _ = aws::parameter_store::edit_ps_value(ps_name, edited_value, client).await;
                            match aws::parameter_store::get_ps_metadata(ps_name, client).await {
                                aws::parameter_store::PsMetadata::Data(data) => {
                                    self.parameter_stores.ps_metadata.insert((ps_name).to_string(), data);
                                }
                                _ => {}
                            }
                        }
                    },
                    Err(_) => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn set_ps_list(&mut self) {
        if self.ps_filter_data.input.is_empty() {
            self.parameter_stores.list_title = "All".to_string();
            self.parameter_stores.display_items = self.parameter_stores.items.clone();
        }else{
            self.parameter_stores.list_title = self.ps_filter_data.input.to_string();

            self.parameter_stores.display_items = self.parameter_stores.items
                .iter()
                .filter(|name| name.trim().to_lowercase().contains(&self.ps_filter_data.input.trim().to_lowercase()))
                .cloned()
                .collect();
        }
    }
}
// ANCHOR_END: application_impl

// ANCHOR: application_test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn toggle_search() {
        let mut app = App::new(Args::parse());
        app.toggle_search();
        assert_eq!(app.filter_ps_list, true);
    }

    #[test]
    fn increment_scrol() {
        let mut app = App::new(Args::parse());
        app.increment_scrol();
        assert_eq!(app.scroll, 1);
    }

    #[test]
    fn decrement_scrol() {
        let mut app = App::new(Args::parse());
        app.decrement_scrol();
        assert_eq!(app.scroll, 0);
    }
    #[test]
    fn clear_scrol() {
        let mut app = App::new(Args::parse());
        app.clear_scrol();
        assert_eq!(app.scroll, 0);
    }

    #[test]
    fn get_selected_value() {
        let mut app = App::new(Args::parse());
        let value = app.get_selected_value();
        assert_eq!(value, "".to_string());
    }

    // #[test]
    // fn get_selected_ps_data() {
    //     let mut app = App::new(Args::parse());
    //     let ps_data = app.get_selected_ps_data();

    //     let none_ps_data = PsMetadata::None;

    //     assert_eq!(ps_data, none_ps_data);
    // }
}
// ANCHOR_END: application_test
