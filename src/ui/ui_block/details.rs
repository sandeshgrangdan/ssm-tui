use ratatui::{
    layout::Layout,
    text::Text,
    prelude::*, 
    style::{Color, Style}, 
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph, Wrap}
};
use crate::app::App;

pub fn render_details(app: &mut App, f: &mut Frame, layout: Rect){
    let ps_metadata = app.get_selected_metadata();
    let ps_value: &String  = app.get_selected_value();

    let ps_desc = match &ps_metadata.name {
        Some(my_ps_description) => my_ps_description,
        None => ""
    };

    let text = Text::from(format!("\n{}",ps_value));

    let paragraph = Paragraph::new(text)
            .block(Block::default().title("Configuration").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true })
            .scroll((app.scroll,0));

    f.render_widget(
        paragraph
        .block(
            Block::default()
                .title(ps_desc)
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow)),
        // .alignment(Alignment::Center),
        layout,
    );
}
