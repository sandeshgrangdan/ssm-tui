use ratatui::{
    prelude::*, 
    style::Style, 
    widgets::{Block, BorderType, Borders, List}
};
use crate::app::App;

pub fn render_ps_list(app: &mut App, f: &mut Frame, layout: Rect){
    let mut ps_list_with_or_without_search = "All";

    if app.ps_filter_data.input.is_empty() {
        app.parameter_stores.display_items = app.parameter_stores.items.clone();
    }else{
        ps_list_with_or_without_search = &app.ps_filter_data.input;
        app.parameter_stores.display_items = app.parameter_stores.items
            .iter()
            .filter(|name| name.trim().to_lowercase().contains(&app.ps_filter_data.input.trim().to_lowercase()))
            .cloned()
            .collect();
    }

    let list = List::new(app.parameter_stores.display_items.clone())
            .block(
                Block::default()
                .title(format!("PS ({}), {}",ps_list_with_or_without_search,app.parameter_stores.display_items.len()))
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
