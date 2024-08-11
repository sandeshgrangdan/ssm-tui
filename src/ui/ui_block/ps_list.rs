use ratatui::{
    layout::Layout, 
    prelude::*, 
    style::{Color, Style}, 
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph}
};
use crate::app::App;

pub fn render_ps_list(app: &mut App, f: &mut Frame, layout: Rect){
    let mut ps_list_with_or_without_search = &app.ps_filter_data.input;

    let mut with_or_witout_filtered_list = vec![];

    if app.ps_filter_data.input.is_empty() {
        ps_list_with_or_without_search = &app.ps_filter_data.default_messages;
        with_or_witout_filtered_list = app.parameter_store_names.items.clone();
    }else{
        ps_list_with_or_without_search = &app.ps_filter_data.input;
        for ps_name in &app.parameter_store_names.items {
            if ps_name.to_lowercase().contains(&app.ps_filter_data.input.to_lowercase()) {
                with_or_witout_filtered_list.push(ps_name.clone());
            }
        }
    }

    let list = List::new(with_or_witout_filtered_list)
            .block(
                Block::default()
                .title(format!("PS ({})",ps_list_with_or_without_search))
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
        &mut app.parameter_store_names.state
    );
}
