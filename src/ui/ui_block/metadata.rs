use ratatui::{
    layout::Layout, 
    prelude::*, 
    style::{Color, Style}, 
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph}
};
use crate::app::App;

pub fn render_metadata(app: &mut App, f: &mut Frame, layout: Rect){
    let selected_ps_index = match app.parameter_store_names.state.selected() {
        Some(metadata) => metadata,
        None => 0
    };

    let ps_metadata = &app.parameter_store_names.ps_metadata[selected_ps_index];

    let ps_name = match &ps_metadata.name{
        Some(name) => name,
        None => ""
    };

    f.render_widget(
        Paragraph::new(format!(
            "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
        Counter: {}
      ",
      ps_name
        ))
        .block(
            Block::default()
                .title("Metadata")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        layout,
    );
}

