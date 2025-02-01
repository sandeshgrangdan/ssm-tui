use aws_sdk_ssm::types::ParameterMetadata;
use ratatui::widgets::ListState;

use crate::app::aws::parameter_store::ParameterStoreMetadata;

#[derive(Debug, Clone)]
pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<String>,
    pub display_items: Vec<String>,
    pub last_selected: Option<usize>,
    pub ps_metadata: Vec<ParameterMetadata>,
    pub ps_values: Vec<ParameterStoreMetadata>,
    pub list_title: String,
}

impl Default for StatefulList {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulList {
    pub fn new() -> Self {
        Self {
            state: ListState::default(),
            items: vec![],
            display_items: vec![],
            last_selected: None,
            ps_metadata: vec![],
            ps_values: vec![],
            list_title: String::from("All"),
        }
    }

    pub fn next(&mut self) {
        if !self.display_items.is_empty() {
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
        if !self.display_items.is_empty() {
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
