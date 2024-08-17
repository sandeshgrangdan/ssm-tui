use ratatui::{
    prelude::*, 
    style::Style, 
    widgets::{Block, BorderType, Borders, List}
};
use crate::app::App;

pub fn render_ps_list(app: &mut App, f: &mut Frame, layout: Rect){
    let list = List::new(app.parameter_stores.display_items.clone())
            .block(
                Block::default()
                .title(format!("PS ({}), {}",app.parameter_stores.list_title,app.parameter_stores.display_items.len()))
                // .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
            )
            .highlight_style(
                Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::REVERSED)
            )
            .highlight_symbol(">")
            .repeat_highlight_symbol(true);
        

    f.render_stateful_widget(
        list,
        layout,
        &mut app.parameter_stores.state
    );
}
