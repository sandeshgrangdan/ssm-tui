use ratatui::{
    prelude::*, 
    style::{Color, Style}, 
    widgets::{Block, BorderType, Borders, Padding, Paragraph, Wrap}
};
use chrono::{DateTime, Utc};

use crate::app::App;

pub fn render_modified_user(app: &mut App, f: &mut Frame, layout: Rect){
    let ps_metadata = &app.get_selected_metadata();

    let modified_user = match &ps_metadata.last_modified_user{
        Some(name) => name,
        None => ""
    };


    let modified_date = match &ps_metadata.last_modified_date {
        Some(date) => date,
        None => panic!("Error for Datetime")
    };


    // Create a DateTime<Utc> from the timestamp and subsecond nanoseconds
    let datetime: DateTime<Utc> = DateTime::from_timestamp(modified_date.secs(), modified_date.subsec_nanos()).unwrap();

    // Format the DateTime to the desired format
    let formatted_date = datetime.format("%a, %d %b %Y %H:%M:%S GMT").to_string();

    let text = vec![
        Line::from(vec!["User 👤: ".green().bold(), modified_user.green().bold()]),
        Line::from(vec!["Date 📅: ".green().bold(), formatted_date.green().bold()]),
    ];

    f.render_widget(
        Paragraph::new(text).wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title("Last modified")
                // .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(1, 1, 1, 1)),
        )
        .style(Style::default().fg(Color::Yellow)),
        // .alignment(Alignment::Left),
        layout,
    );
}