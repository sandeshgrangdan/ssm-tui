use ratatui::{
    layout::Layout, 
    prelude::*, 
    style::{Color, Style}, 
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph}
};
use crate::app::App;

pub fn render_user_info(app: &mut App, f: &mut Frame, layout: Rect){
    f.render_widget(
        Paragraph::new(format!(
            "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
        Counter: {:?}
      ",
            app.parameter_store_names.last_selected
        ))
        .block(
            Block::default()
                .title("User Info")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        layout,
    );
}