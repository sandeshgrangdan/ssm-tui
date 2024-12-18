use std::{collections::HashMap, sync::Arc};

use ratatui::widgets::ListState;
use aws_sdk_ssm::types::ParameterMetadata;


#[derive(Debug, Clone)]
pub struct StatefulList {
    pub state: ListState,
    pub items: Arc<Vec<String>>,
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
            items: Arc::new(vec![]),
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
