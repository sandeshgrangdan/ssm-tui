use aws::parameter_store::SsmClient;
use aws_sdk_ssm::types::{ParameterTier, ParameterType};
use clap::Parser;
use rand::Rng;

use crate::ui::widgets::state_fullist::StatefulList;
use input::user_input::{
    InputMode::{self, Editing, Normal},
    UserInput,
};
use ratatui::widgets::ScrollbarState;

// Public mod
pub mod aws;
pub mod input;

#[derive(Parser, Debug, Clone)]
pub enum SelectedTab {
    List,
    Details,
}

// ANCHOR_END: action
/// AWS Systems Manager - Parameter Store TUI Client
#[derive(Parser, Debug, Default, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of your AWS profile.
    #[arg(short, long, default_value_t = String::from("None"))]
    pub profile: String,

    /// AWS Region.
    #[arg(short, long, default_value_t = String::from("None"))]
    pub region: String,
}

/// Application.
#[derive(Debug, Clone)]
pub struct App {
    pub selected_tab: SelectedTab,
    pub vertical_scroll: usize,
    pub vertical_scroll_state: ScrollbarState,
    /// should the application exit?
    pub should_quit: bool,
    /// counter
    pub parameter_stores: StatefulList,
    pub scroll: u16,
    pub filter_ps_list: bool,
    pub search: (bool, UserInput),
    pub add_ps: (bool, UserInput),
    pub add_ps_desc: (bool, UserInput),
    pub delete_ps: (bool, UserInput),
    pub ps_tier: ParameterTier,
    pub ps_type: ParameterType,
    ssm_client: SsmClient,
    pub args: Args,
    pub show_help: bool,
    pub input_mode: InputMode,
}

// ANCHOR: application_impl
impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(args: Args) -> Self {
        Self {
            selected_tab: SelectedTab::List,
            vertical_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),
            parameter_stores: StatefulList::new(),
            should_quit: false,
            scroll: 0,
            filter_ps_list: false,
            search: (false, UserInput::new()),
            add_ps: (false, UserInput::new()),
            add_ps_desc: (false, UserInput::new()),
            delete_ps: (false, UserInput::new()),
            ps_tier: ParameterTier::Standard,
            ps_type: ParameterType::String,
            input_mode: InputMode::Normal,
            ssm_client: SsmClient::None,
            args,
            show_help: false, // ssm_client: aws::parameter_store::get_aws_client(args.profile, args.region).await
        }
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_scrol(&mut self) {
        self.scroll += 1;
    }

    pub fn decrement_scrol(&mut self) {
        if self.scroll == 0 {
            self.scroll = 0;
        } else {
            self.scroll -= 1;
        }
    }

    pub fn clear_scrol(&mut self) {
        self.scroll = 0;
    }

    pub fn toggle_search(&mut self) {
        if self.search.0 {
            self.input_mode = Normal;
        } else {
            self.input_mode = Editing;
        }
        self.search.0 = !self.search.0
    }

    fn generate_random_file_name(&self) -> String {
        let mut rng = rand::thread_rng();
        let random_string: String = (0..10)
            .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
            .collect();
        format!("/tmp/{}.txt", random_string) // Creating the file in /tmp directory
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
        assert!(app.search.0);
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

    //     let none_ps_data = SelectedPsMetadata::None;

    //     assert_eq!(ps_data, none_ps_data);
    // }
}
// ANCHOR_END: application_test
