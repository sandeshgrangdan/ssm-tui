use ratatui::{
    layout::Layout,
    text::Text,
    prelude::*, 
    style::{Color, Style}, 
    widgets::{
        Block, 
        BorderType, 
        Borders, 
        Paragraph, 
        Wrap,
        Padding
    }
};
use crate::app::App;

pub fn render_details(app: &mut App, f: &mut Frame, layout: Rect){
    let ps_metadata = app.get_selected_metadata();
    let ps_value: &String  = app.get_selected_value();

    let ps_desc = match &ps_metadata.name {
        Some(my_ps_description) => my_ps_description,
        None => ""
    };

    let text = Text::from(format!("{}",ps_value));

    let paragraph = Paragraph::new(text)
            .block(Block::default().title("Configuration").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .wrap(Wrap { trim: true })
            .scroll((app.scroll,0));

    f.render_widget(
        paragraph
        .block(
            Block::default()
                .title(format!("Value({})",ps_desc))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .padding(Padding::new(1, 1, 1, 1)),
        )
        .style(Style::default().fg(Color::Rgb((83), (178), (226)))),
        // .alignment(Alignment::Center),
        layout,
    );
}
